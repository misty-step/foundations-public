use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    process::Command,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::Duration,
};

#[path = "support/routes.rs"]
mod routes;
use routes::{REMOVED, ROUTES};

#[test]
fn readback_requires_one_complete_matching_attempt_and_bounds_failures() {
    // Exercise the real curl and byte comparisons. Only retry delays are recorded, not slept.
    let script = fs::canonicalize("scripts/verify-served.sh").unwrap();
    let files: Vec<Vec<u8>> = ROUTES
        .iter()
        .map(|(_, file, _)| fs::read(format!("dist/{file}")).unwrap())
        .collect();
    for scenario in 0..10 {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("dist")).unwrap();
        fs::create_dir(dir.path().join("bin")).unwrap();
        for ((_, file, _), bytes) in ROUTES.iter().zip(&files) {
            fs::write(dir.path().join("dist").join(file), bytes).unwrap();
        }
        let sleep = dir.path().join("bin/sleep");
        fs::write(&sleep, "#!/bin/sh\nprintf '%s\\n' \"$1\" >> sleeps.txt\n").unwrap();
        fs::set_permissions(&sleep, fs::Permissions::from_mode(0o755)).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let done = stop.clone();
        let bodies = files.clone();
        let server = thread::spawn(move || {
            let mut paths = Vec::new();
            let mut attempt = 0;
            while !done.load(Ordering::Relaxed) {
                let (mut stream, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }
                    Err(error) => panic!("{error}"),
                };
                stream
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut reader = BufReader::new(stream.try_clone().unwrap());
                let mut request = String::new();
                reader.read_line(&mut request).unwrap();
                let route = request.split_whitespace().nth(1).unwrap().to_owned();
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).unwrap() == 0 || line == "\r\n" {
                        break;
                    }
                }
                if route == ROUTES[0].0 {
                    attempt += 1;
                }
                let index = ROUTES.iter().position(|(path, _, _)| *path == route);
                paths.push(route);
                if scenario == 7 {
                    continue;
                } // real connection close without an HTTP response
                let stale = match scenario {
                    1 => [Some(0), Some(1), Some(2)]
                        .get(attempt - 1)
                        .copied()
                        .flatten(),
                    2..=5 => Some(scenario - 2),
                    _ => None,
                };
                let mut body = index.map(|index| bodies[index].clone()).unwrap_or_default();
                if stale.is_some() && stale == index {
                    body.extend_from_slice(b"stale trailing bytes\n");
                }
                let status = if scenario == 6 {
                    "503 Service Unavailable"
                } else if index.is_none() && scenario == 9 {
                    "301 Moved Permanently"
                } else if index.is_none() && scenario != 8 {
                    assert!(REMOVED.contains(&paths.last().unwrap().as_str()));
                    "404 Not Found"
                } else {
                    "200 OK"
                };
                write!(
                    stream,
                    "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                    body.len()
                )
                .unwrap();
                stream.write_all(&body).unwrap();
            }
            (attempt, paths)
        });
        let output = Command::new("bash")
            .arg(&script)
            .arg(url)
            .current_dir(dir.path())
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    dir.path().join("bin").display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .output()
            .unwrap();
        stop.store(true, Ordering::Relaxed);
        let (attempts, paths) = server.join().unwrap();
        let expected_attempts = match scenario {
            0 => 1,
            1 => 4,
            _ => 9,
        };
        assert_eq!(attempts, expected_attempts, "scenario {scenario}");
        assert_eq!(
            output.status.success(),
            scenario < 2,
            "scenario {scenario}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let expected_paths: Vec<_> = (0..expected_attempts)
            .flat_map(|attempt| {
                let count = match scenario {
                    0 => ROUTES.len() + REMOVED.len(),
                    1 => [1, 2, 3, ROUTES.len() + REMOVED.len()][attempt],
                    2..=5 => scenario - 1,
                    8..=9 => ROUTES.len() + 1,
                    _ => 1,
                };
                ROUTES
                    .iter()
                    .map(|(path, _, _)| *path)
                    .chain(REMOVED)
                    .take(count)
                    .map(str::to_owned)
            })
            .collect();
        assert_eq!(paths, expected_paths, "scenario {scenario}");
        let sleeps = fs::read_to_string(dir.path().join("sleeps.txt")).unwrap_or_default();
        assert_eq!(sleeps, "10\n".repeat(expected_attempts - 1));
        if scenario < 2 {
            assert!(
                String::from_utf8_lossy(&output.stdout).contains("match the verified artifact")
            );
            for ((_, file, _), bytes) in ROUTES.iter().zip(&files) {
                assert_eq!(
                    fs::read(
                        dir.path()
                            .join(format!("served-{}", file.replace('/', "-")))
                    )
                    .unwrap(),
                    *bytes
                );
            }
        } else {
            assert!(String::from_utf8_lossy(&output.stderr).contains("did not converge"));
        }
    }
}
