<script lang="ts">
    import { blur } from 'svelte/transition';
    import SettingsMenuBar from "$lib/menu/SettingsMenuBar.svelte";

    let { children } = $props();
</script>

<div class="w-full h-full flex flex-col" in:blur={{ delay: 200, duration: 200 }}>
    <SettingsMenuBar />
    {@render children()}
</div>
