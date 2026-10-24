use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};
use std::{collections::HashSet, error::Error, fs, process::Command};

const STYLE: &str = include_str!("style.html");
const NAV: &str = "<nav aria-label=\"Documents\"><a href=\"/\">Constitution</a> · <a href=\"/way-of-working\">Way of working</a></nav>";

fn slug(text: &str) -> String {
    let slug = text
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>()
        .join("-");
    // Bare numbers are reserved for articles.
    if !slug.is_empty() && slug.bytes().all(|b| b.is_ascii_digit()) {
        format!("section-{slug}")
    } else {
        slug
    }
}

fn text_until(events: &[Event], end: TagEnd) -> String {
    events
        .iter()
        .take_while(|event| **event != Event::End(end))
        .filter_map(|event| match event {
            Event::Text(text) | Event::Code(text) => Some(text.as_ref()),
            _ => None,
        })
        .collect()
}

// Cited links need stable targets: `/#12` for article 12, `/way-of-working#prove`
// for a bold-led section, and a slug for every heading below the title. Only
// top-level blocks get ids. Numbered ids belong to the Constitution's articles
// alone: a second top-level numbered list there fails the build rather than
// letting `/#n` open something other than article n.
fn markdown(source: &str, articles: bool) -> Result<String, String> {
    let mut events: Vec<Event> = Parser::new_ext(
        source,
        Options::ENABLE_TABLES | Options::ENABLE_STRIKETHROUGH,
    )
    .collect();
    let mut used = HashSet::new();
    let mut unique = |id: String| {
        let mut candidate = id.clone();
        let mut n = 2;
        while !used.insert(candidate.clone()) {
            candidate = format!("{id}-{n}");
            n += 1;
        }
        candidate
    };
    let mut numbered = HashSet::new();
    let mut lists: Vec<Option<u64>> = Vec::new();
    let mut quotes = 0;
    for i in 0..events.len() {
        let replacement = match &events[i] {
            Event::Start(Tag::BlockQuote(_)) => {
                quotes += 1;
                None
            }
            Event::End(TagEnd::BlockQuote(_)) => {
                quotes -= 1;
                None
            }
            Event::Start(Tag::List(start)) => {
                lists.push(*start);
                None
            }
            Event::End(TagEnd::List(_)) => {
                lists.pop();
                None
            }
            Event::Start(Tag::Item) if articles && lists.len() == 1 && quotes == 0 => {
                match lists.last_mut() {
                    Some(Some(number)) => {
                        if !numbered.insert(*number) {
                            return Err(format!(
                                "/#{number} is claimed twice; article numbers must come from one top-level numbered list"
                            ));
                        }
                        let id = unique(number.to_string());
                        *number += 1;
                        Some(Event::Html(CowStr::from(format!("<li id=\"{id}\">"))))
                    }
                    _ => None,
                }
            }
            Event::Start(Tag::Paragraph)
                if lists.is_empty()
                    && quotes == 0
                    && events.get(i + 1) == Some(&Event::Start(Tag::Strong)) =>
            {
                let lead = text_until(&events[i + 2..], TagEnd::Strong);
                let id = slug(&lead);
                (lead.ends_with('.') && !id.is_empty())
                    .then(|| Event::Html(CowStr::from(format!("<p id=\"{}\">", unique(id)))))
            }
            Event::Start(Tag::Heading {
                level,
                id: None,
                classes,
                attrs,
            }) if *level != HeadingLevel::H1 && quotes == 0 && lists.is_empty() => {
                let id = slug(&text_until(&events[i + 1..], TagEnd::Heading(*level)));
                (!id.is_empty()).then(|| {
                    Event::Start(Tag::Heading {
                        level: *level,
                        id: Some(CowStr::from(unique(id))),
                        classes: classes.clone(),
                        attrs: attrs.clone(),
                    })
                })
            }
            _ => None,
        };
        if let Some(event) = replacement {
            events[i] = event;
        }
    }
    let mut output = String::new();
    html::push_html(&mut output, events.into_iter());
    Ok(output)
}

fn page(title: &str, description: &str, head: &str, body: &str) -> String {
    format!(
        "<!doctype html>\n<html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\n<title>{title}</title>\n{description}{STYLE}\n{head}{body}\n</html>\n"
    )
}

fn build() -> Result<(), Box<dyn Error>> {
    let git = Command::new("git").args(["rev-parse", "HEAD"]).output()?;
    if !git.status.success() {
        return Err("cannot identify source revision".into());
    }
    let revision = String::from_utf8(git.stdout)?.trim().to_owned();
    // Read all inputs first: a missing source must fail, never reuse stale output.
    let documents = ["index", "way-of-working"]
        .map(|name| fs::read_to_string(format!("{name}.md")))
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    // Render everything before touching dist/: a render error must leave no partial artifact.
    let rendered = ["index", "way-of-working"]
        .into_iter()
        .zip(&documents)
        .map(|(name, source)| {
            markdown(source, name == "index").map_err(|error| format!("{name}.md: {error}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    if std::path::Path::new("dist").exists() {
        fs::remove_dir_all("dist")?;
    }
    fs::create_dir("dist")?;
    for ((name, source), main) in ["index", "way-of-working"]
        .into_iter()
        .zip(documents)
        .zip(rendered)
    {
        let body = format!(
            "{NAV}\n<main>{main}</main>\n<footer>Source revision: <a href=\"https://github.com/misty-step/foundations/commit/{revision}\">{revision}</a> · <a href=\"/{name}.md\">Markdown</a> · <a href=\"https://github.com/misty-step/foundations/commits/master/{name}.md\">History</a></footer>"
        );
        fs::write(
            format!("dist/{name}.html"),
            page(
                "Foundations — Misty Step",
                "<meta name=\"description\" content=\"The living engineering reference.\">\n",
                "",
                &body,
            ),
        )?;
        fs::write(format!("dist/{name}.md"), source.as_bytes())?;
        println!("Built {revision} {name}: {} source bytes", source.len());
    }

    fs::write("dist/_headers", include_bytes!("headers"))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    build()
}
