<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';

import exampleProgram from '../../factorial.asm?raw'
import CodeEditor from './components/CodeEditor.vue';
import TheTerminal from './components/TheTerminal.vue';
import InfoPanel from './components/InfoPanel.vue';
import VMControl from './components/VMControl.vue';

import NanoRiscVM from './vm';

const sourceCode = ref(exampleProgram)
const vm = ref(new NanoRiscVM(null))

function loadProgram() {
  if (!vm.value) {
    return
  }

  vm.value.loadProgram(sourceCode.value)
}

onMounted(() => {
  window.addEventListener('resize', onWindowResize)

  onWindowResize()
})

onUnmounted(() => {
  window.removeEventListener('resize', onWindowResize)
})

function onWindowResize() {
  const terminal = document.querySelector('#terminal')
  const codeEditor = document.querySelector('#codeEditor')
  const vmControl = document.querySelector('#vmControl')

  const termRect = terminal!.getBoundingClientRect()
  const vmControlRect = vmControl!.getBoundingClientRect()

  codeEditor?.setAttribute('style', `height: ${window.innerHeight - vmControlRect.height - termRect.height}px;`)
}
</script>

<template>
  <div class="flex h-full w-full">
    <div class="flex flex-col w-full h-full">
      <VMControl id="vmControl" @vm-step="vm.tick()" @vm-compile="loadProgram" @vm-run="vm.run()" @vm-stop="vm.reset()" />

      <div class="codeContainer w-full h-full">
        <CodeEditor id="codeEditor" :vm="vm" v-model:content="sourceCode" />
        <TheTerminal id="terminal" :vm="vm" />
      </div>
    </div>

    <InfoPanel />
  </div>
</template>

<style scoped>
.codeContainer {
  display: flex;
  flex-direction: column;
}
</style>
