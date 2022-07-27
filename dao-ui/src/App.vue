<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const text = ref<string>("");

const setClipboard = async () => {
  await invoke("set_clipboard_text", { text: text.value });
};

const getClipboard = async () => {
  const val: string = await invoke("get_clipboard_text");
  text.value = val;
};
</script>

<template>
  <textarea v-model="text" name="text" cols="30" rows="10"></textarea>
  <div style="display: flex">
    <button @click="getClipboard" style="background-color: green">
      Get clipboard
    </button>
    <button @click="setClipboard" style="background-color: red">
      Set clipboard
    </button>
  </div>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
}
.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
