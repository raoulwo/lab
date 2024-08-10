// So called *extension traits* are used to add methods
// to existing types.
trait IsEmoji {
    fn is_emoji(&self) -> bool;
}

impl IsEmoji for char {
    fn is_emoji(&self) -> bool {
        // ...

        false
    }
}

use std::io::{self, Write};

struct HtmlDocument {}

trait WriteHtml {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()>;
}

// You can create generic impl blocks in order add extension
// traits to multiple types at once ...
impl<W: Write> WriteHtml for W {
    fn write_html(&mut self, html: &HtmlDocument) -> io::Result<()> {
        Ok(())
    }
}

fn main() {}
