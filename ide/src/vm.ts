import {
  vm_create,
  vm_set_dbg_callback,
  vm_load_assembly,
  vm_tick,
  vm_get_registers,
  vm_pc_to_location,
  vm_get_status,
  vm_reset,
  vm_get_pc,
  vm_get_sp
} from '../../web/pkg'
import { useAppStore } from './appStore'

export enum VMStatus {
  Idle = 0,
  Yield,
  Running,
  Finished,
  Error
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

export type RuntimeError = {
  message: string
}

export default class NanoRiscVM {
  __handle: number
  __dbgCallback?: (text: string) => void

  constructor() {
    this.__handle = vm_create()
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
    let status = this.status()
    this.reset()

    do {
      this.tick()
      status = this.status()
    } while (status != VMStatus.Finished && status != VMStatus.Idle)
  }

  tick(): RuntimeError {
    const error = vm_tick(this.__handle)
    this.__refreshData()

    if (error) {
      this.__dbgCallback?.call(undefined, `\x1b[31mRuntime error: ${error.message}\x1b[37m`)
    }

    return error
  }

  reset() {
    vm_reset(this.__handle)
    this.__refreshData()
  }

  status(): VMStatus {
    return vm_get_status(this.__handle)
  }

  __refreshData() {
    const store = useAppStore()

    store.$patch({
      vm: {
        status: this.status(),
        registers: vm_get_registers(this.__handle),
        pc: vm_get_pc(this.__handle),
        sp: vm_get_sp(this.__handle)
      }
    })
  }
}
