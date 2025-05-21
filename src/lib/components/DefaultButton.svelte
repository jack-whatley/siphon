<script lang="ts">
    import type { Snippet } from "svelte";
    import { Button } from "bits-ui";
    import type {ClassValue} from "svelte/elements";

    interface Props {
        children: Snippet;
        callback: () => void;
        disabled?: boolean;
    }

    let props: Props = $props();
</script>

<Button.Root bind:disabled={props.disabled} onclick={props.callback}
             class="bg-emerald-700 p-2 rounded-md shadow-sm transition-all duration-100 not-disabled:hover:cursor-pointer
            not-disabled:hover:opacity-85 not-disabled:active:scale-[0.98] flex flex-row items-center
            justify-center disabled:opacity-45">
    {@render props.children()}
</Button.Root>
