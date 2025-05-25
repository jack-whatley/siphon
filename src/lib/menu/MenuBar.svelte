<script lang="ts">
    import {Bell,BellRinging,GearSix,ArrowLeft} from "phosphor-svelte";

    import {page} from '$app/state';
    import LargeButton from "$lib/components/LargeButton.svelte";
    import Status from "$lib/components/Status.svelte";
    import {notificationMenu,downloaderState,ffmpegState,downloaderStatus,ffmpegStatus} from "$lib/state.svelte";
    import SiphonTooltip from "$lib/components/SiphonTooltip.svelte";
    import {goto} from "$app/navigation";
    import {onMount} from "svelte";

    let hasNotifications = $derived(notificationMenu.notifications.length > 0);

    let isSettingsPage = $derived(page.url.pathname.includes("/settings"));
    let settingsLink = $derived(isSettingsPage ? "/" : "/settings");

    onMount(async () => await Promise.all([downloaderStatus(), ffmpegStatus()]))
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
