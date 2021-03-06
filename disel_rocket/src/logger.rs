use std::env;
use std::fs;
use log::{debug, error, info, trace, warn};

pub fn init() -> Result<(), fern::InitError> {
    // 環境変数から`LOG_LEVEL`を読み取り、それが`INFO`であればtrueを返す
    let log_level = env::var("LOG_LEVEL").unwrap_or("INFO".into());
    // `Result<String, VarError>`から`Result<LevelFilter, ParseLevelError>`へ変換し、`log::LevelFilter::Info`ならtrueを返す
    let log_level = log_level
        .parse::<log::LevelFilter>()
        .unwrap_or(log::LevelFilter::Info);

    // 
    let mut builder = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stderr());

        let log_file = env::var("LOG_FILE").ok();
        if let Some(log_file) = log_file {
            let log_file = fs::File::create(log_file)?;
            builder = builder.chain(log_file);
        }

        builder.apply()?;

        trace!("TRACE output enabled");
        debug!("DEBUG output enabled");
        info!("INFO output enabled");
        warn!("WARN output enabled");
        error!("ERROR output enabled");

    Ok(())
}