use std::path::Path;
use anyhow::Result;
use inquire::{Confirm, Select, Text};
use crate::manifest::{Element, ElementValue};

pub fn init(manifest_path: &Path) -> Result<()> {
    let name = Text::new("Name of the template.")
        .prompt()?;
    let description = Text::new("Description of the template.")
        .prompt()?;
    let version = Text::new("Version of the template.")
        .with_default("1.0.0")
        .prompt()?;

    let mut elements: Vec<Element> = Vec::new();
    let mut i = 0;

    loop {
        print!("\n");

        let message = if i == 0 {
            "Do you want to add an element?"
        } else {
            "Do you want to add another element?"
        };

        if !Confirm::new(message).with_default(true).prompt()? {
            break;
        }

        let id = Text::new("Element identifier (will be exposed for Tera templates).").prompt()?;
        let title = Text::new("Element title.").prompt()?;
        let el_description = Text::new("Element description.").prompt()?;

        let kind = Select::new("Element type.", vec!["String", "Boolean", "Number"]).prompt()?;

        let default = match kind {
            "Boolean" => {
                let val = Confirm::new("Default value?").with_default(false).prompt()?;
                Some(ElementValue::Boolean(val))
            }
            "Number" => {
                let val = Text::new("Default value? (leave empty to skip)")
                    .prompt()?;
                if val.is_empty() {
                    None
                } else {
                    Some(ElementValue::Number(val.parse()?))
                }
            }
            "String" => {
                let val = Text::new("Default value? (leave empty to skip)").prompt()?;
                if val.is_empty() { None } else { Some(ElementValue::String(val)) }
            }
            _ => None,
        };

        elements.push(Element {
            id,
            title,
            description: el_description,
            default,
        });

        i += 1;
    }

    let manifest = crate::manifest::Manifest { title: name, description, version, elements };
    let toml = toml::to_string_pretty(&manifest)?;
    std::fs::write(manifest_path, toml)?;

    Ok(())
}
