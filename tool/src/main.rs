use chrono::prelude::*;
use dateparser::parse;
use regex::Regex;
use std::path::Path;
use std::path::PathBuf;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[derive(Debug, serde::Deserialize)]
struct Frontmatter {
    title: Option<String>,
    updated: Option<DateTime<Utc>>,
    created: Option<DateTime<Utc>>,
}

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    process_dir(Path::new("./from_notion")).unwrap();
}

fn process_dir(path: &Path) -> anyhow::Result<()> {
    anyhow::ensure!(path.is_dir(), "{path:?} is not a directory.");

    let pattern = format!("{}/**/*.md", path.display());
    println!("Searching for {pattern}");

    for file in glob::glob(&pattern)?.flatten() {
        process_file(&file)?;
    }

    Ok(())
}

fn process_file(path: &Path) -> anyhow::Result<()> {
    let md_text = std::fs::read_to_string(path).expect("must read");
    let (matter, _body) = markdown_frontmatter::parse::<Frontmatter>(&md_text).unwrap();

    // tracing::info!("{matter:#?}");
    // generate new path.
    let mut new_name = PathBuf::new();

    // Search for 'Published On' else use created field from metadata.
    let regex_published_on = Regex::new(r"Published On:\s+(?P<date>.+?)\n").expect("must compile");
    let mut published_date = if let Some(caps) = regex_published_on.captures(&md_text) {
        tracing::info!("==> Found published date {:?}", caps);
        parse(&caps["date"]).ok()
    } else {
        matter.created
    };

    if let Some(date) = published_date {
        new_name.push(date.format("%Y").to_string());

        let title = matter.title.unwrap_or("NA".to_string());
        let filename = title.replace(" ", "_");
        new_name.push(format!("{}-{}.md", date.format("%Y-%m-%d"), filename,));
    }

    let new_path = Path::new("../content/posts").join(new_name);

    anyhow::ensure!(
        new_path.parent().and_then(|x| x.parent()).unwrap().is_dir(),
        "{:?} is not a directory",
        new_path.parent().and_then(|x| x.parent())
    );

    // create parent of this new_path is doesn't exists.
    if let Some(parent) = new_path.parent()
        && !parent.is_dir()
    {
        std::fs::create_dir(parent)?;
    }

    // copy to new location.
    std::fs::write(&new_path, md_text)?;
    println!("Wrote {} to {}", path.display(), new_path.display());

    Ok(())
}
