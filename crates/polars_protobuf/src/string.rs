/// Convert a CamelCase or PascalCase string to snake_case.
/// Example: "UserLoyalty" -> "user_loyalty"
pub fn to_snake_case(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, c) in chars.iter().enumerate() {
        if c.is_uppercase() {
            if i > 0 && chars[i - 1].is_lowercase() {
                result.push('_');
            }
            result.push_str(&c.to_lowercase().to_string());
        } else {
            result.push(*c);
        }
    }
    result
}

/// Add indentation to each line of a multiline string.
/// Empty lines remain empty (no indentation added).
///
/// # Arguments
///
/// * `text` - The multiline string to indent
/// * `indent` - The indentation string to prepend to each non-empty line
///
/// # Example
///
/// ```
/// let text = "line1\n\nline2";
/// let indented = indent_lines(text, "    ");
/// assert_eq!(indented, "    line1\n\n    line2");
/// ```
pub fn indent_lines(text: &str, indent: &str) -> String {
    text.lines()
        .map(|line| {
            if line.is_empty() {
                String::new()
            } else {
                format!("{}{}", indent, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_snake_case_basic() {
        assert_eq!(to_snake_case("UserLoyalty"), "user_loyalty");
    }

    #[test]
    fn test_to_snake_case_pascal_case() {
        assert_eq!(to_snake_case("PascalCase"), "pascal_case");
    }

    #[test]
    fn test_to_snake_case_camel_case() {
        assert_eq!(to_snake_case("camelCase"), "camel_case");
    }

    #[test]
    fn test_to_snake_case_single_word() {
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("user"), "user");
    }

    #[test]
    fn test_to_snake_case_multiple_uppercase() {
        // Consecutive uppercase letters don't get separated
        assert_eq!(to_snake_case("XMLParser"), "xmlparser");
        assert_eq!(to_snake_case("HTTPServer"), "httpserver");
    }

    #[test]
    fn test_to_snake_case_empty_string() {
        assert_eq!(to_snake_case(""), "");
    }

    #[test]
    fn test_to_snake_case_all_uppercase() {
        // All uppercase becomes all lowercase without underscores
        assert_eq!(to_snake_case("ABC"), "abc");
    }

    #[test]
    fn test_to_snake_case_all_lowercase() {
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn test_to_snake_case_with_numbers() {
        // Numbers don't trigger underscore insertion
        assert_eq!(to_snake_case("User2Name"), "user2name");
    }

    #[test]
    fn test_indent_lines_basic() {
        let text = "line1\nline2\nline3";
        let result = indent_lines(text, "    ");
        assert_eq!(result, "    line1\n    line2\n    line3");
    }

    #[test]
    fn test_indent_lines_with_empty_lines() {
        let text = "line1\n\nline2";
        let result = indent_lines(text, "    ");
        assert_eq!(result, "    line1\n\n    line2");
    }

    #[test]
    fn test_indent_lines_multiple_empty_lines() {
        let text = "line1\n\n\nline2";
        let result = indent_lines(text, "    ");
        assert_eq!(result, "    line1\n\n\n    line2");
    }

    #[test]
    fn test_indent_lines_empty_string() {
        let text = "";
        let result = indent_lines(text, "    ");
        assert_eq!(result, "");
    }

    #[test]
    fn test_indent_lines_single_line() {
        let text = "single line";
        let result = indent_lines(text, "  ");
        assert_eq!(result, "  single line");
    }

    #[test]
    fn test_indent_lines_only_empty_lines() {
        // lines() doesn't include trailing empty lines, so "\n\n\n" becomes 3 empty lines
        // which when joined with "\n" becomes "\n\n" (2 newlines between 3 empty strings)
        let text = "\n\n\n";
        let result = indent_lines(text, "    ");
        assert_eq!(result, "\n\n");
    }

    #[test]
    fn test_indent_lines_custom_indent() {
        let text = "line1\nline2";
        let result = indent_lines(text, ">>>");
        assert_eq!(result, ">>>line1\n>>>line2");
    }

    #[test]
    fn test_indent_lines_tab_indent() {
        let text = "line1\nline2";
        let result = indent_lines(text, "\t");
        assert_eq!(result, "\tline1\n\tline2");
    }

    #[test]
    fn test_indent_lines_trailing_newline_handling() {
        // Note: lines() doesn't include trailing newline, so we test the actual behavior
        let text = "line1\nline2\n";
        let result = indent_lines(text, "  ");
        // lines() removes trailing empty line, so result won't have trailing newline
        assert_eq!(result, "  line1\n  line2");
    }

    #[test]
    fn test_indent_lines_mixed_content() {
        let text = "first\n\nsecond\nthird\n\nfourth";
        let result = indent_lines(text, "  ");
        assert_eq!(result, "  first\n\n  second\n  third\n\n  fourth");
    }
}
