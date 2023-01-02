use std::time::SystemTime;

use chrono::{DateTime, Utc};
use fern::colors::{Color, ColoredLevelConfig};

pub fn log_init(debug_enabled: bool) -> Result<(), log::SetLoggerError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .debug(Color::Blue)
        .error(Color::Red);

    let today: DateTime<Utc> = SystemTime::now().into();
    let today = today.format("%Y-%m-%d");
    let filename = format!("log-{}.log", today);

    let stdout_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    let file_dispatcher = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}:{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(filename).unwrap());

    fern::Dispatch::new()
        // Ignore non-interesting logs from other sources
        .level(log::LevelFilter::Warn)
        // Keep our logs
        .level_for(
            "chip8",
            if debug_enabled {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            },
        )
        .chain(stdout_dispatcher)
        .chain(file_dispatcher)
        .apply()?;
    Ok(())
}
