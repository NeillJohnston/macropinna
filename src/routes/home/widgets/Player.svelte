<script lang="ts">
    import { invoke } from '@tauri-apps/api';
	import { onMount } from 'svelte';

    let artist: string | undefined = undefined;
    let title: string | undefined = undefined;
    let album: string | undefined = undefined;
    let url: string | undefined = undefined;

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
    <img id="art" src={url} alt="Album Art for Musical Artist"/>
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
        object-fit: contain;
        width: auto;
        height: 100%;
        aspect-ratio: 1/1;
        margin-right: 0.5rem;
    }

    #artist {
        font-weight: bold;
    }
    
    #title {
        font-size: 2.00rem;
        font-weight: bold;
    }

    #album {
        font-size: 0.71rem;
        font-style: italic;
    }
</style>