<script setup>
import { onUpdated, ref } from "vue";

import ChatInput from "./ChatInput.vue";
import ChatMessage from "./ChatMessage.vue";
import ChatWrapper from "./ChatWrapper.vue";

const ws = new WebSocket("ws://localhost:2034?username=Vlad");

ws.onopen = () => {
  ws.send("Sss");
};

if (ws.OPEN) {
  // ws.send("message");
  ws.onmessage = (socket, message) => {
    console.log(message);
  };
}

const messages = ref([]);
const container = ref(null);

// FIXME: this must scroll the container to the last message
onUpdated(() => {
  const containerElement = container.value.$el;

  if (containerElement) {
    containerElement.scrollTop = containerElement.scrollHeight;
  }
});

const onSubmit = (author, text) => {
  messages.value.push({
    author,
    text: text.trim(),
    isAuthor: true
  });
};
</script>

<template>
  <ChatWrapper ref="container">
    <ChatMessage
      v-for="(message, index) in messages"
      :key="index"
      :author="message.author"
      :text="message.text"
      :is-author="message.isAuthor"
    />
  </ChatWrapper>
  <ChatInput @submit="onSubmit" />
</template>
