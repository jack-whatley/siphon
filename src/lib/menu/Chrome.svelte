<script lang="ts">
    import { Funnel, Minus, X } from "phosphor-svelte";
    import ChromeButton from "$lib/components/ChromeButton.svelte";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    function minimiseWindow() {
        let window = getCurrentWindow();

        window.minimize();
    }

    function closeWindow() {
        let window = getCurrentWindow();

        window.close();
    }
</script>

<div data-tauri-drag-region
     class="bg-neutral-800 text-white border-zinc-700 border-b h-[35px] flex flex-row items-center pl-2 select-none z-50">
    <Funnel width="16" height="16" />
    <p class="px-2">Siphon</p>
    <div class="ml-auto flex flex-row items-center h-full">
        <ChromeButton label="minimise" callback={minimiseWindow}>
            <Minus width="16" height="16" />
        </ChromeButton>
        <ChromeButton label="minimise" callback={closeWindow}>
            <X width="16" height="16" />
        </ChromeButton>
    </div>
</div>
