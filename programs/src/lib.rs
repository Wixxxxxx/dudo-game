// pub mod instructions;
// pub use instructions::*;

// pub mod helpers;

// pub mod state;
// pub use state::*;

// use pinocchio::{
//     ProgramResult, account_info::AccountInfo, entrypoint, msg, program_error::INCORRECT_PROGRAM_ID,
//     pubkey::Pubkey,
// };
// entrypoint!(process_instruction);

// pub fn process_instruction(
//     _program_id: &Pubkey,
//     accounts: &[AccountInfo],
//     instruction_data: &[u8],
// ) -> ProgramResult {
//     match instruction_data.split_first() {
//         Some(Instruction1::DISCRIMINATOR, data)
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
