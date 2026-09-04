use crate::error::{CodingError, CodingResult};
use crate::parser::ParsedCode;

pub struct CodeModifier;

#[derive(Debug, Clone)]
pub struct CodeEdit {
    pub start_line: usize,
    pub end_line: usize,
    pub new_content: String,
}

impl CodeModifier {
    pub fn apply_edit(code: &mut ParsedCode, edit: &CodeEdit) -> CodingResult<()> {
        if edit.start_line == 0 || edit.start_line > code.line_count {
            return Err(CodingError::ModificationFailed(
                "Invalid start line".to_string(),
            ));
        }

        if edit.end_line < edit.start_line || edit.end_line > code.line_count {
            return Err(CodingError::ModificationFailed(
                "Invalid end line".to_string(),
            ));
        }

        let new_lines: Vec<String> = edit.new_content.lines().map(|l| l.to_string()).collect();

        // Replace lines in the vector
        let mut result_lines = code.lines[..edit.start_line - 1].to_vec();
        result_lines.extend(new_lines);
        if edit.end_line < code.line_count {
            result_lines.extend_from_slice(&code.lines[edit.end_line..]);
        }

        code.lines = result_lines;
        code.line_count = code.lines.len();
        code.content = code.lines.join("\n");

        Ok(())
    }

    pub fn apply_multiple_edits(code: &mut ParsedCode, edits: &[CodeEdit]) -> CodingResult<()> {
        // Sort edits in reverse line order to avoid index shifts
        let mut sorted_edits = edits.to_vec();
        sorted_edits.sort_by(|a, b| b.start_line.cmp(&a.start_line));

        for edit in sorted_edits {
            Self::apply_edit(code, &edit)?;
        }

        Ok(())
    }

    pub fn replace_line(code: &mut ParsedCode, line_num: usize, new_content: &str) -> CodingResult<()> {
        if line_num == 0 || line_num > code.line_count {
            return Err(CodingError::ModificationFailed(
                "Invalid line number".to_string(),
            ));
        }

        code.lines[line_num - 1] = new_content.to_string();
        code.content = code.lines.join("\n");

        Ok(())
    }

    pub fn insert_line(code: &mut ParsedCode, after_line: usize, content: &str) -> CodingResult<()> {
        if after_line > code.line_count {
            return Err(CodingError::ModificationFailed(
                "Invalid insert position".to_string(),
            ));
        }

        code.lines.insert(after_line, content.to_string());
        code.line_count = code.lines.len();
        code.content = code.lines.join("\n");

        Ok(())
    }

    pub fn delete_line(code: &mut ParsedCode, line_num: usize) -> CodingResult<()> {
        if line_num == 0 || line_num > code.line_count {
            return Err(CodingError::ModificationFailed(
                "Invalid line number".to_string(),
            ));
        }

        code.lines.remove(line_num - 1);
        code.line_count = code.lines.len();
        code.content = code.lines.join("\n");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::CodeParser;

    #[test]
    fn test_replace_line() {
        let mut code = CodeParser::parse("test.rs", "fn main() {\n    println!(\"old\");\n}").unwrap();
        CodeModifier::replace_line(&mut code, 2, "    println!(\"new\");").unwrap();

        assert_eq!(code.get_line(2), Some("    println!(\"new\");"));
    }

    #[test]
    fn test_insert_line() {
        let mut code = CodeParser::parse("test.rs", "fn main() {\n}").unwrap();
        CodeModifier::insert_line(&mut code, 1, "    let x = 5;").unwrap();

        assert_eq!(code.line_count, 3);
        assert_eq!(code.get_line(2), Some("    let x = 5;"));
    }

    #[test]
    fn test_delete_line() {
        let mut code = CodeParser::parse("test.rs", "line1\nline2\nline3").unwrap();
        CodeModifier::delete_line(&mut code, 2).unwrap();

        assert_eq!(code.line_count, 2);
        assert_eq!(code.get_line(2), Some("line3"));
    }

    #[test]
    fn test_apply_edit() {
        let mut code = CodeParser::parse("test.rs", "line1\nline2\nline3").unwrap();
        let edit = CodeEdit {
            start_line: 1,
            end_line: 2,
            new_content: "new_line1\nnew_line2".to_string(),
        };
        CodeModifier::apply_edit(&mut code, &edit).unwrap();

        assert_eq!(code.line_count, 3);
        assert_eq!(code.get_line(1), Some("new_line1"));
    }

    #[test]
    fn test_apply_multiple_edits() {
        let mut code = CodeParser::parse("test.rs", "a\nb\nc\nd\ne").unwrap();
        let edits = vec![
            CodeEdit {
                start_line: 5,
                end_line: 5,
                new_content: "e_modified".to_string(),
            },
            CodeEdit {
                start_line: 1,
                end_line: 1,
                new_content: "a_modified".to_string(),
            },
        ];
        CodeModifier::apply_multiple_edits(&mut code, &edits).unwrap();

        assert_eq!(code.get_line(1), Some("a_modified"));
        assert_eq!(code.get_line(5), Some("e_modified"));
    }
}
