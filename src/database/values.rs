pub enum DatabaseValue {
    None,
    Some(String),
    String(String),
    Int(i32),
    Int64(i64),
    Float(f64),
    Boolean(bool),
}
