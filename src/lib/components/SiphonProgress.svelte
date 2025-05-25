<script lang="ts">
    import {Progress} from "bits-ui";
    import {ProgressBar} from "$lib/utils/progress";
    import {onMount} from "svelte";

    let progress = $state(new ProgressBar(7));

    onMount(() => progress.current = 3);
</script>

<Progress.Root bind:value={progress.current} max={progress.max}  />
