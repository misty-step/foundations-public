#!/usr/bin/env python3
"""Read Foundations in real Chromium with page JavaScript disabled, as a reader does.

Usage: browse.py --base URL --out DIR
Prints one JSON object per check on stdout; screenshots go to DIR/screens/.
Exits 0 when every check passed, 3 when it finished with failed checks.
Run it through `verify-foundations browse`, which supplies a pinned Playwright.
"""

import argparse
import json
import pathlib
import sys

from playwright.sync_api import sync_playwright

PAGES = [("/", "constitution", "Constitution"), ("/way-of-working", "way-of-working", "Way of working")]
VIEWPORTS = {"phone": {"width": 390, "height": 844}, "desktop": {"width": 1440, "height": 900}}
SCHEMES = ["light", "dark"]
TITLE = "Foundations — Misty Step"
failed = 0


def report(check, target, ok, **detail):
    global failed
    failed += not ok
    print(json.dumps({"check": check, "target": target, "ok": bool(ok), **detail}), flush=True)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", required=True)
    parser.add_argument("--out", required=True)
    args = parser.parse_args()
    base = args.base.rstrip("/")
    screens = pathlib.Path(args.out) / "screens"
    screens.mkdir(parents=True, exist_ok=True)

    with sync_playwright() as p:
        browser = p.chromium.launch()

        def context(viewport="desktop", scheme="light"):
            return browser.new_context(
                java_script_enabled=False, viewport=VIEWPORTS[viewport], color_scheme=scheme
            )

        # Read: both documents at phone and desktop width, in light and dark.
        backgrounds = {}
        for path, slug, heading in PAGES:
            for viewport in VIEWPORTS:
                for scheme in SCHEMES:
                    ctx = context(viewport, scheme)
                    page = ctx.new_page()
                    response = page.goto(base + path)
                    h1 = page.locator("main h1").inner_text()
                    nav = page.get_by_role("navigation", name="Documents")
                    links = all(
                        nav.get_by_role("link", name=name, exact=True).is_visible()
                        for name in ("Constitution", "Way of working")
                    )
                    width, client, background = page.evaluate(
                        "[document.documentElement.scrollWidth, document.documentElement.clientWidth,"
                        " getComputedStyle(document.documentElement).backgroundColor]"
                    )
                    backgrounds[(slug, viewport, scheme)] = background
                    shot = screens / f"{slug}-{viewport}-{scheme}.png"
                    page.screenshot(path=shot)
                    report(
                        "read",
                        f"{path} {viewport} {scheme}",
                        response.status == 200
                        and page.title() == TITLE
                        and h1 == heading
                        and links
                        and width <= client,
                        status=response.status,
                        h1=h1,
                        nav_links=links,
                        scroll_width=width,
                        client_width=client,
                        background=background,
                        screenshot=shot.name,
                    )
                    ctx.close()
            for viewport in VIEWPORTS:
                light, dark = (backgrounds[(slug, viewport, s)] for s in SCHEMES)
                report("color-scheme", f"{path} {viewport}", light != dark, light=light, dark=dark)

        # Navigate: document links and the footer's Markdown link, by clicking.
        ctx = context()
        page = ctx.new_page()
        page.goto(base + "/")
        page.get_by_role("navigation", name="Documents").get_by_role("link", name="Way of working").click()
        page.wait_for_load_state()
        report("navigate", "Constitution -> Way of working", page.url == base + "/way-of-working"
               and page.locator("main h1").inner_text() == "Way of working", url=page.url)
        page.get_by_role("navigation", name="Documents").get_by_role("link", name="Constitution").click()
        page.wait_for_load_state()
        report("navigate", "Way of working -> Constitution", page.url == base + "/"
               and page.locator("main h1").inner_text() == "Constitution", url=page.url)
        footer = page.locator("footer")
        revision = footer.get_by_role("link").first.inner_text()
        page.screenshot(path=screens / "constitution-footer.png", full_page=True)
        with page.expect_response(lambda r: r.url.endswith("/index.md")) as markdown:
            footer.get_by_role("link", name="Markdown").click()
        body = markdown.value.text()
        report("navigate", "footer Markdown link", markdown.value.status == 200
               and body.startswith("# Constitution\n"), status=markdown.value.status,
               content_type=markdown.value.headers.get("content-type"), revision=revision)
        ctx.close()

        # Cite: every anchor on both pages scrolls its target into view on a phone.
        for path, slug, _ in PAGES:
            ctx = context("phone")
            page = ctx.new_page()
            page.goto(base + path)
            ids = page.evaluate("[...document.querySelectorAll('main [id]')].map(e => e.id)")
            report("anchors-present", path, bool(ids), ids=ids)
            height = VIEWPORTS["phone"]["height"]
            for anchor in ids:
                target = f"document.getElementById({json.dumps(anchor)}).getBoundingClientRect().top"
                # Start each anchor from the unscrolled page, then follow the link.
                page.goto(base + path)
                start = page.evaluate(target)
                page.goto(f"{base}{path}#{anchor}")
                top = page.evaluate(target)
                scrolled = page.evaluate("window.scrollY")
                at_end = page.evaluate(
                    "window.scrollY + innerHeight >= document.documentElement.scrollHeight - 2")
                # The link brings the target to the top of the screen (sub-pixel offsets aside),
                # unless the page has run out of room to scroll; a target that started below
                # the first screen got there only because the link scrolled the page.
                ok = (abs(top) <= 2 or (at_end and 0 <= top < height)) and (start < height or scrolled > 0)
                if anchor in ("12", "prove"):
                    page.screenshot(path=screens / f"anchor-{slug}-{anchor}.png")
                report("anchor", f"{path}#{anchor}", ok, start_top=round(start, 1), top=round(top, 1),
                       scroll_y=scrolled, at_end=at_end)
            ctx.close()

        # Recover: an unknown path is a native 404 with no redirect, then home loads.
        ctx = context("phone")
        page = ctx.new_page()
        response = page.goto(base + "/not-a-reference")
        page.screenshot(path=screens / "not-found.png")
        report("not-found", "/not-a-reference", response.status == 404
               and response.request.redirected_from is None, status=response.status)
        response = page.goto(base + "/")
        report("recover", "/ after 404", response.status == 200
               and page.locator("main h1").inner_text() == "Constitution", status=response.status)
        ctx.close()
        browser.close()
    return 3 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
