use super::sign_extend;
use crate::hardware::register::condition_flag;
use crate::hardware::register::Registers;

/// - ADD takes two values and stores them in a register.
/// - In register mode, the second value to add is found in a register.
/// - In immediate mode, the second value is embedded in the right-most 5 bits of the instruction.
/// - Values which are shorter than 16 bits need to be sign extended.
/// - Any time an instruction modifies a register, the condition flags need to be updated
/// If bit [5] is 0, the second source operand is obtained from SR2.
/// If bit [5] is 1, the second source operand is obtained by sign-extending the imm5 field to 16 bits.
/// In both cases, the second source operand is added to the contents of SR1 and the result stored in DR.
/// The condition codes are set, based on whether the result is negative, zero, or positive.

pub fn add(instr: u16, registers: &mut Registers) {
    /* destination register (DR) */
    let dr = (instr >> 9) & 0x7;
    /* first operand (SR1) */
    let sr1 = (instr >> 6) & 0x7;
    /* whether we are in immediate mode */
    let imm_flag = (instr >> 5) & 0x1;
    if imm_flag == 1 {
        let imm5 = sign_extend(instr & 0x1F, 5);
        //val is declared as u32 to prevent from overflow.
        let val: u32 = imm5 as u32 + registers.get(sr1) as u32;
        //val is declared as u16, so that type arithmatic kick in and number is rounded to get fit into u16.
        registers.update(dr, val as u16);
    } else {
        /* first operand (SR2) */
        let sr2 = instr & 0x7;
        let val: u32 = registers.get(sr1) as u32 + registers.get(sr2) as u32;
        registers.update(dr, val as u16);
    }
    condition_flag::update_r_cond_register(dr, registers);
}
