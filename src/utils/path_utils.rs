pub fn canonicalize_path(path: &str) -> anyhow::Result<String> {
    let home_dir = std::env::var("HOME").unwrap_or("".to_string());
    let home_expanded = path.replace("~", &home_dir);
    Ok(std::fs::canonicalize(&home_expanded)
        .map_err(|_e| anyhow::anyhow!("no such path: {}", path))?
        .to_str()
        .expect("no such path")
        .to_string())
}
