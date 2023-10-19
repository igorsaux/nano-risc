import { defineStore } from 'pinia'
import { VMStatus, type CodeError } from './vm'

export type VMState = {
  status: VMStatus
  pc: number
  registers: Array<string | number>
  errors: CodeError[]
}

export const useAppStore = defineStore({
  id: 'app',
  state: () => ({
    vm: {
      status: VMStatus.Idle,
      pc: 0,
      registers: [],
      errors: []
    } as VMState
  })
})
