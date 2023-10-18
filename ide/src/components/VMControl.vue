<script setup lang="ts">
defineEmits(['vmRun', 'vmStep', 'vmStop', 'vmCompile'])
</script>

<template>
	<div class="flex gap-4 p-2">
		<h2 class="font-bold">VM Control</h2>

		<div class="flex gap-2">
			<button class="btn-primary" @click="$emit('vmCompile')">
				<div class="codicon codicon-gear" />Compile & Load
			</button>
			<button class="btn-primary" @click="$emit('vmRun')">
				<div class="codicon codicon-debug-start" />Run
			</button>
			<button class="btn-primary" @click="$emit('vmStep')">
				<div class="codicon codicon-debug-continue-small" />Step
			</button>
			<button class="btn-primary" @click="$emit('vmStop')">
				<div class="codicon codicon-debug-stop" />Stop
			</button>
		</div>
	</div>
</template>
