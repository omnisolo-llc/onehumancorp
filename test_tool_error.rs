#[derive(Debug)]
pub enum ToolError {
    Transient(String),
    LlmRecoverable(String),
    UserFixable(String),
    Fatal(String),
}

impl From<String> for ToolError {
    fn from(e: String) -> Self {
        ToolError::LlmRecoverable(e)
    }
}

impl From<&str> for ToolError {
    fn from(e: &str) -> Self {
        ToolError::LlmRecoverable(e.to_string())
    }
}

fn do_something() -> Result<String, ToolError> {
    let opt: Option<&str> = None;
    let _ = opt.ok_or("missing val")?;

    let res: Result<(), String> = Err("some err".to_string());
    res?;

    Ok("done".to_string())
}

fn main() {
    println!("{:?}", do_something());
}
