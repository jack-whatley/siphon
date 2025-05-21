use crate::downloader::install::VersionWrapper;
use crate::utils::paths;
use eyre::{ensure, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum PresetTypes {
    MP3,
    MP4,
    MKV,
}

impl PresetTypes {
    pub fn to_string(&self) -> String {
        match self {
            PresetTypes::MP3 => "mp3".to_string(),
            PresetTypes::MP4 => "mp4".to_string(),
            PresetTypes::MKV => "mkv".to_string(),
        }
    }

    pub fn all() -> Vec<String> {
        vec![
            PresetTypes::MP3.to_string(),
            PresetTypes::MP4.to_string(),
            PresetTypes::MKV.to_string(),
        ]
    }
}

pub struct CommandBuilder {
    path: PathBuf,
    args: Vec<String>,
    url: String,
}

impl CommandBuilder {
    pub fn new() -> Self {
        Self {
            path: paths::downloader_path(),
            args: vec![],
            url: "".to_string(),
        }
    }

    pub fn with_version_check(&mut self) -> &mut Self {
        self.args.push("--version".to_owned());

        self
    }

    pub fn with_ffmpeg_args(&mut self) -> &mut Self {
        self.args.push("--ffmpeg-location".to_owned());
        self.args
            .push(paths::ffmpeg_executable().display().to_string());

        self
    }

    pub fn with_url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    pub fn with_preset(&mut self, preset: PresetTypes) -> &mut Self {
        self.args.push("-t".to_owned());
        self.args.push(preset.to_string());

        self
    }

    pub fn with_download_path(&mut self, path: PathBuf) -> &mut Self {
        let string_path = path.display().to_string();

        self.args.push("--paths".to_owned());
        self.args.push(string_path);

        self
    }

    pub async fn run(&self) -> Result<String> {
        let mut command = Command::new(&self.path);

        for arg in &self.args {
            command.arg(arg.as_str());
        }

        command.arg(&self.url);

        let output = command.output().await?;
        ensure!(output.status.success(), "Failed to run yt-dlp command");

        let parsed_std_out = String::from_utf8(output.stdout)?.trim().to_owned();
        tracing::debug!("Captured downloader output: '{}'", &parsed_std_out);

        Ok(parsed_std_out)
    }
}

pub async fn get_version() -> Option<VersionWrapper> {
    let executable = paths::downloader_path();

    if !executable.exists() {
        return None;
    }

    let str_version = CommandBuilder::new().with_version_check().run().await;

    match str_version {
        Ok(str_version) => {
            let wrapper = VersionWrapper::parse(&str_version);

            match wrapper {
                Ok(wrapper) => Some(wrapper),
                Err(e) => {
                    tracing::error!("Failed to parse version from yt-dlp: {}", e);

                    None
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get version from yt-dlp: {}", e);

            None
        }
    }
}

const DOWNLOAD_LINE: &str = "[ExtractAudio] Destination: ";

pub fn extract_download_path(base_dir: &str, std_out: &str, preset: PresetTypes) -> String {
    let mut extracted_line = None::<String>;

    for line in std_out.lines() {
        if line.contains(DOWNLOAD_LINE) {
            tracing::info!("Line: {}", line);
            let split_point = line
                .find(format!(".{}", preset.to_string()).as_str())
                .unwrap();
            extracted_line = Some(line.split_at(split_point + 4).0.to_string());
            tracing::info!(
                "Split Section: {}",
                line.split_at(split_point + 4).0.to_string()
            );
        }
    }

    match extracted_line {
        Some(line) => {
            tracing::info!("Base Path: {}", base_dir);
            tracing::info!(
                "Appended Part: {}",
                line.replace(DOWNLOAD_LINE, "").trim_start().trim_end()
            );

            let path = PathBuf::from(base_dir)
                .join(line.replace(DOWNLOAD_LINE, "").trim_start().trim_end())
                .display()
                .to_string();

            tracing::info!("Path: {}", path);
            path
        }
        None => "".to_owned(),
    }
}
