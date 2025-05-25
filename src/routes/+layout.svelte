<script lang="ts">
    import '../app.css'

    import Chrome from "$lib/menu/Chrome.svelte";
    import MenuBar from "$lib/menu/MenuBar.svelte";
    import Notifications from "$lib/menu/Notifications.svelte";
    import {onDestroy, onMount} from "svelte";
    import {SiphonNotification, SiphonNotificationHandler} from "$lib/utils";
    import {downloaderStatus, notificationMenu} from "$lib/state.svelte";
    import type {SiphonBackendNotification} from "$lib/utils/notifications";
    import FirstTimePopup from "$lib/menu/FirstTimePopup.svelte";
    import {invoke} from "@tauri-apps/api/core";

    let { children } = $props();

    let firstTime = $state(false);

    onMount(async () => {
        await SiphonNotificationHandler.Instance().RegisterListener(addNotification);
        await getFirstTime();
    });

    onDestroy(() => SiphonNotificationHandler.Instance().destroyListener());

    function addNotification(notification: SiphonBackendNotification) {
        notificationMenu.notifications.push(new SiphonNotification(notification));
    }

    async function getFirstTime() {
        try {
            firstTime = await invoke("initial_setup_required");
        }
        catch (e) {
            SiphonNotificationHandler.Instance().RaiseError(e);
        }
    }
</script>

<FirstTimePopup bind:firstTime={firstTime} />

<div class="w-screen h-screen flex flex-col">
    <Chrome />
    <MenuBar />
    <div class="bg-neutral-800 text-white flex-1 flex flex-row max-w-screen">
        {@render children()}
        <Notifications />
    </div>
</div>
