import {listen, type UnlistenFn} from "@tauri-apps/api/event";
import { v4 as uuidv4 } from "uuid";

export enum Level {
    Info,
    Warning,
    Error,
}

export class SiphonBackendNotification {
    level: Level;
    message: string;

    constructor(level: Level, message: string) {
        this.level = level;
        this.message = message;
    }
}

export class SiphonNotification {
    id: string;
    level: Level;
    message: string;

    constructor(notification: SiphonBackendNotification) {
        this.id = uuidv4();
        this.level = notification.level;
        this.message = notification.message;
    }
}

export class SiphonNotificationHandler {
    private static _instance: SiphonNotificationHandler;
    private constructor() {
        this._unlisten = null;
        this._onEvent = null;
    }

    private _unlisten: UnlistenFn | null;
    private _onEvent: ((a: SiphonBackendNotification) => void) | null;

    private setUnlisten(unlisten: UnlistenFn) {
        this._unlisten = unlisten;
    }

    public static Instance(): SiphonNotificationHandler {
        return this._instance || (this._instance = new SiphonNotificationHandler());
    }

    public RaiseError(error: any) {
        let notification = new SiphonBackendNotification(Level.Error, error as string);
        if (this._onEvent !== null) {
            this._onEvent(notification);
        }
    }

    public async RegisterListener(onEvent: (a: SiphonBackendNotification) => void) {
        this._onEvent = onEvent;

        this.setUnlisten(
            await listen<SiphonBackendNotification>('notification',
                (event) => onEvent(event.payload))
        );
    }

    public destroyListener() {
        if (this._unlisten !== null) {
            this._unlisten();
        }
    }
}
