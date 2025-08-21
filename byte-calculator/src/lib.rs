use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, program_error::ProgramError, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    
    if instruction_data.len() < 3 {
        msg!("Need opcode + 2 operands");
        return Ok(());
    }

    let operation = instruction_data[0];
    let left = instruction_data[1] as i64;
    let right = instruction_data[2] as i64;

    let result = match operation {
        0 => {
            msg!("Addition: {} + {}", left, right);
            left + right
        },
        1 => {
            msg!("Subtraction: {} - {}", left, right);
            left - right
        },
        2 => {
            msg!("Multiplication: {} * {}", left, right);
            left * right
        },
        3 => {
            msg!("Division: {} / {}", left, right);
            if right != 0 {
                left / right
            } else {
                msg!("Division by zero is not allowed");
                return Err(ProgramError::InvalidInstructionData);
            }
        },
        4 => {
            msg!("Modulus: {} % {}", left, right);
            if right != 0 {
                left % right
            } else {
                msg!("Modulus by zero is not allowed");
                return Err(ProgramError::InvalidInstructionData);
            }
        },
        5 => {
            msg!("Power: {} ^ {}", left, right);
            if right >= 0 {
            left.pow(right as u32)               
            } else {
                msg!("Negative exponent is not allowed");
                return Err(ProgramError::InvalidInstructionData);
            }

        },
        _ =>{
            msg!("Unknown operation: {}", operation);
            return Err(ProgramError::InvalidInstructionData);

        }
    };

    msg!("Result = {}", result);
    Ok(())
}