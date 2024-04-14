use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};
use std::path::Path;

const PERS_NR: &str = "pers_nr";
const PAISY_ABRNR: &str = "paisy_abrnr";
const NAME: &str = "name";
const VORNAME: &str = "vorname";
const DATUM: &str = "datum";
const LOHNART_BESCHREIBUNG: &str = "lohnart_beschreibung";
const WERTVERAENDERUNG: &str = "wertveraenderung";

lazy_static! {
    static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9]{6}").unwrap();
}

// Funktion zum Lesen der Datei
pub fn get_text_data_as_rows(path: &str) -> io::Result<Vec<HashMap<String, String>>> {
    let path = Path::new(path);
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut rows = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if should_process(&line) {
            let row =
                line_to_map(&line).map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?;
            rows.push(row);
        }
    }

    Ok(rows)
}

// Eine Funktion, die eine Zeile der Textdatei nimmt und ein HashMap erstellt
fn line_to_map(line: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert(PERS_NR.to_string(), line[0..12].trim().to_string());
    map.insert(PAISY_ABRNR.to_string(), line[12..26].trim().to_string());
    map.insert(NAME.to_string(), line[26..58].trim().to_string());
    map.insert(VORNAME.to_string(), line[58..82].trim().to_string());
    map.insert(DATUM.to_string(), line[82..92].trim().to_string());
    map.insert(
        LOHNART_BESCHREIBUNG.to_string(),
        line[92..159].trim().to_string(),
    );
    map.insert(WERTVERAENDERUNG.to_string(), line[159..].trim().to_string());

    let time = line[159..].replace(" ", "").replace(",", ".");
    map.insert("wertveraenderung".to_string(), format_time(time.as_str())?);

    Ok(map)
}

fn should_process(line: &str) -> bool {
    RE.is_match(line)
}

fn format_time(time: &str) -> Result<String, std::num::ParseFloatError> {
    let time: f32 = time.parse()?;
    let sign = if time.is_sign_negative() { "-" } else { "+" };
    let hours = time.trunc() as i32;
    let minutes = ((time - hours as f32).abs() * 60.0).round() as i32;
    Ok(format!("{} {:02}:{:02}", sign, hours.abs(), minutes))
}
