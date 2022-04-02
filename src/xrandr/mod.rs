use std::process::Command;

use nom::character::complete::{char, u16};
use nom::combinator::all_consuming;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug)]
pub struct Screen {
    pub name: String,
    pub dim: (u16, u16),
    pub offset: (u16, u16),
}

impl Screen {
    pub fn list() -> std::io::Result<Vec<Screen>> {
        let out = Command::new("xrandr").output()?;

        if !out.status.success() {
            return Ok(vec![]);
        }

        let buf = match String::from_utf8(out.stdout) {
            Ok(b) => b,
            Err(_) => return Ok(vec![]),
        };

        let lines = buf.split('\n');
        let mut screens = Vec::new();

        for line in lines {
            if !line.starts_with(char::is_whitespace) {
                let mut parts = line.split(char::is_whitespace);
                let first = parts.next();
                let second = parts.next();
                let third = parts.next();
                let forth = parts.next();

                let (name, dim) = match (first, second, third, forth) {
                    (Some(name), Some("connected"), Some("primary"), Some(dim)) => (name, dim),
                    (Some(name), Some("connected"), Some(dim), _) => (name, dim),
                    _ => continue,
                };

                let (dim, offset) = match all_consuming(parse_dim)(dim) {
                    Ok((_, dims)) => dims,
                    Err(_) => continue,
                };

                let name = name.to_string();

                screens.push(Screen { name, dim, offset });
            }
        }

        Ok(screens)
    }
}

fn parse_dim(input: &str) -> IResult<&str, ((u16, u16), (u16, u16))> {
    let mut parser = tuple((u16, char('x'), u16, char('+'), u16, char('+'), u16));

    let (buf, (w, _, h, _, x, _, y)) = parser(input)?;

    Ok((buf, ((w, h), (x, y))))
}
