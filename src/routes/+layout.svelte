<script lang="ts">
    import '../app.css'

    import Chrome from "$lib/menu/Chrome.svelte";
    import MenuBar from "$lib/menu/MenuBar.svelte";
    import Notifications from "$lib/menu/Notifications.svelte";
    import {onDestroy, onMount} from "svelte";
    import {SiphonNotification, SiphonNotificationHandler} from "$lib/utils";
    import {notificationMenu} from "$lib/state.svelte";
    import type {SiphonBackendNotification} from "$lib/utils/notifications";

    let { children } = $props();

    onMount(async () => await SiphonNotificationHandler.Instance().RegisterListener(addNotification));
    onDestroy(() => SiphonNotificationHandler.Instance().destroyListener());

    function addNotification(notification: SiphonBackendNotification) {
        notificationMenu.notifications.push(new SiphonNotification(notification));
    }
</script>

<div class="w-screen h-screen flex flex-col">
    <Chrome />
    <MenuBar />
    <div class="bg-neutral-800 text-white flex-1 flex flex-row max-w-screen">
        {@render children()}
        <Notifications />
    </div>
</div>
