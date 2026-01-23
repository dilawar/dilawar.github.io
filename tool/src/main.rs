use chrono::prelude::*;
use std::{fmt::Formatter, path::Path, string::ParseError};
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

    process_dir(Path::new("../content/posts/from_notion")).unwrap();
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
    tracing::info!("Processing {path:?}");
    let md_text = std::fs::read_to_string(path).expect("must read");
    let (matter, _body) = markdown_frontmatter::parse::<Frontmatter>(&md_text).unwrap();
    tracing::info!("{matter:#?}");

    // generate new path.

    Ok(())
}
