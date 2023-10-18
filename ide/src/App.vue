<script setup lang="ts">
import { ref } from 'vue';

import exampleProgram from '../../factorial.asm?raw'
import CodeEditor from './components/CodeEditor.vue';
import TheTerminal from './components/TheTerminal.vue';
import InfoPanel from './components/InfoPanel.vue';
import VMControl from './components/VMControl.vue';

import NanoRiscVM from './vm';

const sourceCode = ref(exampleProgram)
const vm = ref(new NanoRiscVM())
</script>

<template>
  <div class="flex h-full w-full">
    <div class="flex flex-col w-full h-full">
      <VMControl @vm-step="vm.tick()" @vm-compile="vm.loadProgram(sourceCode)" @vm-run="vm.run()" @vm-stop="vm.reset()" />

      <div class="flex flex-col w-full h-full">
        <CodeEditor v-model:content="sourceCode" />
        <TheTerminal :vm="vm" />
      </div>
    </div>

    <InfoPanel :vm="vm" />
  </div>
</template>

<style scoped>
</style>
