// Output Formatting

use anyhow::Result;

/// Output format options
#[derive(Debug, Clone)]
pub enum OutputFormat {
    Table,
    Json,
    Yaml,
    Plain,
}

impl OutputFormat {
    pub fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::Json),
            "yaml" | "yml" => Ok(Self::Yaml),
            "plain" | "text" => Ok(Self::Plain),
            _ => Err(anyhow::anyhow!("Unknown output format: {}", s)),
        }
    }
}