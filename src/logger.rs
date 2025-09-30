use chrono::Local;
use log::{error, info, warn};

pub enum Log {
    INFO(String),
    ERROR(String),
    WARN(String),
}

impl Log {
    fn emit(self) {
        match self {
            Log::INFO(val) => {
                info!("{}", val);
            }
            Log::WARN(val) => {
                warn!("{}", val);
            }
            Log::ERROR(val) => {
                error!("{}", val);
            }
        }
    }
}
pub fn setup_logger() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug) // sett minst logging-nivå
        .chain(std::io::stdout()) // logg til terminal
        .chain(fern::log_file("game.log")?) // logg til fil
        .apply()?;
    Ok(())
}

pub fn log(entry: Log) {
    entry.emit();
}
