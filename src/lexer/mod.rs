mod error;
mod token;

pub use error::*;
pub use token::*;

pub fn lex<'i>(mut input: &'i str) -> Result<Vec<Token<'i>>> {
    let mut tokens = vec![];

    let mut pos = Position::default();
    loop {
        if input.starts_with(' ') {
            pos.col += 1;
            input = split(input, 1);
        } else if input.starts_with('\t') {
            pos.col += 4;
            input = split(input, 1);
        } else if input.starts_with('\n') {
            pos.line += 1;
            pos.col = 0;
            input = split(input, 1);
        } else {
            let (input_, token) = Token::lex(input, &mut pos)?;
            input = input_;

            if let Some(token) = token {
                tokens.push(token);
                if input.is_empty() {
                    break;
                }
            }
        }
    }

    Ok(tokens)
}

fn split(data: &str, at: usize) -> &str {
    if at >= data.len() {
        ""
    } else {
        &data[at..]
    }
}
