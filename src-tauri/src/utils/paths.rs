use std::path::PathBuf;

pub const APP_GUID: &str = "dev.jackwhatley.siphon";

#[cfg(target_os = "windows")]
pub const DOWNLOAD_EXECUTABLE: &str = "yt-dlp.exe";

#[cfg(target_os = "macos")]
pub const DOWNLOAD_EXECUTABLE: &str = "yt-dlp_macos";

#[cfg(target_os = "linux")]
pub const DOWNLOAD_EXECUTABLE: &str = "yt-dlp_linux";

pub fn default_app_dir() -> PathBuf {
    dirs_next::data_dir().unwrap().join(APP_GUID)
}

pub fn downloader_path() -> PathBuf {
    default_app_dir().join(DOWNLOAD_EXECUTABLE)
}

pub fn ffmpeg_dir() -> PathBuf {
    default_app_dir().join("ffmpeg")
}

pub fn ffmpeg_executable() -> PathBuf {
    ffmpeg_dir()
        .join("ffmpeg-2025-05-19-git-c55d65ac0a-essentials_build")
        .join("bin")
        .join("ffmpeg.exe")
}
