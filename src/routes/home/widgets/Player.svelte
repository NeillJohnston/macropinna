<script lang="ts">
    import { invoke } from '@tauri-apps/api';
    import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

    export let props: any;

    let artist: string | undefined = undefined;
    let title: string | undefined = undefined;
    let album: string | undefined = undefined;
    let url: string | undefined = undefined;

    const getUrl = (url: string) => {
        // Local files need to be loaded with the asset protocol
        // TODO assumes that a URL from the media player always comes prefixed
        // with a protocol (file://, http://, etc.) - might not be the case with
        // every player
        if (url.startsWith('file://')) {
            return convertFileSrc(url.slice(7));
        }
        else {
            return url;
        }
    }

    onMount(() => {
        return setInterval(async () => {
            const metadata: any = await invoke('get_player_metadata');

            artist = metadata.track_artists?.join(', ');
            title = metadata.track_title;
            album = metadata.album_name;
            // TODO can't load local files, which are provided by Firefox (and
            // probably all local-hosted players)
            url = metadata.art_url;
        }, 1_000);
    });
</script>

<div id="player">
    {#if url}
    <img id="art" src={getUrl(url)} alt="Album art" />
    {/if}
    <div id="info">
        <p id="title">{title}</p>
        <p id="artist">{artist}</p>
        <p id="album">{album}</p>
    </div>
</div>

<style>
    p {
        margin: 0;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
    }
    
    #player {
        width: 100%;
        height: 100%;
        display: flex;
        padding: 0.25rem;
        box-sizing: border-box;
    }

    #info {
        flex: 1;
        /* Surprisingly counterintuitive even for CSS:
        https://stackoverflow.com/questions/36230944/prevent-flex-items-from-overflowing-a-container
        Anyway, it works. Tested on several Red Sparrowes songs :+1: */
        min-width: 0;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }

    #art {
        object-fit: cover;
        width: auto;
        height: 100%;
        aspect-ratio: 1/1;
        margin-right: 0.5rem;
    }
    
    #title {
        font-size: 1.41em;
        font-weight: bold;
    }

    #artist {
        font-size: 0.71em;
        font-weight: bold;
    }

    #album {
        font-size: 0.71em;
        font-style: italic;
    }
</style>