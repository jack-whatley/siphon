<script lang="ts">
    import type { Snippet } from "svelte";
    import { Button } from "bits-ui";

    interface Props {
        children: Snippet;
        callback: () => void;
        disabled?: boolean;
    }

    let {
        children,
        callback,
        disabled = $bindable(false)
    }: Props = $props();
</script>

<Button.Root bind:disabled onclick={callback}
        class="bg-neutral-800 p-2 rounded-md shadow-sm transition-all duration-100 not-disabled:hover:cursor-pointer
            not-disabled:hover:bg-neutral-700 not-disabled:active:scale-[0.98] flex flex-row items-center
            justify-center disabled:opacity-45">
    {@render children()}
</Button.Root>
