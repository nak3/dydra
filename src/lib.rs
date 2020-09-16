pub mod elf_loader;
pub mod emu_env;
pub mod instr_info;
pub mod riscv;
pub mod riscv_csr;
pub mod riscv_csr_def;
pub mod riscv_decoder;
pub mod riscv_inst_id;
pub mod tcg;
pub mod x86;

use crate::emu_env::EmuEnv;

pub fn run(filename: String, exp_gpr: &[u64; 32]) -> usize {
    let mut emu = EmuEnv::new();
    emu.run(&filename);
    let gpr_vec = emu.get_gpr();
    for (gpr_val, exp_val) in gpr_vec.iter().zip(exp_gpr.iter()) {
        if gpr_val != exp_val {
            print!("Failed: {:016x} != {:016x}\n", *gpr_val, *exp_val);
            return 1;
        }
    }
    return 0;
}

pub fn run_riscv_test(filename: String) -> u64 {
    let mut emu = EmuEnv::new();
    emu.run(&filename);
    return emu.get_mem(0x1000) as u64;
}