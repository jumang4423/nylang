pub fn str_to_bool(_str: String) -> bool {
  match _str.to_lowercase().as_str() {
    "true" => true,
    "false" => false,
    "1" => true,
    "0" => false,
    _ => false,
  }
}
