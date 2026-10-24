mod support;
use pulldown_cmark::{Options, Parser, html};
use regex::Regex;
use std::{collections::BTreeSet, fs, process::Command};
use support::{REMOVED, ROUTES, Workerd, get};

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
fn element<'a>(html: &'a str, tag: &str) -> &'a str {
    html.split_once(&format!("<{tag}>"))
        .unwrap()
        .1
        .split_once(&format!("</{tag}>"))
        .unwrap()
        .0
}
fn complete_main(page: &str, source: &str) {
    let mut rendered = String::new();
    html::push_html(
        &mut rendered,
        Parser::new_ext(
            source,
            Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
        ),
    );

    // Anchor ids are the only addition to plain CommonMark output.
    let ids = Regex::new(r#"<(li|p|h[2-6]) id="[^"]+">"#).unwrap();
    assert_eq!(ids.replace_all(element(page, "main"), "<$1>"), rendered);
}
fn slug(text: &str) -> String {
    let slug = text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join("-");
    if !slug.is_empty() && slug.bytes().all(|b| b.is_ascii_digit()) {
        format!("section-{slug}")
    } else {
        slug
    }
}
// Every numbered article, bold-led section and subheading has the link target guidance cites.
fn escape(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
fn anchors(page: &str, source: &str, numbered: bool) {
    let main = element(page, "main");
    // `/#n` must open article n itself: the id sits on the item that starts with its title.
    let articles = Regex::new(r"(?m)^(\d+)\. \*\*([^*]+)\*\*").unwrap();
    let leads = Regex::new(r"(?m)^\*\*([^*]+\.)\*\*").unwrap();
    let headings = Regex::new(r"(?m)^#{2,6} (.+)$").unwrap();
    let expected: Vec<String> = articles
        .captures_iter(source)
        .filter(|_| numbered)
        .map(|c| {
            format!(
                "<li id=\"{}\">\n<p><strong>{}</strong>",
                &c[1],
                escape(&c[2])
            )
        })
        .chain(
            leads
                .captures_iter(source)
                .map(|c| format!("<p id=\"{}\">", slug(&c[1]))),
        )
        .chain(
            headings
                .captures_iter(source)
                .filter(|c| !slug(&c[1]).is_empty())
                .map(|c| {
                    let level = c[0].len() - c[0].trim_start_matches('#').len();
                    format!("<h{level} id=\"{}\">", slug(&c[1]))
                }),
        )
        .collect();
    assert!(!expected.is_empty());
    for anchor in &expected {
        // Tight article lists have no <p> wrapper; loose ones do.
        let tight = anchor.replace("\n<p><strong>", "<strong>");
        assert!(
            main.contains(anchor.as_str()) || main.contains(tight.as_str()),
            "missing anchor {anchor}"
        );
    }
    let found = Regex::new(r#" id="[^"]+""#)
        .unwrap()
        .find_iter(main)
        .count();
    assert_eq!(found, expected.len(), "unexpected or duplicate anchors");
}
fn privacy(text: &str) {
    let pattern = r"(?i)\br\d{2}(?:group)?\b|/home/|phaedrus@";
    assert!(
        !Regex::new(pattern).unwrap().is_match(text),
        "private identifier in publication"
    );
}

#[test]
fn reference_sources_navigation_and_complete_prose() {
    let revision = String::from_utf8(
        Command::new("git")
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap();

    for (name, title) in [
        ("index", "Constitution"),
        ("way-of-working", "Way of working"),
    ] {
        let source = read(&format!("{name}.md"));
        let html = read(&format!("dist/{name}.html"));
        assert_eq!(
            fs::read(format!("dist/{name}.md")).unwrap(),
            source.as_bytes()
        );
        privacy(&source);
        complete_main(&html, &source);
        anchors(&html, &source, name == "index");
        assert_eq!(element(&html, "h1"), title);
        assert_eq!(element(&html, "title"), "Foundations — Misty Step");
        assert!(
            html.contains("color-scheme: light dark")
                && html.contains("prefers-color-scheme: dark")
        );
        assert!(!html.contains("<script") && !html.contains("audit-checklist"));
        for link in [
            "/".to_owned(),
            "/way-of-working".to_owned(),
            format!("/{name}.md"),
            format!("https://github.com/misty-step/foundations/commits/master/{name}.md"),
            format!(
                "https://github.com/misty-step/foundations/commit/{}",
                revision.trim()
            ),
        ] {
            assert!(
                html.contains(&format!("href=\"{link}\"")),
                "{name}: missing {link}"
            );
        }
        assert!(!html.contains("/research"));
    }
}

#[test]
fn published_routes_are_exactly_two_documents_and_markdown() {
    let expected: BTreeSet<_> = ROUTES
        .iter()
        .map(|(_, file, _)| (*file).to_owned())
        .chain(["_headers".to_owned()])
        .collect();
    let actual: BTreeSet<_> = fs::read_dir("dist")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            assert!(
                entry.file_type().unwrap().is_file(),
                "extra directory or symlink: {:?}",
                entry.path()
            );
            entry.file_name().into_string().unwrap()
        })
        .collect();
    assert_eq!(actual, expected, "extra or missing publication route");
}

#[test]
fn artifact_serves_over_workerd_with_rejection_and_recovery() {
    let server = Workerd::start();
    for (route, file, content_type) in ROUTES {
        let (status, headers, body) = get(&format!("{}{route}", server.url));
        assert_eq!(status, 200, "{route}");
        assert!(
            headers.contains(&format!("content-type: {content_type}; charset=utf-8")),
            "{headers}"
        );
        assert!(
            headers.contains("x-content-type-options: nosniff")
                && headers.contains("referrer-policy: no-referrer")
        );
        assert_eq!(body, fs::read(format!("dist/{file}")).unwrap(), "{route}");
    }
    for route in REMOVED.into_iter().chain(["/not-a-reference", "/_headers"]) {
        let (status, headers, _) = get(&format!("{}{route}", server.url));
        assert_eq!(status, 404, "{route}");
        assert!(!headers.contains("location:"), "{route}: must not redirect");
        let (status, _, body) = get(&format!("{}/", server.url));
        assert_eq!(status, 200);
        assert_eq!(body, fs::read("dist/index.html").unwrap());
    }
    let output = Command::new("bash")
        .args(["scripts/verify-served.sh", &server.url])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn renderer_has_exact_markdown_semantics_and_rejects_missing_input() {
    let dir = tempfile::tempdir().unwrap();
    for file in ["index.md", "way-of-working.md"] {
        let target = dir.path().join(file);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::copy(file, target).unwrap();
    }
    assert!(
        Command::new("git")
            .args(["init", "--quiet"])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success()
    );
    assert!(
        Command::new("git")
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "commit",
                "--quiet",
                "--allow-empty",
                "-m",
                "fixture"
            ])
            .current_dir(dir.path())
            .status()
            .unwrap()
            .success()
    );
    fs::write(dir.path().join("index.md"), "# Exact\n\n**bold** and `code` & <test>\n\n- one\n- two\n\n[Source](https://example.org/?a=1&b=2)\n\n## Deploy `CI` keys\n\n**Milei check.** Cut.\n\n**Prove.** Walk.\n\n**Prove.** Again.\n\n## 1\n\n> **Quoted.** Aside.\n\n3. Third\n4. Fourth\n   1. Nested\n").unwrap();
    let run = || {
        Command::new(env!("CARGO_BIN_EXE_foundations"))
            .current_dir(dir.path())
            .output()
            .unwrap()
    };
    assert!(run().status.success());
    let html = fs::read_to_string(dir.path().join("dist/index.html")).unwrap();
    assert!(html.contains("<main><h1>Exact</h1>\n<p><strong>bold</strong> and <code>code</code> &amp; <test></p>\n<ul>\n<li>one</li>\n<li>two</li>\n</ul>\n<p><a href=\"https://example.org/?a=1&amp;b=2\">Source</a></p>\n<h2 id=\"deploy-ci-keys\">Deploy <code>CI</code> keys</h2>\n<p id=\"milei-check\"><strong>Milei check.</strong> Cut.</p>\n<p id=\"prove\"><strong>Prove.</strong> Walk.</p>\n<p id=\"prove-2\"><strong>Prove.</strong> Again.</p>\n<h2 id=\"section-1\">1</h2>\n<blockquote>\n<p><strong>Quoted.</strong> Aside.</p>\n</blockquote>\n<ol start=\"3\">\n<li id=\"3\">Third</li>\n<li id=\"4\">Fourth\n<ol>\n<li>Nested</li>\n</ol>\n</li>\n</ol>\n</main>"));

    fs::write(dir.path().join("dist/obsolete.html"), "stale").unwrap();
    assert!(run().status.success());
    assert!(!dir.path().join("dist/obsolete.html").exists());
    fs::remove_file(dir.path().join("way-of-working.md")).unwrap();
    assert!(!run().status.success());
    // Numbered lists outside the Constitution are ordinary lists: no `/#n`, no refusal.
    let recovered = "# Recovered\n\n1. One\n\nBreak.\n\n1. Again\n";
    fs::write(dir.path().join("way-of-working.md"), recovered).unwrap();
    assert!(run().status.success());
    assert_eq!(
        fs::read(dir.path().join("dist/way-of-working.md")).unwrap(),
        recovered.as_bytes()
    );
    assert!(
        !fs::read_to_string(dir.path().join("dist/way-of-working.html"))
            .unwrap()
            .contains("<li id=")
    );
    // A second top-level numbered list would steal `/#1` from article 1: refuse to build.
    fs::write(
        dir.path().join("index.md"),
        "1. Article\n\nBreak.\n\n1. Impostor\n",
    )
    .unwrap();
    let refused = run();
    assert!(!refused.status.success());
    // The refused render left the previous artifact whole, not a partial dist/.
    assert!(dir.path().join("dist/way-of-working.html").exists());
    assert!(String::from_utf8_lossy(&refused.stderr).contains("/#1 is claimed twice"));
}
