import {SiphonNotification} from "$lib/utils";
import {InstallStatus} from "$lib/index";

export const notificationMenu = $state({
    open: false,
    notifications: new Array<SiphonNotification>(),
});

export const downloaderState = $state({
    install: InstallStatus.Missing,
    isDownloading: false,
    isUpdating: false,
});

export const ffmpegState = $state({
    install: InstallStatus.Missing,
    isDownloading: false,
    isUpdating: false,
});
