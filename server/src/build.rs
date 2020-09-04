use super::watch;
use anyhow::Result;
use futures::StreamExt;
use wasm_bindgen_cli_support::Bindgen;

fn build_wasm() -> Result<()> {
    let status = std::process::Command::new("cargo")
        .current_dir("./app")
        .args(vec!["build"])
        .status()?;
    let mut b = Bindgen::new();
    b.input_path("./target/wasm32-unknown-unknown/debug/app.wasm")
        .web(true)?
        .keep_debug(true)
        .debug(true)
        .typescript(false)
        .generate("./public")?;
    if status.success() {
        println!("Built new wasm.");
    }
    Ok(())
}

pub async fn watch_wasm() -> Result<()> {
    let mut watcher = watch::FolderWatcher::new();
    watcher.watch();
    while let Some(_) = watcher.next().await {
        println!("Building..");
        build_wasm().expect("Failed to build new wasm.");
    }
    Ok(())
}
