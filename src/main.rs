mod ble;
mod error;

fn main() -> error::Result<()> {
    let matches = clap::App::new("BLE Asset Scanner")
        .version("0.1")
        .author("Jacob Trueb <jtrueb@northwestern.edu")
        .about("Search for asset tags and record contact")
        .arg(
            clap::Arg::with_name("endpoint")
                .help("The address of the asset tracking API")
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    env_logger::init();
    log::trace!("CLAP: {:#?}", matches);

    let api_endpoint = matches.value_of("endpoint");
    log::info!("Using API: {:?}", api_endpoint);

    let context = ble::Context::new();
    let central = ble::CentralManager::new(&context);
    central.run()
}
