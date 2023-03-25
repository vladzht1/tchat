<script setup>
import { ref, defineEmits, onMounted } from "vue";

const emit = defineEmits(["submit"]);

const text = ref("");
const inputRef = ref(null);

onMounted(() => {
  inputRef.value.focus();
});

const clearInput = () => {
  text.value = "";
};

const onKeyPress = (event) => {
  if (event.key === "Enter" && event.shiftKey) {
    event.preventDefault();
    onFormSubmit(event);
  }
};

const shouldSubmitForm = () => {
  return text.value.trim().length != 0;
};

const onFormSubmit = (event) => {
  event.preventDefault();

  if (!shouldSubmitForm()) {
    return;
  }

  emit("submit", "Vlad", text.value);

  clearInput();
};
</script>

<template>
  <form @submit="onFormSubmit" class="block w-full">
    <textarea
      ref="inputRef"
      v-model="text"
      @keypress="onKeyPress"
      class="block w-full border border-gray-500 rounded-md p-1.5 text-base resize-y focus:outline-none focus:border-gray-700 focus:drop-shadow-md focus:shadow-gray-600"
    ></textarea>
    <small class="text-sm text-zinc-800"> Press Shift+Enter to send a message </small>
  </form>
</template>

<style scoped></style>
