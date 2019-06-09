use super::error::*;
use super::split;
use super::Position;

pub struct Comment;

impl Comment {
    pub(super) fn lex<'i>(input: &'i str, pos: &mut Position) -> Result<&'i str> {
        let mut line = vec![];
        let mut block = vec![];

        if input.starts_with("//") {
            pos.col += 2;
            line.push(2);
        } else if input.starts_with("/*") {
            pos.col += 2;
            block.push(2);
        } else {
            return Err(Error::not_handled());
        }

        let mut i = 2;
        loop {
            if line.is_empty() && block.is_empty() {
                break;
            }

            if input.is_empty() {
                return Err(Error::not_handled());
            }

            if split(input, i).starts_with("/*") {
                i += 2;
                pos.col += 2;
                block.push(i);
                continue;
            }

            if split(input, i).starts_with("*/") {
                i += 2;
                pos.col += 2;
                block.pop();
                continue;
            }

            if split(input, i).starts_with('\n') {
                i += 1;
                pos.line += 1;
                pos.col = 0;
                line.pop();
                continue;
            }

            i += 1;
        }

        Ok(split(input, i))
    }
}
