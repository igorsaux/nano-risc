<script setup lang="ts">
import RegistersList from './RegistersList.vue';
import { VMStatus } from '@/vm';
import { useAppStore } from '../appStore'

const store = useAppStore()

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
			<span>Status:</span> <span class="text-neutral-500">{{ statusToString(store.vm.status)
			}}</span>
		</span>

		<RegistersList />
	</div>
</template>
