<script lang="ts">
    import {Bell,BellRinging,GearSix,ArrowLeft} from "phosphor-svelte";

    import {page} from '$app/state';
    import LargeButton from "$lib/components/LargeButton.svelte";
    import Status from "$lib/components/Status.svelte";
    import {InstallStatus, parseInstallStatus} from "$lib";
    import {notificationMenu,downloaderState,ffmpegState} from "$lib/state.svelte";
    import SiphonTooltip from "$lib/components/SiphonTooltip.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {onMount} from "svelte";
    import {SiphonNotificationHandler} from "$lib/utils";
    import {goto} from "$app/navigation";

    let hasNotifications = $derived(notificationMenu.notifications.length > 0);
    let status = $state({
        state: InstallStatus.Missing,
        updatingState: false,
        updatingDownloader: false,
    });

    // Strange condition to avoid svelte warning...
    let statusUpdatable = $derived(!(status.state === InstallStatus.Missing || status.state === InstallStatus.UpdateAvailable))
    let downloadDisabled = $derived(statusUpdatable || status.updatingDownloader || status.updatingState);

    let isSettingsPage = $derived(page.url.pathname.includes("/settings"));
    let settingsLink = $derived(isSettingsPage ? "/" : "/settings");

    async function checkStatus() {
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

    async function checkFFmpegStatus() {
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

    async function updateDownloader() {
        downloaderState.isDownloading = true;

        try {
            await invoke("update_downloader");
            await checkStatus();
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
        finally {
            downloaderState.isDownloading = false;
        }
    }

    async function updateFFmpeg() {
        ffmpegState.isDownloading = true;

        try {
            await invoke("update_ffmpeg");
            await checkFFmpegStatus();
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
        finally {
            ffmpegState.isDownloading = false;
        }
    }

    onMount(async () => await Promise.all([checkStatus(), checkFFmpegStatus()]))
</script>

<div class="bg-zinc-900 border-b border-zinc-700 text-white p-2 flex flex-row items-center justify-items-start">
    <p class="pr-2 select-none">Downloader:</p>
    <Status status={downloaderState.install} />
    <SiphonTooltip>
        {#snippet trigger()}
            <p class="px-2 select-none">FFmpeg:</p>
        {/snippet}
        <p class="select-none text-xs">FFmpeg is an optional dependency only used with the mp3 preset.</p>
    </SiphonTooltip>
    <Status status={ffmpegState.install} />
    <div class="ml-auto flex flex-row items-center gap-2">
        <LargeButton callback={() => notificationMenu.open = !notificationMenu.open}>
            {#if hasNotifications}
                <BellRinging width="16" height="16" />
            {:else}
                <Bell width="16" height="16" />
            {/if}
        </LargeButton>
        <LargeButton callback={() => goto(settingsLink)}>
            {#if isSettingsPage}
                <ArrowLeft width="16" height="16" />
            {:else}
                <GearSix width="16" height="16" />
            {/if}
        </LargeButton>
    </div>
</div>
