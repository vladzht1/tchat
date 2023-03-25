<script setup></script>

<template>
  <div class="chat--wrapper">
    <slot></slot>
  </div>
</template>

<style scoped>
.chat--wrapper {
  height: 70vh;
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding: 5px;
  border: 1px solid gray;
  border-radius: 5px;
  overflow-y: scroll;
  overflow-x: hidden;
}
</style>
