<script setup>
import { ref } from 'vue';
import { ai_workshop_hf_backend } from 'declarations/ai_workshop_hf_backend/index';
let translatedText = ref('');

async function handleSubmit(e) {
  e.preventDefault();
  const target = e.target;
  const tralslation = target.querySelector('#tralslation').value;
  await ai_workshop_hf_backend.translate_req(tralslation).then((response) => {
    translatedText.value = response;
  });
}
</script>

<template>
  <main>
    <img src="/logo2.svg" alt="DFINITY logo" />
    <br />
    <br />
    <form action="#" @submit="handleSubmit">
      <label for="tralslation">Enter your text to tralslate: &nbsp;</label>
      <input id="tralslation" alt="tralslation" type="text" />
      <button type="submit">Click Me!</button>
    </form>
    <section id="translatedText">{{ translatedText }}</section>
  </main>
</template>
