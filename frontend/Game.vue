<template>
  
  <p>{{ gameId }}</p>
  <div v-if="gameId">
    <button @click="startGame">start game</button>
  </div>
  <div v-else>
    <button @click="createGame">create game</button>
    <input v-model="gameIdInput" />
    <button @click="joinGame">join game</button>
  </div>
</template>

<script setup>
import { onMounted, ref } from "vue";
 
const props = defineProps({
  connection: Object,
});

const gameId = ref(null);
const gameIdInput = ref("")

function startGame() {
  connection.invoke("StartGame")
}

function createGame() {
  console.log("create game")
  connection.invoke("CreateGame");
}

function joinGame() {
  connection.invoke("JoinGame", gameIdInput.value)
}

const { connection } = props.connection;

onMounted(() => {
  connection.on("GameCreated", (id) => {
    console.log(id);
    gameId.value = id
  });

  connection.on("JoinedGame", (id) => {
    console.log("joined", id)
    gameId.value = id
  })

  
  console.log("game on!")
});

</script>
