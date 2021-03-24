
pub fn get_test_input(file_name: &str) -> String {
    std::fs::read_to_string(file_name).unwrap()
}