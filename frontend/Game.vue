<template>
</template>

<script setup>
import { onMounted } from "vue";
import { initialize } from "./game";
 
const props = defineProps({
  connection: Object,
});

const { connection } = props.connection;

onMounted(() => {
  initialize(connection)
});

</script>
