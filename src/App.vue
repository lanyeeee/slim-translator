<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getCurrent } from "@tauri-apps/api/window";

const result = ref<string>("");
const panel = getCurrent();

let pined = false;

function hide() {
  panel.hide();
  pined = false;
}

onMounted(async () => {
  await panel.listen<string>("translate", (event) => {
    result.value = event.payload;
  });

  window.addEventListener("keydown", (event) => {
    if (event.key == "Escape") {
      hide();
    }
  });

  document.body.addEventListener("mousedown", () => {
    pined = true
  });

  window.addEventListener("blur", () => {
    if (!pined) {
      hide();
    }
  });

});

</script>

<template>
  <div class="flex flex-col h-full w-full min-h-screen">
    <div class="flex-1 flex-col m-2  bg-blueGray" data-tauri-drag-region>
      <div class="flex justify-end" data-tauri-drag-region>
        <img src="./assets/close.svg" alt="close" @click="hide" />
      </div>
      <span class="whitespace-pre-line m-0 bg-white">
        {{ result }}
      </span>
    </div>
  </div>
</template>

<style>
body {
  margin: 0;
}
</style>