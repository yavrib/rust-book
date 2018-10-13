pub fn connect() -> String {
  format!("This is server!")
}

pub fn super_connect() -> String {
  super::connect()
}
