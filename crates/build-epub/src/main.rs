use log::{debug, info, error};
use mdbook::{errors::Error, renderer::RenderContext};

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
    env_logger::init();

    let md = mdbook::MDBook::load(".")?;
    let destination = md.build_dir_for("epub");

    debug!("Epub book destination = {:#?}", destination.display());
    debug!("Epub book config = {:#?}", md.config);

    let context = RenderContext::new(md.root, md.book, md.config, destination);
    match mdbook_epub::generate(&context) {
        Ok(_) => {
            info!("Epub generated in directory {}", context.destination.display());
        }
        Err(error) => {
            error!("Failed to generated epub {}", error);
        }
    }
    Ok(())
}
