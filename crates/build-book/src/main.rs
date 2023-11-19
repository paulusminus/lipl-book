use mdbook::MDBook;
use mdbook::errors::Error;

fn main() -> Result<(), Error> {
    let mdbook = MDBook::load(".")?;
    mdbook.build()
}
