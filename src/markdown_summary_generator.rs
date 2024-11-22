
use std::{
    fs::{metadata, read_dir, File},
    io::{Result, Write},
    path::{Path, PathBuf},
};

// TODO 
// Tests 
// Documentation 

pub struct MarkdownSummaryGenerator {
    root: PathBuf,
}

impl MarkdownSummaryGenerator {
    pub fn new() -> Self { 
        let root: PathBuf = std::env::current_dir()
        .expect("Cannot get current directory ")
        .join( "src");
        MarkdownSummaryGenerator { root }
    }

    pub fn build(&self) {
        let mut entries: Vec<String> = Vec::new();

        self.traverse_dir(&self.root, &mut entries).expect("Failed to process directory");

        entries.sort();
        
        self.write(entries, &self.root).expect("Error writing Summary MD");
    }

    fn traverse_dir(&self, dir: &Path, entries: &mut Vec<String>) -> Result<()> {
        read_dir(dir)?
            .filter_map(Result::ok)
            .for_each(|entry| {
                let path = entry.path();
                entries.push(self.format_path(&path));
                 if path.is_dir() {
                    self.traverse_dir(&path, entries)
                    .expect("Failed to process directory");
                }
            });
        Ok(())
    }


fn write(&self, folders: Vec<String>, root: &Path) -> Result<()> {
    let summary_file = &root.join("SUMMARY.md");
    
    let metadata = metadata(summary_file)?;

    if  metadata.len() != 0 {
        return Ok(());
    }
    
    let mut summary = File::create(summary_file).expect("Cannot create summary file");
    
    writeln!(summary, "# Summary").expect("Failed to write to summary file");
    
    for path in folders {
        let indent_level = path.matches('/').count();
        let indent = "  ".repeat(indent_level);
        
        if path.ends_with(".md") && !path.contains("README") && !path.contains("SUMMARY.md"){
            writeln!(
                summary,
                "{}- [{}]({})",
                indent,
                self.file_title(Path::new(&path)),
                path
            )
            .expect("Failed to write to summary file");
        } else {
            if  !path.ends_with("rs") && !path.contains("README") && !path.contains("SUMMARY.md") {

                let p = path.ends_with(|c: char| c.is_alphabetic());

                if p {
                    writeln!(
                        summary,
                    "{}- [{}]({}/README.md)",
                    indent,
                    self.file_title(Path::new(&path)),
                    path
                )
                .expect("Failed to write to summary file");
        }
        }
    }
}

Ok(())
}

fn capitalize_first_letter(&self, s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn file_title(&self, path: &Path) -> String {
    path.file_stem()
    .expect("Failed to find file stem")
    .to_str()
        .map(|f| {
            f.replace("_", " ")
            .replace("-", " ")
                .split_whitespace()
                .map(|s| self.capitalize_first_letter(s))
                .collect::<Vec<String>>()
                .join(" ")
        })
        .unwrap_or_default()
    }
    
    fn format_path(&self, path: &Path) -> String {
    let base_path = path
        .ancestors()
        .find(|p| p.ends_with("src"))
        .unwrap_or(path);

    let relative_path = path.strip_prefix(base_path).unwrap_or(path);
    
    format!(
        "./{}",
        relative_path
            .to_str()
            .map(|f| f.replace("\\", "/"))
            .expect("Error formatting the the file path")
        )
    }
}
