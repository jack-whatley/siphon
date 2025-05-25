<script lang="ts">
    import {CircleNotch, FolderOpen} from "phosphor-svelte";

    import TextInput from "$lib/components/TextInput.svelte";
    import LargeButton from "$lib/components/LargeButton.svelte";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {SiphonNotificationHandler} from "$lib/utils";
    import {open} from "@tauri-apps/plugin-dialog";
    import ComboBox from "$lib/components/ComboBox.svelte";
    import FormLabel from "$lib/components/FormLabel.svelte";
    import DefaultButton from "$lib/components/DefaultButton.svelte";
    import {revealItemInDir} from "@tauri-apps/plugin-opener";
    import {blur} from 'svelte/transition';
    import {InstallStatus} from "$lib";
    import {downloaderState,ffmpegState} from "$lib/state.svelte";

    let url = $state("");
    let path = $state("")
    let preset = $state("");

    let presets = $state<Array<{ value: string; label: string; disabled?: boolean }>>([]);

    let isDownloading = $state(false);
    let latestDownload = $state("");

    let downloadDisabled = $derived(
        isDownloading || preset === "" || url === "" ||
        downloaderState.install !== InstallStatus.Installed ||
        ffmpegState.install !== InstallStatus.Installed
    );

    onMount(async () => path = await getDefaultPath());
    onMount(async () => presets = await getPresets());

    async function openDirectory() {
        let defaultPath = await getDefaultPath();

        let directory = await open({
            multiple: false,
            directory: true,
            defaultPath: defaultPath
        });

        if (directory !== null) {
            path = directory;
        }
    }

    async function getDefaultPath(): Promise<string> {
        try {
            return await invoke<string>('default_download_dir');
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }

        return "";
    }

    async function getPresets(): Promise<Array<{ value: string; label: string; disabled?: boolean }>> {
        let presets: Array<string> | null = null;

        try {
            presets = await invoke<Array<string>>('all_presets');
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }

        if (presets === null) {
            return [];
        }
        
        let result: Array<{ value: string; label: string; disabled?: boolean }> = [];

        for (let i = 0; i < presets.length; i++) {
            result.push({ value: presets[i], label: presets[i] });
        }
        
        return result;
    }

    async function downloadVideo() {
        isDownloading = true;

        try {
            latestDownload = await invoke('download_video', { url: url, directory: path, preset: preset.toUpperCase() });
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
        finally {
            isDownloading = false;
        }
    }

    async function openSaveDirectory() {
        try {
            console.log(latestDownload);

            if (latestDownload !== "") {
                await revealItemInDir(latestDownload);
            }
            else {
                await revealItemInDir(path);
            }
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
    }
</script>

<div class="w-full h-full p-2 flex flex-col gap-2" in:blur={{ delay: 200, duration: 200 }}>
    <div class="flex flex-row gap-2">
        <div class="flex-1">
            <FormLabel label="URL">
                <TextInput bind:text={url} />
            </FormLabel>
        </div>
        <FormLabel label="Preset">
            <ComboBox bind:value={preset} bind:items={presets} placeholder="" />
        </FormLabel>
    </div>
    <div class="flex flex-row gap-2">
        <div class="flex-1">
            <FormLabel label="Output Directory">
                <TextInput bind:text={path} />
            </FormLabel>
        </div>
        <div class="mt-auto">
            <LargeButton callback={async () => await openDirectory()}>
                <FolderOpen width="16" height="16" />
            </LargeButton>
        </div>
    </div>
    <div class="flex flex-row mt-auto ml-auto gap-2">
        <DefaultButton callback={async () => await downloadVideo()} bind:disabled={downloadDisabled}>
            {#if isDownloading}
                <CircleNotch width="16" height="16" class="animate-spin" />
            {:else}
                <p class="select-none text-white text-xs">Download</p>
            {/if}
        </DefaultButton>
        <DefaultButton callback={async () => await openSaveDirectory()}>
            <p class="select-none text-white text-xs">Open Directory</p>
        </DefaultButton>
    </div>
</div>
