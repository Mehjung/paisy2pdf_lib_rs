use crate::get_text_data::get_text_data_as_rows;
use minijinja::{Environment, Value};
use path_clean::PathClean;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub fn process_files(
    paths: &[&str],
    output_path: &str,
    optional_paths: &[&str],
) -> std::io::Result<()> {
    let binding = normalize_path(output_path);
    let cleaned_output_path = binding.as_str();
    for (path, optional_path) in paths.iter().zip(optional_paths.iter()) {
        let new_path = replace_path(optional_path, cleaned_output_path).ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::Other, "Failed to generate new path")
        })?;

        generate_htmlstring_template(path, &new_path)?;
    }
    Ok(())
}

fn generate_htmlstring_template(path: &str, new_path: &str) -> std::io::Result<()> {
    let rows = get_text_data_as_rows(&path)?;

    let rendered = render_template(rows)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let html_output_path = add_suffix(&new_path, "html").ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to add HTML suffix".to_string(),
    ))?;

    let output_pdf_path = add_suffix(&new_path, "pdf").ok_or(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Failed to add PDF suffix".to_string(),
    ))?;

    write_to_file(&html_output_path, &rendered)?;

    let _ = convert_html_to_pdf(html_output_path.clone(), output_pdf_path);
    remove_file(&html_output_path)?;
    Ok(())
}

fn render_template(
    rows: Vec<HashMap<String, String>>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut env = Environment::new();
    let template_string = include_str!("../template.html");

    //let template_string = read_to_string(template)?;
    env.add_template("table.html", &template_string)?;

    let template = env.get_template("table.html")?;
    let mut context = HashMap::new();
    context.insert("rows", Value::from(rows));

    Ok(template.render(&context)?)
}

fn write_to_file(path: &str, content: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())
}

fn add_suffix(file_path: &str, suffix: &str) -> Option<String> {
    let path = Path::new(file_path);
    if let Some(stem) = path.file_stem().and_then(OsStr::to_str) {
        let mut new_path = PathBuf::from(path.parent().unwrap_or(Path::new("")));
        new_path.push(format!("{}.{}", stem, suffix));
        Some(new_path.to_string_lossy().into_owned())
    } else {
        None
    }
}

fn replace_path(source_path: &str, target_path: &str) -> Option<String> {
    let source = Path::new(source_path);
    let target = Path::new(target_path);

    // Extrahieren des Dateinamens aus dem Quellpfad
    if let Some(file_name) = source.file_name().and_then(OsStr::to_str) {
        // Erstellen eines neuen Pfadobjekts basierend auf dem Zielverzeichnis
        let mut new_path = PathBuf::from(target);
        // Hinzufügen des Dateinamens zum Zielverzeichnis
        new_path.push(file_name);
        // Konvertierung des Pfadobjekts in einen String und Rückgabe
        Some(new_path.to_string_lossy().into_owned())
    } else {
        // Rückgabe von None, falls kein Dateiname im Quellpfad gefunden wurde
        None
    }
}

fn convert_html_to_pdf(html_file_path: String, output_pdf_path: String) -> std::io::Result<()> {
    let chrome_path = "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe";
    let args = [
        "--headless",
        "--disable-gpu",
        "--no-pdf-header-footer",
        &format!("--print-to-pdf={}", Path::new(&output_pdf_path).display()),
        &format!("file://{}", Path::new(&html_file_path).display()),
    ];

    let mut child = Command::new(chrome_path).args(&args).spawn()?;

    let ecode = child.wait()?;

    if ecode.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to generate PDF",
        ))
    }
}

fn normalize_path(path: &str) -> String {
    let path_buf = PathBuf::from(path);
    let cleaned_path = path_buf.clean(); // Verwenden Sie die clean Methode, um den Pfad zu bereinigen
    cleaned_path.to_string_lossy().into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_path() {
        let path = "C:\\Users\\isla1\\OneDrive\\Desktop\\Data\\";
        let normalized_path = normalize_path(path);
        assert_eq!(normalized_path, "C:\\Users\\isla1\\OneDrive\\Desktop\\Data");
    }
}
