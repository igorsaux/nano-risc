<script setup lang="ts">
import { useAppStore } from '@/appStore';

const store = useAppStore()
</script>

<template>
	<div class="flex flex-col gap-2">
		<h2 class="font-bold">Registers</h2>
		<div class="flex flex-wrap gap-1">
			<div v-for="(register, index) in store.vm.registers" v-bind:key="index">
				<span>R{{ index }}:</span> <span class="text-neutral-500">{{ register }}</span>
			</div>
		</div>
		<div class="flex gap-1">
			<span><span>PC:</span> <span class="text-neutral-500">{{ store.vm.pc }}</span></span>
			<span><span>SP:</span> <span class="text-neutral-500">{{ store.vm.sp }}</span></span>
		</div>
	</div>
</template>
