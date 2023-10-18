import {
  vm_create,
  vm_set_dbg_callback,
  vm_load_program,
  vm_tick,
  vm_get_registers,
  vm_reset,
  vm_get_pc
} from '../../web/pkg'

export enum VMStatus {
  Idle = 0,
  Yield,
  Running,
  Finished
}

export default class NanoRiscVM {
  __handle: number
  __dbgCallback?: (text: string) => void
  registers: Array<string | number>
  pc: number
  status: VMStatus

  constructor() {
    this.__handle = vm_create()
    this.registers = vm_get_registers(this.__handle)
    this.pc = vm_get_pc(this.__handle)
    this.status = VMStatus.Idle

    vm_set_dbg_callback(this.__handle, (text: string) => {
      this.__dbgCallback?.call(undefined, text)
    })
  }

  setDbgCallback(callback: (text: string) => void) {
    this.__dbgCallback = callback
  }

  loadProgram(code: string) {
    this.reset()
    vm_load_program(this.__handle, code)
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
    this.__refreshData()
    this.status = VMStatus.Idle
  }

  __refreshData() {
    this.registers = vm_get_registers(this.__handle)
    this.pc = vm_get_pc(this.__handle)
  }
}
