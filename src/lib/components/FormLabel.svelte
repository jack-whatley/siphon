<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props {
        label: string;
        children: Snippet;
    }

    let {
        label = $bindable(""),
        children,
    }: Props = $props();
</script>

<div class="flex flex-col gap-1">
    <p class="select-none text-sm ml-1">{label}</p>
    {@render children()}
</div>
