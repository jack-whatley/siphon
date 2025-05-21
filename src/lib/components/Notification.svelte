<script lang="ts">
    import {Button} from "bits-ui";
    import {SiphonNotification,Level} from "$lib/utils";
    import { notificationMenu } from "$lib/state.svelte";

    let {
        notification
    }: { notification: SiphonNotification } = $props();

    function onClick() {
        let index = notificationMenu.notifications.indexOf(notification, 0);

        if (index > -1) {
            notificationMenu.notifications.splice(index, 1);
        }
    }
</script>

<Button.Root onclick={onClick}
        class="bg-neutral-800 flex flex-row justify-center rounded-md hover:cursor-pointer">
    {#if notification.level.valueOf() === Level.Info}
        <div class="w-[5px] rounded-tl-md rounded-bl-md bg-emerald-600"></div>
    {:else if notification.level.valueOf() === Level.Warning}
        <div class="w-[5px] rounded-tl-md rounded-bl-md bg-amber-500"></div>
    {:else}
        <div class="w-[5px] rounded-tl-md rounded-bl-md bg-rose-800"></div>
    {/if}
    <div class="flex-1 flex flex-col my-1 ml-1 mr-2 select-none items-start">
        <p class="text-xs text-left">{notification.message}</p>
    </div>
</Button.Root>
