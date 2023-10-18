<script setup lang="ts">
import * as monaco from 'monaco-editor'
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
import { onMounted, ref, watchEffect } from 'vue';

self.MonacoEnvironment = {
  getWorker(_workedId, _label) {
    return new editorWorker()
  }
}

const props = defineProps<{
  content: string
}>()
const emits = defineEmits(['update:content'])
const editorNode = ref()

let editor: monaco.editor.IStandaloneCodeEditor | null = null

watchEffect(() => {
  editor?.getModel()?.setValue(props.content)
})

onMounted(() => {
  editor = monaco.editor.create(editorNode.value, {
    value: props.content,
    language: 'mips',
    theme: 'vs-dark',
    automaticLayout: true
  })
  editor.getModel()?.onDidChangeContent(() => {
    emits('update:content', editor?.getModel()?.getValue() ?? '')
  })
})
</script>

<template>
  <div class="min-h-[40ch]" ref="editorNode"></div>
</template>
