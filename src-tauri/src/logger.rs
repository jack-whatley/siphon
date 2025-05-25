use crate::utils::paths;
use eyre::{Result, WrapErr};
use serde::{Serialize, Serializer};
use std::fmt::Display;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter};
use tracing::metadata::LevelFilter;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Layer, Registry};

#[derive(Debug, Clone, Serialize)]
pub enum UIEventLevel {
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize)]
pub struct UIEvent {
    level: UIEventLevel,
    message: String,
}

impl UIEvent {
    pub fn downloader_missing() -> Self {
        Self {
            level: UIEventLevel::Warning,
            message: "Youtube downloader is missing, click the download button to install."
                .to_string(),
        }
    }

    pub fn downloader_update_available() -> Self {
        Self {
            level: UIEventLevel::Info,
            message:
                "Youtube downloader has an update available, click the download button to install."
                    .to_string(),
        }
    }

    pub fn downloader_installed() -> Self {
        Self {
            level: UIEventLevel::Info,
            message: "Youtube downloader has installed/updated successfully.".to_string(),
        }
    }

    pub fn downloaded_video(path: &str) -> Self {
        Self {
            level: UIEventLevel::Info,
            message: format!("Video downloaded successfully to: '{}'", path),
        }
    }
}

pub fn send_ui_event(event: UIEvent, app: &AppHandle) {
    app.emit("notification", event).unwrap_or_else(|err| {
        tracing::error!("Failed to send notification to ui: {}", err);
    });
}

pub const LOG_NAME: &str = "application.log";

pub fn logger_path() -> PathBuf {
    paths::default_app_dir().join(LOG_NAME)
}

/// Initialise the logging for Siphon.
pub fn init() -> Result<()> {
    let log_path = logger_path();

    fs::create_dir_all(paths::default_app_dir())?;
    let log_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(log_path)?;

    let subscriber = Registry::default()
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(true)
                .with_filter(LevelFilter::from_level(Level::INFO)),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .compact()
                .with_ansi(false)
                .with_writer(log_file)
                // .with_span_events(FmtSpan::FULL)
                .with_filter(LevelFilter::from_level(Level::INFO)),
        );

    tracing::subscriber::set_global_default(subscriber)
        .context("Failed to register tracing subscriber")?;

    Ok(())
}

#[tauri::command]
pub fn open_log() -> std::result::Result<(), String> {
    let log_path = logger_path();

    tauri_plugin_opener::open_path(&log_path, None::<&str>)
        .map_err(|err| format!("Failed to open log file: {:#?}", err))?;

    Ok(())
}

#[tauri::command]
pub fn log_error(msg: String) {
    tracing::error!("{}", msg);
}

#[derive(Debug)]
pub struct CommandError(eyre::Error);

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#}", self.0)
    }
}

impl Serialize for CommandError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<T> From<T> for CommandError
where
    T: Into<eyre::Report>,
{
    fn from(t: T) -> Self {
        let report = t.into();

        tracing::error!("Command Returning Error:\n{:#?}", report);

        Self(report)
    }
}

pub type SiphonResult<T> = std::result::Result<T, CommandError>;
