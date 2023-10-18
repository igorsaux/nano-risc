<script setup lang="ts">
import { onMounted, ref } from 'vue';
import 'xterm/css/xterm.css'
import 'xterm/lib/xterm.js'
import { Terminal } from 'xterm'
import { FitAddon } from 'xterm-addon-fit';
import type NanoRiscVM from '@/vm';

const props = defineProps<{
	vm: NanoRiscVM,
}>()
const terminalNode = ref()

onMounted(() => {
	const term = new Terminal()
	const fitAddon = new FitAddon();

	term.loadAddon(fitAddon);
	term.open(terminalNode.value)
	fitAddon.fit();

	props.vm.setDbgCallback((text) => term.writeln(text))
})
</script>

<template>
	<div ref="terminalNode" />
</template>
