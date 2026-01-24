use chrono::prelude::*;
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
    let md_text = std::fs::read_to_string(path).expect("must read");
    let (matter, _body) = markdown_frontmatter::parse::<Frontmatter>(&md_text).unwrap();

    // tracing::info!("{matter:#?}");
    // generate new path.
    let mut new_name = PathBuf::new();
    if let Some(year) = matter.created {
        // tracing::info!("  year={:?}", year.year());
        new_name.push(year.format("%Y").to_string());

        let title = matter.title.unwrap_or("NA".to_string());
        let filename = title.replace(" ", "_");
        new_name.push(format!("{}-{}", year.format("%Y-%m-%d"), filename,));
    }

    let new_path = Path::new("../content/posts").join(new_name);

    anyhow::ensure!(
        new_path.parent().and_then(|x| x.parent()).unwrap().is_dir(),
        "{:?} is not a directory",
        new_path.parent().and_then(|x| x.parent())
    );

    // create parent of this new_path is doesn't exists.
    if let Some(parent) = new_path.parent()
        && !parent.is_dir() {
            std::fs::create_dir(parent)?;
        }

    // copy to new location.
    std::fs::write(&new_path, md_text)?;
    println!("Wrote {path:?} to {new_path:?}");

    Ok(())
}
