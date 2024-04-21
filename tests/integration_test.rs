use rust_paisy2pdf::common::mock_string::MOCK_STRINGS;
use rust_paisy2pdf::process;

#[test]
fn test_full_processing() {
    let paths = ["C:\\Users\\isla1\\OneDrive\\Desktop\\Data\\text_data.txt"];
    let optional_paths = ["ohnealles.txt"];
    let output_path = "C:\\Users\\isla1\\OneDrive\\Desktop\\Data\\";
    let result = process::process(MOCK_STRINGS, output_path, &optional_paths);
    //println!("{:}", mock_string::MOCK_STRINGS[0]);
    assert!(result.is_ok());
}
