use super::error::*;
use super::split;
use super::Position;

pub struct Comment;

impl Comment {
    pub(super) fn lex<'i>(input: &'i str, pos: &mut Position) -> Result<&'i str> {
        let mut tpos = *pos;
        let mut line = vec![];
        let mut block = vec![];

        if input.starts_with("//") {
            tpos.col += 2;
            line.push(2);
        } else if input.starts_with("/*") {
            tpos.col += 2;
            block.push(2);
        } else {
            return Err(Error::not_handled(*pos));
        }

        let mut i = 2;
        loop {
            if line.is_empty() && block.is_empty() {
                break;
            }

            if input.is_empty() {
                return Err(Error::not_handled(*pos));
            }

            if split(input, i).starts_with("/*") {
                i += 2;
                tpos.col += 2;
                block.push(i);
                continue;
            }

            if split(input, i).starts_with("*/") {
                i += 2;
                tpos.col += 2;
                block.pop();
                continue;
            }

            if split(input, i).starts_with('\n') {
                i += 1;
                tpos.line += 1;
                tpos.col = 0;
                line.pop();
                continue;
            }

            i += 1;
        }

        *pos = tpos;
        Ok(split(input, i))
    }
}
