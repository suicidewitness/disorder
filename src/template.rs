use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::manifest::ElementValue;
use anyhow::{anyhow, Context, Result};
use tera::{Context as TeraContext, Tera};
use log::warn;

pub fn collect_files(root: &Path, out_path: &Path, manifest_name: &str) -> Vec<(PathBuf, bool)> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| !e.path().starts_with(out_path))
        .filter(|e| !e.path().to_string_lossy().ends_with(manifest_name))
        .map(|e| {
            // shouldn't panic?
            let rel = e.path().strip_prefix(root).context("Failed to strip prefix").unwrap().to_path_buf();
            let is_template = rel.to_string_lossy().ends_with(".tera");
            (rel, is_template)
        })
        .collect()
}

pub fn process_template(template_root: &Path, manifest_name: &str, data: Vec<(String, ElementValue)>, output_root: &Path) -> Result<()> {
    if !output_root.exists() {
        fs::create_dir_all(&output_root)?;
    }

    let mut context = TeraContext::new();

    for (key, val) in data {
        match val {
            ElementValue::Boolean(b) => context.insert(key, &b),
            ElementValue::String(s) => context.insert(key, &s),
            ElementValue::Number(n) => context.insert(key, &n),
        }
    }

    let files = collect_files(template_root, output_root, manifest_name);
    let mut tera = Tera::default();

    for (rel, is_template) in files.iter() {
        let src = template_root.join(&rel);

        let out_rel = if *is_template {
            let s = rel.to_string_lossy();
            PathBuf::from(s.strip_suffix(".tera").unwrap())
        } else {
            rel.clone()
        };
        let dest = output_root.join(&out_rel);

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)?;
        }

        if *is_template {
            let content = fs::read_to_string(&src)
                .with_context(|| format!("Failed to read {}", src.display()))?;

            let name = rel.to_string_lossy().to_string();
            tera.add_raw_template(&name, &content)
                .with_context(|| format!("Failed to parse template {}", rel.display()))?;

            let rendered = tera.render(&name, &context)
                .with_context(|| format!("Failed to render {}", rel.display()))?;

            fs::write(&dest, rendered)?;
        } else {
            fs::copy(&src, &dest)?;
        }
    }

    Ok(())
}
