export enum InstallStatus {
    Missing,
    UpdateAvailable,
    Installed
}

export function parseInstallStatus(status: string): InstallStatus {
    switch (status) {
        case "Missing":
            return InstallStatus.Missing;
        case "Update":
            return InstallStatus.UpdateAvailable;
        case "Installed":
            return InstallStatus.Installed;
        default:
            return InstallStatus.Missing;
    }
}
