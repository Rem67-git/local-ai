use log::LevelFilter;

pub fn setup_logging(verbose: bool) {
    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    env_logger::Builder::from_default_env()
        .filter_level(level)
        .format_timestamp_millis()
        .init();
}
