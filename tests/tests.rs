use asset_scanner;

#[test]
fn run_search() -> asset_scanner::error::Result<()> {
    std::env::set_var("RUST_LOG", "asset_scanner::ble=info,error");
    env_logger::init();

    let context = asset_scanner::ble::Context::new();
    let central = asset_scanner::ble::CentralManager::new(&context);
    central.run()
}
