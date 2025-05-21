<script lang="ts">
    import {InstallStatus} from "$lib";

    let { status = $bindable() }: { status: InstallStatus } = $props();

    const colourMap = new Map<InstallStatus, string>([
        [InstallStatus.Missing, "bg-rose-800"],
        [InstallStatus.UpdateAvailable, "bg-blue-700"],
        [InstallStatus.Installed, "bg-emerald-800"]
    ]);

    const textMap = new Map<InstallStatus, string>([
        [InstallStatus.Missing, "Missing"],
        [InstallStatus.UpdateAvailable, "Update Available"],
        [InstallStatus.Installed, "Installed"]
    ]);

    let currentText = $derived(textMap.get(status));
    let currentColour = $derived(colourMap.get(status));
</script>

<div class={["px-2 py-1 rounded-md text-white", currentColour]}>
    <p class="select-none">{currentText}</p>
</div>
