<script lang="ts">
    import {CircleNotch,ArrowsClockwise,DownloadSimple} from "phosphor-svelte";

    import Setting from "$lib/components/Setting.svelte";
    import Status from "$lib/components/Status.svelte";

    import {downloaderState,downloaderStatus,updateDownloader} from "$lib/state.svelte";
    import LargeButton from "$lib/components/LargeButton.svelte";
    import SiphonTooltip from "$lib/components/SiphonTooltip.svelte";

    let downloadDisabled = $derived(downloaderState.isUpdating || downloaderState.isDownloading);
</script>

<div class="flex-1 flex flex-col p-2 gap-2">
    <h1 class="select-none text-md">Downloader Settings</h1>
    <Setting label="Install Status">
        <div class="ml-auto w-min h-min flex items-center"><Status status={downloaderState.install} /></div>
    </Setting>
    <Setting label="Commands">
        <div class="ml-auto w-min h-min flex items-center gap-2">
            <SiphonTooltip bind:disabled={downloadDisabled}>
                {#snippet trigger()}
                    <LargeButton callback={async () => await updateDownloader()} bind:disabled={downloadDisabled}>
                        {#if downloaderState.isDownloading}
                            <CircleNotch width="16" height="16" class="animate-spin" />
                        {:else}
                            <DownloadSimple width="16" height="16" />
                        {/if}
                    </LargeButton>
                {/snippet}
                <p class="select-none">Update the downloader to the latest version.</p>
            </SiphonTooltip>
            <SiphonTooltip bind:disabled={downloaderState.isUpdating}>
                {#snippet trigger()}
                    <LargeButton callback={async () => await downloaderStatus()} bind:disabled={downloaderState.isUpdating}>
                        {#if downloaderState.isUpdating}
                            <CircleNotch width="16" height="16" class="animate-spin" />
                        {:else}
                            <ArrowsClockwise width="16" height="16" class="hover:animate-[spin_1s_ease-in-out]" />
                        {/if}
                    </LargeButton>
                {/snippet}
                <p class="select-none">Check for downloader updates.</p>
            </SiphonTooltip>
        </div>
    </Setting>
</div>
