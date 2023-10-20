<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue';
import 'xterm/css/xterm.css'
import 'xterm/lib/xterm.js'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit';
import type NanoRiscVM from '@/vm';

const props = defineProps<{
	vm: NanoRiscVM,
}>()
const terminalNode = ref()
let term = null as Terminal | null
let fitAddon = null as FitAddon | null

onMounted(() => {
	term = new Terminal()
	fitAddon = new FitAddon();

	term.loadAddon(fitAddon);
	term.open(terminalNode.value)
	fitAddon.fit();

	props.vm.setDbgCallback((text) => term?.writeln(text))

	window.addEventListener('resize', onWindowResize)
})

onUnmounted(() => {
	window.removeEventListener('resize', onWindowResize)
})

function onWindowResize() {
	fitAddon?.fit()
}
</script>

<template>
	<div ref="terminalNode" />
</template>
