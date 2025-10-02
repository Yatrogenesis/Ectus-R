// Code Generation Commands

use anyhow::Result;
use std::path::PathBuf;
use tokio::fs;
use indicatif::{ProgressBar, ProgressStyle};
use console::style;

use crate::{
    client::{AionClient, GenerateCodeRequest},
    output::OutputFormat,
    utils,
    GenerateCommands,
};

pub async fn handle_generate_command(
    command: GenerateCommands,
    client: &AionClient,
    output_format: &OutputFormat,
) -> Result<()> {
    match command {
        GenerateCommands::Code {
            requirements,
            file,
            language,
            framework,
            architecture,
            optimization,
            output_dir,
            include_tests,
            include_docs,
        } => {
            generate_code(
                client,
                requirements,
                file,
                language,
                framework,
                architecture,
                optimization,
                output_dir,
                include_tests,
                include_docs,
                output_format,
            ).await
        }
        GenerateCommands::List { limit, page } => {
            list_generations(client, page, limit, output_format).await
        }
        GenerateCommands::Get { id } => {
            get_generation(client, &id, output_format).await
        }
        GenerateCommands::Download { id, output } => {
            download_generation(client, &id, output, output_format).await
        }
        GenerateCommands::Delete { id, force } => {
            delete_generation(client, &id, force, output_format).await
        }
    }
}

async fn generate_code(
    client: &AionClient,
    requirements: Option<String>,
    file: Option<PathBuf>,
    language: String,
    framework: Option<String>,
    architecture: String,
    optimization: String,
    output_dir: PathBuf,
    include_tests: bool,
    include_docs: bool,
    output_format: &OutputFormat,
) -> Result<()> {
    // Get requirements text
    let requirements_text = match (requirements, file) {
        (Some(text), None) => text,
        (None, Some(path)) => {
            println!("{}", style("📄 Reading requirements from file...").dim());
            fs::read_to_string(&path).await?
        }
        (Some(_), Some(_)) => {
            return Err(anyhow::anyhow!("Cannot specify both --requirements and --file"));
        }
        (None, None) => {
            // Interactive mode
            println!("{}", style("✨ Interactive Requirements Entry").bold().cyan());
            println!("Enter your requirements (press Ctrl+D when finished):");
            utils::read_multiline_input()?
        }
    };

    // Validate inputs
    if requirements_text.trim().is_empty() {
        return Err(anyhow::anyhow!("Requirements cannot be empty"));
    }

    println!("{}", style("🚀 Starting code generation...").bold().green());
    println!("  Language: {}", style(&language).yellow());
    println!("  Architecture: {}", style(&architecture).yellow());
    if let Some(ref fw) = framework {
        println!("  Framework: {}", style(fw).yellow());
    }
    println!("  Optimization: {}", style(&optimization).yellow());
    println!();

    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap()
    );
    pb.set_message("Analyzing requirements...");

    // Create generation request
    let request = GenerateCodeRequest {
        requirements: requirements_text,
        language,
        framework,
        architecture: Some(architecture),
        optimization_level: Some(optimization),
        constraints: Some(serde_json::json!({
            "include_tests": include_tests,
            "include_docs": include_docs
        })),
        context: None,
    };

    // Send generation request
    pb.set_message("Generating code...");
    let result = client.generate_code(request).await?;
    pb.finish_and_clear();

    // Display results
    match output_format {
        OutputFormat::Table => {
            println!("{}", style("✅ Code generation completed!").bold().green());
            println!();
            println!("📊 {}", style("Generation Summary:").bold());
            println!("  ID: {}", style(&result.id).cyan());
            println!("  Files generated: {}", style(result.generated_files_count).yellow());
            println!("  Lines of code: {}", style(result.total_lines_of_code).yellow());
            println!("  Time saved: {:.1} hours", result.estimated_time_saved_hours);
            println!();

            if !result.suggestions.is_empty() {
                println!("💡 {}", style("Suggestions:").bold());
                for suggestion in &result.suggestions {
                    println!("  • {}", suggestion);
                }
                println!();
            }

            println!("📁 {}", style("File Structure:").bold());
            for file in &result.preview.structure {
                println!("  {} ({}) - {}",
                    style(&file.path).cyan(),
                    style(&file.language).dim(),
                    file.purpose
                );
            }
            println!();

            println!("📖 Documentation: {}", style(&result.documentation_url).blue().underlined());
            println!("⬇️  Download: {}", style(&result.download_url).blue().underlined());
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&result)?);
        }
        OutputFormat::Plain => {
            println!("Generation ID: {}", result.id);
            println!("Files: {}", result.generated_files_count);
            println!("Lines of code: {}", result.total_lines_of_code);
        }
    }

    // Ask if user wants to download
    if utils::confirm("Download generated code now?")? {
        download_generation(client, &result.id, output_dir, output_format).await?;
    }

    Ok(())
}

async fn list_generations(
    client: &AionClient,
    page: u32,
    per_page: u32,
    output_format: &OutputFormat,
) -> Result<()> {
    let result = client.list_generations(page, per_page).await?;

    match output_format {
        OutputFormat::Table => {
            use tabled::{Table, Tabled};

            #[derive(Tabled)]
            struct GenerationRow {
                #[tabled(rename = "ID")]
                id: String,
                #[tabled(rename = "Requirements")]
                requirements: String,
                #[tabled(rename = "Language")]
                language: String,
                #[tabled(rename = "Files")]
                files: String,
                #[tabled(rename = "Created")]
                created: String,
            }

            if let Some(generations) = result["generations"].as_array() {
                if generations.is_empty() {
                    println!("{}", style("No generations found").dim());
                    return Ok(());
                }

                let rows: Vec<GenerationRow> = generations
                    .iter()
                    .map(|gen| GenerationRow {
                        id: gen["id"].as_str().unwrap_or("").to_string(),
                        requirements: utils::truncate_text(
                            gen["requirements_preview"].as_str().unwrap_or(""),
                            50
                        ),
                        language: gen["language"].as_str().unwrap_or("").to_string(),
                        files: gen["files_count"].as_u64().unwrap_or(0).to_string(),
                        created: utils::format_timestamp(gen["created_at"].as_str().unwrap_or("")),
                    })
                    .collect();

                let table = Table::new(rows);
                println!("{}", table);

                println!();
                println!("Page {} of {} (total: {})",
                    result["page"].as_u64().unwrap_or(1),
                    (result["total"].as_u64().unwrap_or(0) + per_page as u64 - 1) / per_page as u64,
                    result["total"].as_u64().unwrap_or(0)
                );
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&result)?);
        }
        OutputFormat::Plain => {
            if let Some(generations) = result["generations"].as_array() {
                for gen in generations {
                    println!("{} - {} ({})",
                        gen["id"].as_str().unwrap_or(""),
                        gen["requirements_preview"].as_str().unwrap_or(""),
                        gen["language"].as_str().unwrap_or("")
                    );
                }
            }
        }
    }

    Ok(())
}

async fn get_generation(
    client: &AionClient,
    id: &str,
    output_format: &OutputFormat,
) -> Result<()> {
    let result = client.get_generation_status(id).await?;

    match output_format {
        OutputFormat::Table => {
            println!("{}", style("📋 Generation Details").bold());
            println!();
            println!("ID: {}", style(id).cyan());
            println!("Status: {}", style(result["status"].as_str().unwrap_or("unknown")).yellow());
            if let Some(progress) = result["progress"].as_f64() {
                println!("Progress: {:.1}%", progress * 100.0);
            }
            if let Some(message) = result["message"].as_str() {
                println!("Message: {}", message);
            }
            if let Some(created) = result["created_at"].as_str() {
                println!("Created: {}", utils::format_timestamp(created));
            }
            if let Some(updated) = result["updated_at"].as_str() {
                println!("Updated: {}", utils::format_timestamp(updated));
            }
        }
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&result)?);
        }
        OutputFormat::Plain => {
            println!("ID: {}", id);
            println!("Status: {}", result["status"].as_str().unwrap_or("unknown"));
        }
    }

    Ok(())
}

async fn download_generation(
    client: &AionClient,
    id: &str,
    output_dir: PathBuf,
    output_format: &OutputFormat,
) -> Result<()> {
    println!("{}", style("⬇️  Downloading generated code...").bold().blue());

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.blue} {msg}")
            .unwrap()
    );
    pb.set_message("Downloading...");

    let archive_data = client.download_generated_code(id).await?;
    pb.finish_and_clear();

    // Create output directory
    fs::create_dir_all(&output_dir).await?;

    // Extract archive
    let archive_path = output_dir.join(format!("generation-{}.zip", id));
    fs::write(&archive_path, &archive_data).await?;

    println!("{}", style("📦 Extracting archive...").dim());
    utils::extract_zip(&archive_path, &output_dir)?;

    // Remove archive file
    fs::remove_file(&archive_path).await?;

    match output_format {
        OutputFormat::Table => {
            println!("{}", style("✅ Download completed!").bold().green());
            println!("📁 Files extracted to: {}", style(output_dir.display()).cyan());
        }
        OutputFormat::Json => {
            println!("{}", serde_json::json!({
                "status": "success",
                "output_directory": output_dir.to_string_lossy(),
                "generation_id": id
            }));
        }
        OutputFormat::Yaml => {
            println!("status: success");
            println!("output_directory: {}", output_dir.display());
            println!("generation_id: {}", id);
        }
        OutputFormat::Plain => {
            println!("Downloaded to: {}", output_dir.display());
        }
    }

    Ok(())
}

async fn delete_generation(
    client: &AionClient,
    id: &str,
    force: bool,
    output_format: &OutputFormat,
) -> Result<()> {
    if !force && !utils::confirm(&format!("Delete generation {}?", id))? {
        println!("Cancelled");
        return Ok(());
    }

    println!("{}", style("🗑️  Deleting generation...").dim());
    client.delete_generation(id).await?;

    match output_format {
        OutputFormat::Table => {
            println!("{}", style("✅ Generation deleted successfully").green());
        }
        OutputFormat::Json => {
            println!("{}", serde_json::json!({
                "status": "deleted",
                "generation_id": id
            }));
        }
        OutputFormat::Yaml => {
            println!("status: deleted");
            println!("generation_id: {}", id);
        }
        OutputFormat::Plain => {
            println!("Deleted: {}", id);
        }
    }

    Ok(())
}