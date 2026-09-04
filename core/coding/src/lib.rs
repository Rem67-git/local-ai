pub mod parser;
pub mod analyzer;
pub mod modifier;
pub mod error;

pub use parser::{CodeParser, ParsedCode};
pub use analyzer::{CodeAnalyzer, CodeIssue, IssueType};
pub use modifier::CodeModifier;
pub use error::{CodingError, CodingResult};
