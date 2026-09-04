use crate::error::{CodingError, CodingResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedCode {
    pub language: String,
    pub filename: String,
    pub content: String,
    pub lines: Vec<String>,
    pub line_count: usize,
}

impl ParsedCode {
    pub fn get_line(&self, line_num: usize) -> Option<&str> {
        if line_num > 0 && line_num <= self.lines.len() {
            Some(&self.lines[line_num - 1])
        } else {
            None
        }
    }

    pub fn get_lines(&self, start: usize, end: usize) -> Vec<String> {
        self.lines
            .iter()
            .skip(start.saturating_sub(1))
            .take(end - start.saturating_sub(1))
            .cloned()
            .collect()
    }
}

pub struct CodeParser;

impl CodeParser {
    pub fn parse(filename: &str, content: &str) -> CodingResult<ParsedCode> {
        let language = Self::detect_language(filename)?;
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let line_count = lines.len();

        Ok(ParsedCode {
            language,
            filename: filename.to_string(),
            content: content.to_string(),
            lines,
            line_count,
        })
    }

    fn detect_language(filename: &str) -> CodingResult<String> {
        if let Some(ext) = filename.split('.').last() {
            match ext {
                "rs" => Ok("rust".to_string()),
                "py" => Ok("python".to_string()),
                "js" | "ts" => Ok("javascript".to_string()),
                "go" => Ok("go".to_string()),
                "c" | "h" => Ok("c".to_string()),
                "cpp" | "cc" | "cxx" => Ok("cpp".to_string()),
                "java" => Ok("java".to_string()),
                "json" => Ok("json".to_string()),
                "yaml" | "yml" => Ok("yaml".to_string()),
                "toml" => Ok("toml".to_string()),
                "md" => Ok("markdown".to_string()),
                "txt" => Ok("text".to_string()),
                _ => Ok("unknown".to_string()),
            }
        } else {
            Ok("unknown".to_string())
        }
    }

    pub fn validate_syntax(code: &ParsedCode) -> CodingResult<()> {
        // Basic validation: check for common syntax errors
        let mut brace_count = 0;
        let mut bracket_count = 0;
        let mut paren_count = 0;

        for (line_num, line) in code.lines.iter().enumerate() {
            for ch in line.chars() {
                match ch {
                    '{' => brace_count += 1,
                    '}' => brace_count -= 1,
                    '[' => bracket_count += 1,
                    ']' => bracket_count -= 1,
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    _ => {}
                }

                if brace_count < 0 || bracket_count < 0 || paren_count < 0 {
                    return Err(CodingError::SyntaxError {
                        line: line_num + 1,
                        message: "Unmatched closing bracket/brace/paren".to_string(),
                    });
                }
            }
        }

        if brace_count != 0 {
            return Err(CodingError::SyntaxError {
                line: code.line_count,
                message: "Unmatched braces".to_string(),
            });
        }
        if bracket_count != 0 {
            return Err(CodingError::SyntaxError {
                line: code.line_count,
                message: "Unmatched brackets".to_string(),
            });
        }
        if paren_count != 0 {
            return Err(CodingError::SyntaxError {
                line: code.line_count,
                message: "Unmatched parentheses".to_string(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rust_code() {
        let code = "fn main() {\n    println!(\"Hello\");\n}";
        let parsed = CodeParser::parse("main.rs", code).unwrap();

        assert_eq!(parsed.language, "rust");
        assert_eq!(parsed.line_count, 3);
        assert_eq!(parsed.get_line(1), Some("fn main() {"));
    }

    #[test]
    fn test_detect_language() {
        assert_eq!(CodeParser::detect_language("test.rs").unwrap(), "rust");
        assert_eq!(CodeParser::detect_language("test.py").unwrap(), "python");
        assert_eq!(CodeParser::detect_language("test.js").unwrap(), "javascript");
    }

    #[test]
    fn test_validate_syntax_valid() {
        let code = CodeParser::parse(
            "test.rs",
            "fn main() {\n    let x = [1, 2, 3];\n}",
        ).unwrap();

        assert!(CodeParser::validate_syntax(&code).is_ok());
    }

    #[test]
    fn test_validate_syntax_unmatched_brace() {
        let code = CodeParser::parse(
            "test.rs",
            "fn main() {\n    let x = 5;\n",
        ).unwrap();

        assert!(CodeParser::validate_syntax(&code).is_err());
    }

    #[test]
    fn test_get_lines_range() {
        let code = CodeParser::parse(
            "test.txt",
            "line1\nline2\nline3\nline4\nline5",
        ).unwrap();

        let lines = code.get_lines(2, 4);
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line2");
    }
}
