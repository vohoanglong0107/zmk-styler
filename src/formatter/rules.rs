use itertools::Itertools;

use crate::{ast::AstNode, lexer::Token, source::SourceRange};

use super::{
    ir::{self},
    Format, Source,
};

/// Write node's content as it is.
pub(crate) fn text<T: AstNode>(node: &T, source: &Source) -> Format {
    let node_text =
        std::str::from_utf8(&source[node.range()]).expect("Node must be a valid utf8 string");
    ir::text(node_text)
}

/// A normal text.
pub(crate) fn tag(text: impl ToString) -> Format {
    ir::text(text)
}

/// New line with indentation at current ident level
pub(crate) fn new_line() -> Format {
    ir::new_line()
}

/// New line without indent
pub(crate) fn empty_new_line() -> Format {
    tag("\n")
}

pub(crate) fn space() -> Format {
    tag(" ")
}

/// Concatenates a list of formatted text, separated by a specified separator
pub(crate) fn separated_list(
    formats: impl IntoIterator<Item = Format>,
    separator: Format,
) -> Format {
    ir::concat(itertools::intersperse(formats, separator))
}

/// Concatenates a list of formatted text
pub(crate) fn list(formats: impl IntoIterator<Item = Format>) -> Format {
    ir::concat(formats)
}

/// Concatenates two formated text
pub(crate) fn pair(first: Format, second: Format) -> Format {
    list([first, second])
}

/// Group all formatted text on a single line
/// Or break them to multiple lines
pub(crate) fn group(formats: impl IntoIterator<Item = Format>) -> Format {
    ir::group(formats)
}

/// Do nothing
pub(crate) fn nil() -> Format {
    ir::nil()
}

pub(crate) fn format_leading_trivia(trivia: Vec<Token>, source: &Source) -> Format {
    list(trivia.into_iter().map(|token| {
        let comment_text = text_from_range(token.range, source);
        if token.is_single_line_comment() {
            format_single_line_comment(comment_text)
        } else if token.is_block_comment() {
            let (format, multiline) = format_block_comment(comment_text);
            if multiline {
                pair(format, new_line())
            } else {
                pair(format, space())
            }
        } else if token.is_newline() {
            ir::text_break(0, ir::TextBreakKind::Discretion)
        } else {
            nil()
        }
    }))
}

pub(crate) fn format_trailing_trivia(trivia: Vec<Token>, source: &Source) -> Format {
    list(trivia.into_iter().map(|token| {
        let comment_text = text_from_range(token.range, source);
        if token.is_single_line_comment() {
            pair(space(), format_single_line_comment(comment_text))
        } else if token.is_block_comment() {
            let (format, multiline) = format_block_comment(comment_text);
            if multiline {
                list([space(), format, new_line()])
            } else {
                pair(space(), format)
            }
        } else if token.is_newline() {
            ir::text_break(0, ir::TextBreakKind::Discretion)
        } else {
            nil()
        }
    }))
}

fn format_single_line_comment(comment: &str) -> Format {
    pair(ir::text(comment), new_line())
}

fn format_block_comment(comment: &str) -> (Format, bool) {
    let comment_lines = comment.lines().collect_vec();
    let multiline = comment_lines.len() > 1;
    let mut formatted_comment_lines = Vec::new();
    for line in comment_lines {
        if line.starts_with("/*") {
            formatted_comment_lines.push(ir::text(line));
        } else {
            formatted_comment_lines.push(new_line());
            // Remove indentation from inner comment lines,
            // but leave one space before the * to form
            // /*
            //  */
            // pattern
            let line = String::from(" ") + line.trim_start();
            formatted_comment_lines.push(ir::text(line));
        }
    }
    (list(formatted_comment_lines), multiline)
}

fn text_from_range<'src>(range: SourceRange, source: &'src Source) -> &'src str {
    std::str::from_utf8(&source[range]).expect("Node must be a valid utf8 string")
}
