use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory to scan for z-index definitions
    #[arg(value_name = "DIRECTORY")]
    directory: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ZIndexEntry {
    file_path: String,
    z_index: String,
    line_number: usize,
}

fn scan_file(path: &Path) -> Result<Vec<ZIndexEntry>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read file: {}", path.display()))?;
    
    let z_index_pattern = Regex::new(r"z-\[(\d+)\]|z-(\d+)|zIndex:\s*(\d+)|z-index:\s*(\d+)")?;
    let mut entries = Vec::new();
    
    for (line_number, line) in content.lines().enumerate() {
        if let Some(cap) = z_index_pattern.captures(line) {
            let z_index = cap.iter()
                .skip(1)
                .find_map(|m| m)
                .map(|m| m.as_str())
                .unwrap_or("unknown");
            
            entries.push(ZIndexEntry {
                file_path: path.to_string_lossy().to_string(),
                z_index: z_index.to_string(),
                line_number: line_number + 1,
            });
        }
    }
    
    Ok(entries)
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    let mut all_entries = Vec::new();
    
    for entry in WalkDir::new(&args.directory)
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden directories and node_modules
            let path = e.path();
            let is_hidden = path.file_name()
                .and_then(|name| name.to_str())
                .map(|name| name.starts_with('.'))
                .unwrap_or(false);
            
            !is_hidden && !path.to_string_lossy().contains("/node_modules/")
        })
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ["js", "jsx", "ts", "tsx"].contains(&ext.to_string_lossy().to_lowercase().as_str()) {
                    match scan_file(path) {
                        Ok(entries) => all_entries.extend(entries),
                        Err(e) => eprintln!("Error scanning {}: {}", path.display(), e),
                    }
                }
            }
        }
    }
    
    // Sort entries by z-index value
    all_entries.sort_by(|a, b| b.z_index.parse::<i32>().unwrap_or(0)
        .cmp(&a.z_index.parse::<i32>().unwrap_or(0)));
    
    // Print results in a tree-like structure
    println!("\nZ-Index Tree:");
    println!("=============");
    
    for entry in all_entries {
        println!("z-{}", entry.z_index);
        println!("  ├─ File: {}", entry.file_path);
        println!("  └─ Line: {}", entry.line_number);
        println!();
    }
    
    Ok(())
}