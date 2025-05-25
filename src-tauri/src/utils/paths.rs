use std::path::PathBuf;

pub const APP_GUID: &str = "dev.jackwhatley.siphon";

pub const DOWNLOAD_EXECUTABLE: &str = "yt-dlp.exe";

pub fn default_app_dir() -> PathBuf {
    dirs_next::data_dir().unwrap().join(APP_GUID)
}

pub fn downloader_path() -> PathBuf {
    default_app_dir().join(DOWNLOAD_EXECUTABLE)
}

pub fn ffmpeg_download_dir() -> PathBuf {
    default_app_dir().join("ffmpeg_download")
}

pub fn ffmpeg_executable() -> PathBuf {
    default_app_dir().join("ffmpeg.exe")
}

pub fn ffmpeg_version() -> PathBuf {
    default_app_dir().join("ffmpeg-version.json")
}

pub fn cache_path() -> PathBuf {
    default_app_dir().join("github-cache.json")
}
