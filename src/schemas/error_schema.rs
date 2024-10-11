use serde::{Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorSchema {
    pub error: String,
}
