<script setup lang="ts">
import { useAppStore } from '@/appStore';
import type NanoRiscVM from '@/vm';
import * as monaco from 'monaco-editor'
import editorWorker from 'monaco-editor/esm/vs/editor/editor.worker?worker'
import { onMounted, ref, watchEffect } from 'vue';

self.MonacoEnvironment = {
  getWorker() {
    return new editorWorker()
  }
}

const props = defineProps<{
  vm: NanoRiscVM,
  content: string
}>()
const store = useAppStore()
const emits = defineEmits(['update:content'])
const editorNode = ref()

let editor: monaco.editor.IStandaloneCodeEditor | null = null
let decorations: monaco.editor.IEditorDecorationsCollection | null = null

store.$subscribe((mutation, state) => {
  const model = editor?.getModel()

  if (!model) {
    return
  }
  const newDecorations = [] as monaco.editor.IModelDeltaDecoration[]

  const errors = state.vm.errors.map(error => {
    const location = error.location;

    return {
      severity: monaco.MarkerSeverity.Error,
      message: error.message,
      startLineNumber: location.line,
      startColumn: location.column,
      endColumn: model.getLineMaxColumn(location.line),
      endLineNumber: location.line,
    }
  }) as monaco.editor.IMarkerData[];

  monaco.editor.setModelMarkers(model, '', errors)

  for (const error of errors) {
    newDecorations.push({
      range: {
        startLineNumber: error.startLineNumber,
        startColumn: error.startColumn,
        endLineNumber: error.endLineNumber,
        endColumn: 1
      },
      options: {
        isWholeLine: true,
        inlineClassName: 'errorDecoration'
      }
    })
  }

  const pcLocation = props.vm.pcLocation()

  if (pcLocation) {
    newDecorations.push({
      range: {
        startLineNumber: pcLocation.line,
        startColumn: pcLocation.column,
        endLineNumber: pcLocation.line,
        endColumn: 1
      },
      options: {
        isWholeLine: true,
        className: 'pcLine',
        glyphMargin: {
          position: monaco.editor.GlyphMarginLane.Left
        },
        glyphMarginClassName: 'pcLineGplyh'
      }
    })
  }

  decorations?.set(newDecorations)
})

watchEffect(() => {
  const model = editor?.getModel()

  if (!editor || !model) {
    return
  }

  model.setValue(props.content)
})

onMounted(() => {
  editor = monaco.editor.create(editorNode.value, {
    value: props.content,
    language: 'mips',
    theme: 'vs-dark',
    automaticLayout: true,
    glyphMargin: true,
  })

  editor.getModel()?.onDidChangeContent(() => {
    emits('update:content', editor?.getModel()?.getValue() ?? '')
  })

  decorations = editor.createDecorationsCollection([])
})
</script>

<template>
  <div ref="editorNode"></div>
</template>

<style>
.errorDecoration {
  background-color: rgba(255, 0, 0, 0.1) !important;
}

.pcLine {
  background-color: rgba(240, 191, 76, 0.1);
}

.pcLineGplyh::before {
  position: absolute;
  font-family: 'codicon';
  content: '';
  color: rgb(255, 255, 0);
}

.pcLineGplyh::after {
  position: absolute;
  font-family: 'codicon';
  content: '';
  color: rgb(255, 0, 0);
}
</style>
