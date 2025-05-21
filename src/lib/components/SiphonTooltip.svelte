<script lang="ts">
    import { Tooltip } from "bits-ui";
    import { type Snippet } from "svelte";

    type Props = Tooltip.Props & {
        trigger: Snippet;
        children: Snippet;
        disabled?: boolean;
    };

    let {
        open = $bindable(false),
        trigger,
        children,
        disabled = $bindable(false),
    }: Props = $props();
</script>

<Tooltip.Root bind:open openDelay={125}>
    <Tooltip.Trigger>
        {@render trigger()}
    </Tooltip.Trigger>
    {#if !disabled}
        <Tooltip.Content side="bottom" sideOffset={5}
                         class="select-none text-white bg-black p-2 rounded-md shadow-md text-xs">
            <Tooltip.Arrow class="hidden" />
            {@render children()}
        </Tooltip.Content>
    {/if}
</Tooltip.Root>
