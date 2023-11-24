use std::path::PathBuf;

use log::{error, info};
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

    let md = mdbook::MDBook::load("epub")?;

    // let destination = md.build_dir_for("epub");
    let destination = PathBuf::from("book/epub");

    info!("Epub book destination = {:#?}", destination.display());
    info!("Epub book config = {:#?}", md.config);

    let context = RenderContext::new(md.root, md.book, md.config, destination);
    match mdbook_epub::generate(&context) {
        Ok(_) => {
            info!(
                "Epub generated in directory {}",
                context.destination.display()
            );
        }
        Err(error) => {
            error!("Failed to generated epub {}", error);
        }
    }

    Ok(())

    // match serde_json::from_reader(std::io::stdin()).map_err(|_| mdbook_epub::Error::RenderContext) {
    //     Ok(context) => {
    //         match mdbook_epub::generate(&context) {
    //             Ok(_) => {
    //                 info!("Epub generated in directory {}", context.destination.display());
    //             }
    //             Err(error) => {
    //                 error!("Failed to generated epub {}", error);
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         error!("Failed to get context from stdin {}", error);
    //     }
    // }
    // Ok(())
}
