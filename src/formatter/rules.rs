use itertools::Itertools;

use crate::{ast::AstNode, lexer::Token, source::SourceRange};

use super::{
    context::TriviaFormatContext,
    ir::{self, user_placed_new_line},
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

/// Indents the specified block by one level on a new line
pub(crate) fn indent(doc: Format) -> Format {
    ir::indent(ir::concat([new_line(), doc]))
}

/// Concatenates a list of formatted text, separated by a specified separator
pub(crate) fn separated_list(
    documents: impl IntoIterator<Item = Format>,
    separator: Format,
) -> Format {
    ir::concat(itertools::intersperse(documents, separator))
}

/// Concatenates a list of formatted text
pub(crate) fn list(documents: impl IntoIterator<Item = Format>) -> Format {
    ir::concat(documents)
}

/// Concatenates two formated text
pub(crate) fn pair(first: Format, second: Format) -> Format {
    list([first, second])
}

/// Do nothing
pub(crate) fn nil() -> Format {
    ir::nil()
}

pub(crate) fn format_leading_trivia(
    range: SourceRange,
    source: &Source,
    trivia_context: &mut TriviaFormatContext,
) -> Format {
    let trivia = trivia_context.leading_trivia(range);
    list(
        trivia
            .into_iter()
            .map(|trivia_token| format_trivia(trivia_token, source)),
    )
}

pub(crate) fn format_trailing_trivia(
    range: SourceRange,
    source: &Source,
    trivia_context: &mut TriviaFormatContext,
) -> Format {
    let trivia = trivia_context.trailing_trivia(range);
    list(
        trivia
            .into_iter()
            .map(|trivia_token| format_trivia(trivia_token, source)),
    )
}

fn format_trivia(token: Token, source: &Source) -> Format {
    let comment_text = text_from_range(token.range, source);
    if token.is_single_line_comment() {
        format_single_line_comment(comment_text)
    } else if token.is_block_comment() {
        format_block_comment(comment_text)
    } else if token.is_newline() {
        user_placed_new_line()
    } else {
        nil()
    }
}

fn format_single_line_comment(comment: &str) -> Format {
    pair(ir::text(comment), new_line())
}

fn format_block_comment(comment: &str) -> Format {
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
    // TODO: properly handling spacing and new line
    if multiline {
        formatted_comment_lines.push(new_line());
    } else {
        formatted_comment_lines.push(space());
    }
    list(formatted_comment_lines)
}

fn text_from_range<'src>(range: SourceRange, source: &'src Source) -> &'src str {
    std::str::from_utf8(&source[range]).expect("Node must be a valid utf8 string")
}
