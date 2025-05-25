<script lang="ts">
    import ToolbarButton from "$lib/components/ToolbarButton.svelte";
    import {goto} from "$app/navigation";
    import {page} from '$app/state';

    let downloaderDisabled = $derived(page.url.pathname === "/settings");
    let ffmpegDisabled = $derived(page.url.pathname === "/settings/ffmpeg");
</script>

<div class="bg-zinc-900 border-b border-zinc-700 text-white gap-2 p-2 flex flex-row items-center justify-items-start">
    <ToolbarButton callback={() => goto("/settings")} bind:disabled={downloaderDisabled}>
        <p class="select-none text-sm">Downloader</p>
    </ToolbarButton>
    <ToolbarButton callback={() => goto("/settings/ffmpeg")} bind:disabled={ffmpegDisabled}>
        <p class="select-none text-sm">FFmpeg</p>
    </ToolbarButton>
</div>
