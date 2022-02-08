<script lang="ts">
    import { onMount } from 'svelte';

    interface ChatMessage {
        username: string,
        text: string,
    }

    let socket: WebSocket | null = null
    let messages: ChatMessage[] = []
    let message = ""

    function handleClick() {
        if (!socket) {
            return
        }

        socket.send(message)
        const myMessage = { username: "you", text: message }
        messages = [...messages, myMessage]
        message = ""
    }

    onMount(() => {
        socket = new WebSocket('ws://localhost:3030/chat')

        socket.addEventListener('open', (_) => {
            console.log("open");
        })

        socket.addEventListener('message', (event) => {
            console.log(event.data)
            const recievedMessage = JSON.parse(event.data);
            console.log(message)

            messages = [...messages, recievedMessage]
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
    <p>{message.username}: {message.text}</p>
{/each}
