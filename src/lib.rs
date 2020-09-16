use std::fmt::Arguments;
use std::io::{Result, Write};
use std::time::Instant;

use bytesize::ByteSize;

pub struct Progress<W> {
    inner: W,
    ts: Instant,
    pos: u64,
    len: u64,
    clear: bool,
    buffer: Vec<u8>,
}

impl<W: Write> Progress<W> {
    pub fn new(inner: W, len: u64) -> Self {
        Self {
            inner,
            ts: Instant::now(),
            pos: 0,
            len,
            clear: false,
            buffer: Vec::with_capacity(256),
        }
    }

    #[inline]
    pub fn pos(&mut self, pos: u64) {
        self.pos = pos;
    }

    pub fn render(&mut self) -> Result<()> {
        self.buffer.clear();

        let (width, _) = termion::terminal_size()?;

        let elapsed = self.ts.elapsed().as_secs();
        let bytes_per_second = if elapsed > 0 {
            ByteSize(self.pos / elapsed)
        } else {
            ByteSize(0)
        };

        let percentage = self.pos as f32 / self.len as f32;

        if self.clear {
            write!(self.buffer, "{}", termion::cursor::Restore)?;
        }

        write!(self.buffer, "{}", termion::cursor::Save)?;

        let bar_width = width - 45;
        let bar_filled = (bar_width as f32 * percentage).ceil() as u16;

        for _ in 0..bar_filled {
            write!(self.buffer, "█")?;
        }
        for _ in bar_filled..bar_width {
            write!(self.buffer, "░")?;
        }

        write!(
            self.buffer,
            " {}/{} ({:.2}%) - {}",
            ByteSize(self.pos),
            ByteSize(self.len),
            percentage * 100.0,
            bytes_per_second,
        )?;

        self.inner.write_all(&self.buffer)?;

        self.flush()?;
        self.clear = true;

        Ok(())
    }

    #[inline]
    pub fn finish(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for Progress<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    #[inline]
    fn write_fmt(&mut self, fmt: Arguments) -> Result<()> {
        if self.clear {
            write!(
                self.inner,
                "{}{}",
                termion::clear::CurrentLine,
                termion::cursor::Restore
            )?;

            self.clear = false;
        }

        self.inner.write_fmt(fmt)
    }
}
