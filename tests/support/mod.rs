use std::{
    fs,
    net::TcpListener,
    os::unix::process::CommandExt,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
use tempfile::TempDir;

mod routes;
pub use routes::{REMOVED, ROUTES};

pub fn get(url: &str) -> (u16, String, Vec<u8>) {
    let output = Command::new("curl")
        .args([
            "--silent",
            "--show-error",
            "--max-time",
            "5",
            "--include",
            url,
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let split = output
        .stdout
        .windows(4)
        .position(|x| x == b"\r\n\r\n")
        .unwrap();
    let headers = String::from_utf8(output.stdout[..split].to_vec()).unwrap();
    let status = headers.split_whitespace().nth(1).unwrap().parse().unwrap();
    (
        status,
        headers.to_lowercase(),
        output.stdout[split + 4..].to_vec(),
    )
}

pub struct Workerd {
    pub url: String,
    child: Child,
    _logs: TempDir,
}

impl Workerd {
    pub fn start() -> Self {
        let socket = TcpListener::bind("127.0.0.1:0").unwrap();
        let port = socket.local_addr().unwrap().port();
        drop(socket);
        let logs = tempfile::tempdir().unwrap();
        let logfile = logs.path().join("workerd.log");
        let log = fs::File::create(&logfile).unwrap();
        let child = Command::new("node")
            .args([
                "node_modules/wrangler/bin/wrangler.js",
                "dev",
                "--local",
                "--ip",
                "127.0.0.1",
                "--port",
                &port.to_string(),
                "--inspector-port",
                "0",
                "--show-interactive-dev-session",
                "false",
            ])
            .env("WRANGLER_SEND_METRICS", "false")
            .env("CI", "true")
            .env_remove("CLOUDFLARE_API_TOKEN")
            .env_remove("CLOUDFLARE_ACCOUNT_ID")
            .stdin(Stdio::null())
            .stdout(log.try_clone().unwrap())
            .stderr(log)
            .process_group(0)
            .spawn()
            .unwrap();
        let mut server = Self {
            url: format!("http://127.0.0.1:{port}"),
            child,
            _logs: logs,
        };
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            assert!(
                server.child.try_wait().unwrap().is_none(),
                "{}",
                fs::read_to_string(&logfile).unwrap()
            );
            if fs::read_to_string(&logfile).unwrap().contains("Ready on") {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "{}",
                fs::read_to_string(&logfile).unwrap()
            );
            thread::sleep(Duration::from_millis(50));
        }
        server
    }
}

impl Drop for Workerd {
    fn drop(&mut self) {
        // Wrangler owns a workerd child; terminate only this test's process group.
        let _ = Command::new("kill")
            .args(["-TERM", "--", &format!("-{}", self.child.id())])
            .status();
        let _ = self.child.wait();
    }
}
