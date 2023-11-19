use mdbook::MDBook;
use mdbook::errors::Error;

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
    let mdbook = MDBook::load(".")?;
    mdbook.build()
}
