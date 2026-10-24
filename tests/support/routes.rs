pub const ROUTES: [(&str, &str, &str); 4] = [
    ("/index.md", "index.md", "text/markdown"),
    ("/way-of-working.md", "way-of-working.md", "text/markdown"),
    ("/", "index.html", "text/html"),
    ("/way-of-working", "way-of-working.html", "text/html"),
];

pub const REMOVED: [&str; 8] = [
    "/research",
    "/research/ci",
    "/research/ci/",
    "/research/ci.html",
    "/research/ci.md",
    "/research/ci-evidence.json",
    "/revision.txt",
    "/404.html",
];
