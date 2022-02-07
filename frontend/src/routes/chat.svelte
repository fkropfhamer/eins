<script lang="ts">
    import { onMount } from 'svelte';

    let socket: WebSocket | null = null
    let messages: string[] = []
    let message = ""

    function handleClick() {
        if (!socket) {
            return
        }

        socket.send(message)
        messages = [...messages, `<you>: ${message}`]
        message = ""
    }

    onMount(() => {
        socket = new WebSocket('ws://localhost:3030/chat')

        socket.addEventListener('open', (_) => {
            console.log("open");
        })

        socket.addEventListener('message', (event) => {
            console.log(event.data)
            messages = [...messages, event.data]
        })
    })
</script>


<svelte:head>
	<title>Chat</title>
</svelte:head>

<h1>Chat</h1>
<input bind:value={message} >
<button on:click={handleClick}>send</button>
{#each messages as message}
    <p>{message}</p>
{/each}
