<script setup lang="ts">
import type NanoRiscVM from '@/vm';
import RegistersList from './RegistersList.vue';
import { VMStatus } from '@/vm';

const props = defineProps<{
	vm: NanoRiscVM
}>()

function statusToString(status: VMStatus): string {
	if (status === VMStatus.Idle) {
		return "Idle"
	} else if (status === VMStatus.Running) {
		return "Running"
	} else if (status === VMStatus.Yield) {
		return "Yield"
	} else if (status === VMStatus.Finished) {
		return "Finished"
	}

	throw Error("Value of of range")
}
</script>

<template>
	<div class="p-2">
		<h2 class="font-bold text-xl">Information</h2>
		<span>
			<span>Status:</span> <span class="text-neutral-500">{{ statusToString(vm.status)
			}}</span>
		</span>
		<RegistersList :vm="props.vm" />
	</div>
</template>
