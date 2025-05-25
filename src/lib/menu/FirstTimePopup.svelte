<script lang="ts">
    import ToolbarButton from "$lib/components/ToolbarButton.svelte";
    import {updateDownloader,updateFFmpeg} from "$lib/state.svelte";
    import {SiphonNotificationHandler} from "$lib/utils";

    interface Props {
        firstTime: boolean;
    }

    let {
        firstTime = $bindable(false),
    }: Props = $props();

    async function initialSetup() {
        try {
            firstTime = false;
            await Promise.all([updateDownloader(), updateFFmpeg()]);
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
    }
</script>

{#if firstTime}
    <div class="absolute top-1/2 left-1/2 -translate-1/2 w-screen h-screen z-40 flex items-center justify-center">
        <div class="z-40 bg-slate-500 opacity-30 w-full h-full absolute"></div>
        <div class="bg-zinc-900 border border-zinc-700 w-5/12 text-white rounded-md p-2 flex flex-col gap-2 z-50 select-none">
            <h1 class="text-xl">First Time Setup</h1>
            <p class="text-xs">
                Welcome to Siphon. Please click the button below to download the software requirements automatically.<br><br>You
                can start downloading videos the moment both crosses in the top left become ticks.
            </p>
            <ToolbarButton callback={async () => await initialSetup()}>
                <p>Install</p>
            </ToolbarButton>
        </div>
    </div>
{/if}
