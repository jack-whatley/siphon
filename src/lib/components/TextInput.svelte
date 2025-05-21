<script lang="ts">
    let {
        text = $bindable(""),
        disabled = $bindable(false),
    }: { text: string, disabled?: boolean } = $props();
</script>

<input type="search" bind:value={text} autocapitalize="off" spellcheck="false" autocomplete="off"
       class="bg-zinc-900 p-2 text-xs rounded-md focus:outline-none focus:ring-1 focus:ring-zinc-700" />
