<script lang="ts">
    import { debug } from "@tauri-apps/plugin-log";
    import { getImageUrl, getSteamGridDbApiKey, setSteamGridDbApiKey } from "../lib/api/tauri";
    import { onMount } from "svelte";
    import GameCard from "../components/game-card.svelte";

    let api_key = $state("")
    let appId = $state("")
    let imageUrl = $state("")

    onMount(async () => {
        const key = await getSteamGridDbApiKey();
        if (key) {
            api_key = key;
        }
    });

    async function setApiKey(event: Event) {
        event.preventDefault();
        setSteamGridDbApiKey(api_key);
    }

    async function getImage(event: Event) {
        event.preventDefault();
        const image = await getImageUrl(appId);
        imageUrl = image
        debug(imageUrl)
    }

</script>

<main class="container">
    <h1>Welcome to coolapplauncher</h1>

    <GameCard></GameCard>

    <form class="row" onsubmit={setApiKey}>
        <input
            id="api-input"
            placeholder="Enter steamdb api key"
            bind:value={api_key}
        />
        <button type="submit">Submit</button>
    </form>

    <form class="row" onsubmit={getImage}>
        <input
            id="api-input"
            placeholder="Enter a steam appid"
            bind:value={appId}
        />
        <button type="submit">Submit</button>
    </form>

    <p>
        <img alt="appid game" src="{imageUrl}">
    </p>
</main>

<style>

    :root {
        font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
        font-size: 16px;
        line-height: 24px;
        font-weight: 400;

        color: #0f0f0f;
        background-color: #a5b4a2b6;

        font-synthesis: none;
        text-rendering: optimizeLegibility;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        -webkit-text-size-adjust: 100%;
    }

    .container {
        margin: 0;
        padding-top: 10vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        text-align: center;
    }

    .row {
        display: flex;
        justify-content: center;
    }

    form {
        padding: 5px;
    }

    a {
        font-weight: 500;
        color: #646cff;
        text-decoration: inherit;
    }

    h1 {
        text-align: center;
    }

    img {
        height: 400px
    }

    input,
    button {
        border-radius: 8px;
        border: 1px solid transparent;
        padding: 0.6em 1.2em;
        font-size: 1em;
        font-weight: 500;
        font-family: inherit;
        color: #0f0f0f;
        background-color: #ffffff;
        transition: border-color 0.25s;
        box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
    }

    button {
        cursor: pointer;
    }

    button:hover {
        border-color: #396cd8;
    }
    button:active {
        border-color: #396cd8;
        background-color: #e8e8e8;
    }

    input,
    button {
        outline: none;
    }

    #api-input {
        margin-right: 5px;
    }

    @media (prefers-color-scheme: dark) {
        :root {
            color: #f6f6f6;
            background-color: #2f2f2f;
        }

        input,
        button {
            color: #ffffff;
            background-color: #0f0f0f98;
        }
        button:active {
            background-color: #0f0f0f69;
        }
    }
</style>
