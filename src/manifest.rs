use inquire::{Confirm, Text};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub title: String,
    pub description: String,
    pub version: String,

    pub elements: Vec<Element>,
}

impl Manifest {
    fn prompt_element(element: &Element) -> Result<ElementValue> {
        let description = format!("{}\n  {}\n ", element.title, element.description);

        match &element.default {
            Some(ElementValue::Boolean(default)) => {
                let val = Confirm::new(&description)
                    .with_default(*default)
                    .prompt()?;
                Ok(ElementValue::Boolean(val))
            }
            Some(ElementValue::String(default)) => {
                let val = Text::new(&description)
                    .with_default(default)
                    .prompt()?;
                Ok(ElementValue::String(val))
            }
            Some(ElementValue::Number(default)) => {
                let val = Text::new(&description)
                    .with_default(&default.to_string())
                    .prompt()?;
                let parsed = val.parse::<i64>()?;
                Ok(ElementValue::Number(parsed))
            }
            None => {
                let val = Text::new(&description).prompt()?;
                Ok(ElementValue::String(val))
            }
        }
    }

    pub fn prompt_all(&self) -> Result<Vec<(String, ElementValue)>> {
        let mut results = Vec::new();

        for element in self.elements.iter() {
            let value = Self::prompt_element(element)?;
            results.push((element.id.clone(), value));
        }

        Ok(results)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ElementValue {
    Boolean(bool),
    Number(i64),
    String(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element {
    pub id: String,
    pub title: String,
    pub description: String,
    pub default: Option<ElementValue>,
}
