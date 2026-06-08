use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    reader: R,
    read_bytes: usize,
    read_count: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(reader: R) -> ReadStats<R> {
        ReadStats {
            reader,
            read_bytes: 0,
            read_count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.reader
    }

    pub fn bytes_through(&self) -> usize {
        self.read_bytes
    }

    pub fn reads(&self) -> usize {
        self.read_count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let res = self.reader.read(buf);
        self.read_count += 1;
        if let Ok(s) = res {
            self.read_bytes += s;
        }

        res
    }
}

pub struct WriteStats<W> {
    writer: W,
    write_bytes: usize,
    write_count: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(writer: W) -> WriteStats<W> {
        WriteStats {
            writer,
            write_bytes: 0,
            write_count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.writer
    }

    pub fn bytes_through(&self) -> usize {
        self.write_bytes
    }

    pub fn writes(&self) -> usize {
        self.write_count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let res = self.writer.write(buf);
        self.write_count += 1;
        if let Ok(s) = res {
            self.write_bytes += s;
        }
        res
    }

    fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }
}
