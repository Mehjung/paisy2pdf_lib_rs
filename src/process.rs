use crate::html_template_generator::process_files;

pub fn process(paths: &[&str], output_path: &str) -> std::io::Result<()> {
    process_files(paths, output_path)
}
