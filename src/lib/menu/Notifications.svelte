<script lang="ts">
    import {Trash} from "phosphor-svelte";
    import { notificationMenu } from "$lib/state.svelte";
    import Notification from "$lib/components/Notification.svelte";
    import LargeButton from "$lib/components/LargeButton.svelte";
    import SiphonTooltip from "$lib/components/SiphonTooltip.svelte";

    let menuWidth = $derived(notificationMenu.open ? "border-l w-1/4 p-2" : "w-[0px]");
    let clearDisabled = $derived(notificationMenu.notifications.length === 0);

    async function delayOpen() {
        await new Promise(resolve => setTimeout(resolve, 100));
    }

    function clearNotifications() {
        let length = notificationMenu.notifications.length;

        for (let i = 0; i < length; i++) {
            notificationMenu.notifications.pop();
        }
    }
</script>

<div class={["bg-zinc-900 border-zinc-700 text-white h-full ml-auto transition-[width] duration-100 " +
        "ease-in-out flex flex-col", menuWidth]}>
    {#if notificationMenu.open}
    {#await delayOpen() then _}
        <div class="flex flex-row justify-between items-center pb-2">
            <p class="select-none">Notifications</p>
            <SiphonTooltip bind:disabled={clearDisabled}>
                {#snippet trigger()}
                    <LargeButton callback={clearNotifications} bind:disabled={clearDisabled}>
                        <Trash width="16" height="16" />
                    </LargeButton>
                {/snippet}
                <p>Clear all notifications.</p>
            </SiphonTooltip>
        </div>
        <div class="gap-2 flex flex-col scroll-smooth flex-[1_1_0] min-h-0 overflow-y-auto scrollbar">
            {#each notificationMenu.notifications as notification}
                <Notification notification={notification} />
            {/each}
        </div>
    {/await}
    {/if}
</div>
