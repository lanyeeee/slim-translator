<script setup lang="ts">
import {ref} from "vue";
import {invoke} from "@tauri-apps/api/core";
import {commands, Config} from "../bindings.ts";

const greetMsg = ref("");
const name = ref("");
const config = ref<Config>();

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", {name: name.value});
}

async function getConfig() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  config.value = await commands.getConfig();
  console.log(config.value);
}

async function saveConfig() {
  if (!config.value) {
    return;
  }
  config.value.apiKey = "test";
  const res = await commands.saveConfig(config.value);
  console.log(res);
}

</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..."/>
    <button type="submit">Greet</button>
    <button @click="getConfig">Get Config</button>
    <button @click="saveConfig">Save Config</button>
  </form>

  <p>{{ greetMsg }}</p>
</template>
