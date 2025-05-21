<script lang="ts">
    import { Select } from "bits-ui";
    import { CaretDown, Check } from "phosphor-svelte";

    interface Props {
        value: string;
        items: Array<{ value: string; label: string; disabled?: boolean }>;
        placeholder: string;
    }

    let {
        value = $bindable<string>("test"),
        items = $bindable([]),
        placeholder,
    }: Props = $props();

    let isOpen = $state(false);

    const rotation = $derived(isOpen ? "rotate-180" : "rotate-0");
    const label = $derived(
        value
            ? items.find((item) => item.value === value)?.label
            : placeholder
    );
</script>

<Select.Root bind:open={isOpen} onSelectedChange={(v) => (value = v.value)}>
    <Select.Trigger class="bg-zinc-900 p-2 rounded-md min-w-40 hover:ring-1 hover:ring-zinc-700 hover:cursor-pointer">
        <div class="flex flex-row items-center">
            <p class="select-none text-white text-xs">{label}</p>
            <CaretDown width="16" height="16" class={["ml-auto transition-[rotate] duration-100 ease-in-out rotate-0", rotation]} />
        </div>
    </Select.Trigger>
    <Select.Content class="bg-zinc-900 p-2 mt-2 rounded-md ring-1 ring-zinc-700 text-white text-xs">
        {#each items as item}
            {#if value === item.value}
                <Select.Item value={item.value} label={item.label} disabled={item.disabled}
                             class="w-full flex flex-row items-center p-2 cursor-default rounded-md ring-1 ring-zinc-700
                            hover:bg-zinc-800">
                    {item.label}
                    <div class="ml-auto">
                        <Check width="16" height="16" />
                    </div>
                </Select.Item>
            {:else}
                <Select.Item value={item.value} label={item.label} disabled={item.disabled}
                             class="w-full flex flex-row items-center p-2 cursor-default rounded-md hover:bg-zinc-800
                             hover:cursor-pointer">
                    {item.label}
                </Select.Item>
            {/if}
        {/each}
    </Select.Content>
</Select.Root>
