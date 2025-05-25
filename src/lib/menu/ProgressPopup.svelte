<script lang="ts">
    interface Props {
        title: string
    }

    let {
        title = $bindable()
    }: Props = $props();
</script>

<div class="absolute top-1/2 left-1/2 -translate-1/2 w-screen h-screen z-40 flex items-center justify-center">
    <div class="bg-zinc-900 border border-zinc-700 w-1/2 h-30 text-white rounded-md p-2 flex flex-col opacity-100 z-50">
        <p class="select-none">{title}</p>
    </div>
</div>
