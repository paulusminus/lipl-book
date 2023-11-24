use std::fs::OpenOptions;
use std::path::PathBuf;

use mdbook::errors::Error;
use mdbook::MDBook;

/// Change to workspace root.
///
/// Assumed this xtask is located in `[WORKSPACE]/crates/xtask-build-man`.
fn cwd_to_workspace_root() -> std::io::Result<()> {
    let pkg_root = std::env!("CARGO_MANIFEST_DIR");
    let ws_root = format!("{pkg_root}/../..");
    std::env::set_current_dir(ws_root)
}

fn main() -> Result<(), Error> {
    cwd_to_workspace_root()?;
    let mut mdbook = MDBook::load("html")?;
    mdbook.config.build.build_dir = PathBuf::from("../book");
    mdbook.build()?;

    let out_file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("lipl-book.tar")?;
    let mut builder = tar::Builder::new(out_file);
    builder.follow_symlinks(false);

    std::env::set_current_dir("book")?;
    let book_dir = walkdir::WalkDir::new(".");
    for dir_entry in book_dir.into_iter().filter_map(|f| f.ok()) {
        println!("Archiving: {}", dir_entry.path().to_string_lossy());
        builder.append_path(dir_entry.path())?;
    }

    builder.finish()?;

    Ok(())
}
