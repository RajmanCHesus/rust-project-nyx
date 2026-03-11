pub mod audio;
pub mod image;

use crate::error::NyxResult;

/// Generic renderer trait
pub trait Renderer<I> {
    /// Render input to file at specified path
    fn render(&self, input: I, output_path: &str) -> NyxResult<()>;
}
