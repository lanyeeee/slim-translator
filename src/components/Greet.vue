<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

async function translateAsync() {
  try {
    const response: string = await invoke("translate", { from: "auto", to: "en", content: "你好\n今天的天气怎么样\n你开心吗" });
    console.log(response);
    greetMsg.value = response;
  } catch (e) {
    console.log(e);
    console.error(e);
    greetMsg.value = e as string;
  }
}

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>
  <button @click="translateAsync">Translate</button>

  <p>{{ greetMsg }}</p>
</template>
