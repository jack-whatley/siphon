<script lang="ts">
    import {ArrowsClockwise, Bell, BellRinging, DownloadSimple, CircleNotch} from "phosphor-svelte";

    import LargeButton from "$lib/components/LargeButton.svelte";
    import Status from "$lib/components/Status.svelte";
    import {InstallStatus, parseInstallStatus} from "$lib";
    import { notificationMenu } from "$lib/state.svelte";
    import SiphonTooltip from "$lib/components/SiphonTooltip.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import {SiphonNotificationHandler} from "$lib/utils";

    let hasNotifications = $derived(notificationMenu.notifications.length > 0);
    let status = $state({
        state: InstallStatus.Missing,
        updatingState: false,
        updatingDownloader: false,
    });

    // Strange condition to avoid svelte warning...
    let statusUpdatable = $derived(!(status.state === InstallStatus.Missing || status.state === InstallStatus.UpdateAvailable))
    let downloadDisabled = $derived(statusUpdatable || status.updatingDownloader || status.updatingState);

    async function checkStatus() {
        status.updatingState = true;

        try {
            status.state = parseInstallStatus(await invoke<string>("downloader_state"));
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
        finally {
            status.updatingState = false;
        }
    }

    async function updateDownloader() {
        status.updatingDownloader = true;

        try {
            await invoke("update_downloader");
            await checkStatus();
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
        finally {
            status.updatingDownloader = false;
        }
    }

    onMount(async () => await checkStatus());
</script>

<div class="bg-zinc-900 border-b border-zinc-700 text-white p-2 flex flex-row items-center justify-items-start">
    <p class="pr-2 select-none">Downloader Status:</p>
    <SiphonTooltip>
        {#snippet trigger()}
            <Status status={status.state} />
        {/snippet}
        <p>Shows the install status of the downloader.</p>
    </SiphonTooltip>
    <div class="ml-auto flex flex-row items-center gap-2">
        <SiphonTooltip bind:disabled={downloadDisabled}>
            {#snippet trigger()}
                <LargeButton callback={async () => await updateDownloader()} bind:disabled={downloadDisabled}>
                    {#if status.updatingDownloader}
                        <CircleNotch width="16" height="16" class="animate-spin" />
                    {:else}
                        <DownloadSimple width="16" height="16" />
                    {/if}
                </LargeButton>
            {/snippet}
            <p>Download/Update</p>
        </SiphonTooltip>
        <SiphonTooltip>
            {#snippet trigger()}
                <LargeButton callback={async () => await checkStatus()} bind:disabled={status.updatingState}>
                    {#if status.updatingState}
                        <CircleNotch width="16" height="16" class="animate-spin" />
                    {:else}
                        <ArrowsClockwise width="16" height="16" />
                    {/if}
                </LargeButton>
            {/snippet}
            <p>Check for updates.</p>
        </SiphonTooltip>
        <LargeButton callback={() => notificationMenu.open = !notificationMenu.open}>
            {#if hasNotifications}
                <BellRinging width="16" height="16" />
            {:else}
                <Bell width="16" height="16" />
            {/if}
        </LargeButton>
    </div>
</div>
