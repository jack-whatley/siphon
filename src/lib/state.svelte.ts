import {SiphonNotification} from "$lib/utils";

export const notificationMenu = $state({
    open: false,
    notifications: new Array<SiphonNotification>(),
});
