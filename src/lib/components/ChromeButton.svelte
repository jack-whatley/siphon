<script lang="ts">
    import { Button } from "bits-ui";
    import type { Snippet } from "svelte";

    interface Props {
        children: Snippet;
        label: string;
        callback: () => void;
    }

    let {
        children,
        label,
        callback
    }: Props = $props();
</script>

<Button.Root aria-label={label}
    class="bg-transparent hover:bg-slate-600 hover:cursor-pointer h-full p-2"
    onclick={callback}>
    {@render children()}
</Button.Root>
