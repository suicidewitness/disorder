use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::manifest::Manifest;
use anyhow::Result;
use inquire::Text;
use clap::{Parser, Subcommand};

mod manifest;
mod template;
mod init;

#[derive(Parser)]
#[command(name = "templator", about = "Tera-based template scaffolder.")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate a project from a manifest template
    Create {
        /// Path to manifest.toml
        #[arg(short, long, default_value = "manifest.toml")]
        manifest_path: String,

        /// Directory of the template
        #[arg(short = 'i', long, default_value = "./")]
        input: String,

        /// Output directory (prompts if not provided)
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Interactively create a new manifest.toml
    Init {
        /// Where to write the manifest
        #[arg(short, long, default_value = "manifest.toml")]
        output: String,
    },
}

fn create(manifest_path: &str, input: &str, output: Option<String>) -> Result<()> {
    let file = std::fs::read_to_string(manifest_path)?;
    let manifest = toml::from_str::<Manifest>(&file)?;

    println!("  {} ({})", &manifest.title, &manifest.version);
    println!("  {}\n", &manifest.description);

    let out_path = match output {
        Some(o) => o,
        None => Text::new("Output directory of the template")
            .with_default("./out")
            .prompt()?,
    };

    let template_root = PathBuf::from(input);

    template::process_template(&template_root, manifest_path, manifest.prompt_all()?, Path::new(&out_path))?;

    println!("Successfully filled out the template!");

    Ok(())
}

fn init(output: &str) -> Result<()> {
    init::init(&PathBuf::from_str(output)?)?;

    print!("\n");
    println!("Successfully initialized the template!");
    println!("Use `disorder create` to scaffold a project out of this template.");

    Ok(())
}

fn main() -> Result<()> {
    match Args::parse().command {
        Commands::Create { manifest_path, input, output } => create(&manifest_path, &input, output),
        Commands::Init { output } => init(&output),
    }
}
