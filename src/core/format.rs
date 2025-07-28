use std::io::{self, Write};

use termcolor::{Color, ColorSpec, WriteColor};

pub trait Format {
    fn format<W>(&self, stdout: &mut W, indent: usize, level: usize) -> io::Result<()>
    where
        W: Write + WriteColor;
}

