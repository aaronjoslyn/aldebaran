use anyhow::Result;
use wasm_bindgen_cli_support::Bindgen;

async fn build_wasm() -> Result<()> {
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
    let mut timestamp = std::time::SystemTime::UNIX_EPOCH;
    loop {
        let metadata = tokio::fs::metadata("./app/src/lib.rs").await?;
        let last_modified = metadata.modified()?;
        if last_modified > timestamp {
            build_wasm().await?;
            timestamp = tokio::fs::metadata("./app/src/lib.rs").await?.modified()?;
        }
        tokio::time::delay_for(tokio::time::Duration::from_millis(5000)).await;
    }
}
