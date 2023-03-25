<script setup>
import { computed } from "vue";

const props = defineProps({
  author: String,
  text: String,
  isAuthor: Boolean
});

const getClassName = computed(() => {
  return "message--wrapper " + (props.isAuthor ? "author" : "");
});
</script>

<template>
  <div :class="getClassName">
    <div class="message--author">
      {{ props.author }}
    </div>
    <div class="message--text">
      {{ props.text }}
    </div>
  </div>
</template>

<style scoped>
.message--wrapper {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  border: 1px solid black;
  border-radius: 5px;
  width: 100%;
  padding: 5px 12px;
}

.message--wrapper.author {
  background-color: rgba(48, 207, 255, 0.2);
}

.message--author {
  font-weight: bold;
  font-size: 1.1rem;
  line-height: 1.1;
}

.message--text {
  line-height: 1.1;
}
</style>
