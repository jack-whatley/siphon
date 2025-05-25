<script lang="ts">
    import {InstallStatus} from "$lib";
    import {ArrowUp, Check, X} from "phosphor-svelte";
    import SiphonTooltip from "$lib/components/SiphonTooltip.svelte";

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

    let currentColour = $derived(colourMap.get(status));
    let currentTooltip = $derived(textMap.get(status));
</script>

<SiphonTooltip>
    {#snippet trigger()}
        <div class={["p-1 rounded-full text-white", currentColour]}>
            {#if status === InstallStatus.Missing}
                <X width="16" height="16" />
            {:else if status === InstallStatus.UpdateAvailable}
                <ArrowUp width="16" height="16" />
            {:else}
                <Check width="16" height="16" />
            {/if}
        </div>
    {/snippet}
    <p class="select-none">{currentTooltip}</p>
</SiphonTooltip>
