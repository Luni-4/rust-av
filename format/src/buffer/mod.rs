mod accreader;

pub use self::accreader::AccReader;

use std::io::{BufRead, Seek};

/// Used to interact with a buffer.
pub trait Buffered: BufRead + Seek + BufferedClone + Send + Sync {
    /// Returns the data contained in a buffer as a sequence of bytes.
    fn data(&self) -> &[u8];
    /// Increases the size of a buffer.
    fn grow(&mut self, len: usize);
}

/// Used to clone a buffer.
pub trait BufferedClone {
    /// Clones a boxed buffer.
    fn clone_box(&self) -> Box<dyn Buffered>;
}

impl<T> BufferedClone for T
where
    T: 'static + Buffered + Clone,
{
    fn clone_box(&self) -> Box<dyn Buffered> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Buffered> {
    fn clone(&self) -> Box<dyn Buffered> {
        self.clone_box()
    }
}
