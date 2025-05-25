<script lang="ts">
    import type {Snippet} from "svelte";

    interface Props {
        label: string;
        children: Snippet;
    }

    let {
        label,
        children,
    }: Props = $props();
</script>

<div class="bg-zinc-900 rounded-md h-14 flex flex-row items-center px-4">
    <p class="select-none text-md">{label}</p>
    <div class="w-1/2 ml-auto flex items-center">
        {@render children()}
    </div>
</div>
