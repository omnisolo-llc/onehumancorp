use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ToolError {
    Transient(String),
    LlmRecoverable(String),
    UserFixable(String),
    Fatal(String),
}

impl fmt::Display for ToolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Transient(e) => write!(f, "Transient error: {}", e),
            Self::LlmRecoverable(e) => write!(f, "{}", e),
            Self::UserFixable(e) => write!(f, "User action required: {}", e),
            Self::Fatal(e) => write!(f, "Fatal error: {}", e),
        }
    }
}

impl std::error::Error for ToolError {}

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

// simulate async_trait by returning the result
fn execute() -> Result<String, ToolError> {
    let opt: Option<&str> = None;
    let _ = opt.ok_or("read: path is required")?;

    let e = Err(std::io::Error::new(std::io::ErrorKind::NotFound, "not found"));
    let _ = e.map_err(|e| format!("io error: {}", e))?;

    Ok("done".to_string())
}

fn main() {
    println!("{:?}", execute());
}
