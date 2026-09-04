use crate::error::CodingResult;
use crate::parser::ParsedCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IssueType {
    SyntaxError,
    StyleIssue,
    PotentialBug,
    PerformanceWarning,
    SecurityConcern,
    UnusedCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeIssue {
    pub issue_type: IssueType,
    pub line: usize,
    pub column: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
}

pub struct CodeAnalyzer;

impl CodeAnalyzer {
    pub fn analyze(code: &ParsedCode) -> CodingResult<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        // Check for common issues based on language
        match code.language.as_str() {
            "python" => issues.extend(Self::analyze_python(code)?),
            "rust" => issues.extend(Self::analyze_rust(code)?),
            "javascript" | "js" => issues.extend(Self::analyze_javascript(code)?),
            _ => {}
        }

        // Generic checks
        issues.extend(Self::check_trailing_whitespace(code)?);
        issues.extend(Self::check_long_lines(code)?);

        Ok(issues)
    }

    fn analyze_python(code: &ParsedCode) -> CodingResult<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        for (line_num, line) in code.lines.iter().enumerate() {
            // Check for missing colons on control structures
            if (line.trim().starts_with("if ")
                || line.trim().starts_with("for ")
                || line.trim().starts_with("while ")
                || line.trim().starts_with("def ")
                || line.trim().starts_with("class "))
                && !line.trim().ends_with(':')
                && !line.trim().ends_with('\\')
            {
                issues.push(CodeIssue {
                    issue_type: IssueType::SyntaxError,
                    line: line_num + 1,
                    column: None,
                    message: "Control structure missing colon".to_string(),
                    suggestion: Some("Add ':' at end of line".to_string()),
                });
            }

            // Check for 'except' without specific exception
            if line.contains("except:") {
                issues.push(CodeIssue {
                    issue_type: IssueType::SecurityConcern,
                    line: line_num + 1,
                    column: None,
                    message: "Bare 'except' catches all exceptions".to_string(),
                    suggestion: Some("Specify exception type: except ValueError:".to_string()),
                });
            }
        }

        Ok(issues)
    }

    fn analyze_rust(code: &ParsedCode) -> CodingResult<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        for (line_num, line) in code.lines.iter().enumerate() {
            // Check for unwrap() usage
            if line.contains(".unwrap()") {
                issues.push(CodeIssue {
                    issue_type: IssueType::PotentialBug,
                    line: line_num + 1,
                    column: None,
                    message: "unwrap() may panic".to_string(),
                    suggestion: Some("Use ? operator or match instead".to_string()),
                });
            }

            // Check for clone() in loops
            if line.contains(".clone()") && lines_in_loop(code, line_num) {
                issues.push(CodeIssue {
                    issue_type: IssueType::PerformanceWarning,
                    line: line_num + 1,
                    column: None,
                    message: "Cloning in loop may impact performance".to_string(),
                    suggestion: Some("Consider using references or iterators".to_string()),
                });
            }
        }

        Ok(issues)
    }

    fn analyze_javascript(code: &ParsedCode) -> CodingResult<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        for (line_num, line) in code.lines.iter().enumerate() {
            // Check for var usage
            if line.trim().starts_with("var ") {
                issues.push(CodeIssue {
                    issue_type: IssueType::StyleIssue,
                    line: line_num + 1,
                    column: None,
                    message: "Use 'const' or 'let' instead of 'var'".to_string(),
                    suggestion: Some("Replace 'var' with 'const' or 'let'".to_string()),
                });
            }

            // Check for == instead of ===
            if line.contains("==") && !line.contains("===") {
                issues.push(CodeIssue {
                    issue_type: IssueType::StyleIssue,
                    line: line_num + 1,
                    column: None,
                    message: "Use strict equality ===".to_string(),
                    suggestion: Some("Replace '==' with '==='".to_string()),
                });
            }
        }

        Ok(issues)
    }

    fn check_trailing_whitespace(code: &ParsedCode) -> CodingResult<Vec<CodeIssue>> {
        let mut issues = Vec::new();

        for (line_num, line) in code.lines.iter().enumerate() {
            if line.ends_with(' ') || line.ends_with('\t') {
                issues.push(CodeIssue {
                    issue_type: IssueType::StyleIssue,
                    line: line_num + 1,
                    column: Some(line.len()),
                    message: "Trailing whitespace".to_string(),
                    suggestion: Some("Remove trailing spaces/tabs".to_string()),
                });
            }
        }

        Ok(issues)
    }

    fn check_long_lines(code: &ParsedCode) -> CodingResult<Vec<CodeIssue>> {
        let mut issues = Vec::new();
        const MAX_LINE_LENGTH: usize = 120;

        for (line_num, line) in code.lines.iter().enumerate() {
            if line.len() > MAX_LINE_LENGTH {
                issues.push(CodeIssue {
                    issue_type: IssueType::StyleIssue,
                    line: line_num + 1,
                    column: None,
                    message: format!("Line too long ({} > {} chars)", line.len(), MAX_LINE_LENGTH),
                    suggestion: Some("Break into multiple lines".to_string()),
                });
            }
        }

        Ok(issues)
    }
}

fn lines_in_loop(code: &ParsedCode, line_num: usize) -> bool {
    let mut in_loop = false;
    for i in 0..=line_num {
        let line = &code.lines[i];
        if line.contains("for ") || line.contains("while ") {
            in_loop = true;
        }
        if in_loop && (line.trim() == "}" || line.trim().ends_with("}")) {
            in_loop = false;
        }
    }
    in_loop
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::CodeParser;

    #[test]
    fn test_analyze_python_missing_colon() {
        let code = CodeParser::parse("test.py", "if x > 5\n    print(x)").unwrap();
        let issues = CodeAnalyzer::analyze(&code).unwrap();

        assert!(issues.iter().any(|i| i.issue_type == IssueType::SyntaxError));
    }

    #[test]
    fn test_analyze_rust_unwrap() {
        let code = CodeParser::parse("main.rs", "let x = result.unwrap();").unwrap();
        let issues = CodeAnalyzer::analyze(&code).unwrap();

        assert!(issues.iter().any(|i| i.issue_type == IssueType::PotentialBug));
    }

    #[test]
    fn test_trailing_whitespace() {
        let code = CodeParser::parse("test.txt", "hello world   \nfoo").unwrap();
        let issues = CodeAnalyzer::analyze(&code).unwrap();

        assert!(issues.iter().any(|i| i.issue_type == IssueType::StyleIssue
            && i.message.contains("Trailing")));
    }

    #[test]
    fn test_long_lines() {
        let long_line = "a".repeat(150);
        let code = CodeParser::parse("test.txt", &long_line).unwrap();
        let issues = CodeAnalyzer::analyze(&code).unwrap();

        assert!(issues.iter().any(|i| i.message.contains("Line too long")));
    }
}
