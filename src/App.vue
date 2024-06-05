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
  <div class="container" style="height: 100vh; width: 100vh;" data-tauri-drag-region>
    <p style="white-space: pre-wrap">{{ result }}</p>
  </div>
</template>
