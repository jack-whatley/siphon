import {SiphonNotification, SiphonNotificationHandler} from "$lib/utils";
import {InstallStatus, parseInstallStatus} from "$lib/index";
import {invoke} from "@tauri-apps/api/core";

export const notificationMenu = $state({
    open: false,
    notifications: new Array<SiphonNotification>(),
});

export const downloaderState = $state({
    install: InstallStatus.Missing,
    isDownloading: false,
    isUpdating: false,
});

export async function downloaderStatus() {
    downloaderState.isUpdating = true;

    try {
        downloaderState.install = parseInstallStatus(await invoke<string>("downloader_state"));
    }
    catch (e) {
        SiphonNotificationHandler.Instance().RaiseError(e);
    }
    finally {
        downloaderState.isUpdating = false;
    }
}

export async function updateDownloader() {
    downloaderState.isDownloading = true;

    try {
        await invoke("update_downloader");
        await downloaderStatus();
    }
    catch (e) {
        SiphonNotificationHandler.Instance().RaiseError(e);
    }
    finally {
        downloaderState.isDownloading = false;
    }
}

export const ffmpegState = $state({
    install: InstallStatus.Missing,
    isDownloading: false,
    isUpdating: false,
});

export async function ffmpegStatus() {
    ffmpegState.isUpdating = true;

    try {
        ffmpegState.install = parseInstallStatus(await invoke<string>("ffmpeg_state"));
    }
    catch (e) {
        SiphonNotificationHandler.Instance().RaiseError(e);
    }
    finally {
        ffmpegState.isUpdating = false;
    }
}

export async function updateFFmpeg() {
    ffmpegState.isDownloading = true;

    try {
        await invoke("update_ffmpeg");
        await ffmpegStatus();
    }
    catch (e) {
        SiphonNotificationHandler.Instance().RaiseError(e);
    }
    finally {
        ffmpegState.isDownloading = false;
    }
}
