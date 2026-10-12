fn run() -> Result<(), String> {
    let a: Vec<_> = std::env::args().collect();
    if a.len() != 3 {
        return Err("usage: production-bottleneck-simulator CONFIG REPORT_JSON".into());
    }
    if std::fs::metadata(&a[1]).map_err(|e| e.to_string())?.len() > 2_000_000 {
        return Err("input exceeds 2 MB".into());
    }
    let c = serde_json::from_str(&std::fs::read_to_string(&a[1]).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let r = production_bottleneck_simulator::simulate(c)?;
    std::fs::write(&a[2], serde_json::to_string_pretty(&r).unwrap()).map_err(|e| e.to_string())
}
fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(2)
    }
}
