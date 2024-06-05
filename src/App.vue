<script setup lang="ts">
import { listen } from "@tauri-apps/api/event"
import { onMounted, ref } from "vue"

const result = ref<string>("")

onMounted(async () => {
  await listen<string>("translate", (event) => {
    result.value = event.payload
  });
});

</script>

<template>
  <div class="h-full w-full min-h-screen flex">
    <div class="flex-1 p-2 m-2  bg-blueGray" data-tauri-drag-region>
      <span class="w-full whitespace-pre-line m-0 bg-white">{{ result }}</span>
    </div>
  </div>
</template>

<style>
body {
  margin: 0;
}
</style>