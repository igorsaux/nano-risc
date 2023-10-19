import {
  vm_create,
  vm_set_dbg_callback,
  vm_load_assembly,
  vm_tick,
  vm_get_registers,
  vm_pc_to_location,
  vm_reset,
  vm_get_pc
} from '../../web/pkg'
import { useAppStore } from './appStore'

export enum VMStatus {
  Idle = 0,
  Yield,
  Running,
  Finished
}

export type Limits = {
  regularRegisters: number
  pins: number
}

export type Location = {
  line: number
  column: number
  offset: number
}

export type CodeError = {
  message: string
  location: Location
}

export default class NanoRiscVM {
  __handle: number
  __dbgCallback?: (text: string) => void
  registers: Array<string | number>
  pc: number
  status: VMStatus

  constructor(limits: Limits | null) {
    this.__handle = vm_create(limits)
    this.registers = vm_get_registers(this.__handle)
    this.pc = vm_get_pc(this.__handle)
    this.status = VMStatus.Idle

    this.__refreshData()

    vm_set_dbg_callback(this.__handle, (text: string) => {
      this.__dbgCallback?.call(undefined, text)
    })
  }

  setDbgCallback(callback: (text: string) => void) {
    this.__dbgCallback = callback
  }

  loadProgram(code: string) {
    this.reset()

    const store = useAppStore()
    const error = vm_load_assembly(this.__handle, code) as CodeError | null

    if (error) {
      store.vm.errors = [error]
    } else {
      store.vm.errors = []
    }

    this.__refreshData()
  }

  pcLocation(): Location | null {
    return vm_pc_to_location(this.__handle)
  }

  run() {
    this.reset()

    do {
      this.tick()
    } while (this.status != VMStatus.Finished && this.status != VMStatus.Idle)
  }

  tick(): VMStatus {
    this.status = vm_tick(this.__handle)
    this.__refreshData()

    return this.status
  }

  reset() {
    vm_reset(this.__handle)
    this.status = VMStatus.Idle
    this.__refreshData()
  }

  __refreshData() {
    const store = useAppStore()

    store.$patch({
      vm: {
        status: this.status,
        registers: vm_get_registers(this.__handle),
        pc: vm_get_pc(this.__handle)
      }
    })
  }
}
