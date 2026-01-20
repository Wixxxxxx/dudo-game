// use crate::helpers;
// use crate::state::Game;
// use pinocchio::{account_info::AccountInfo, program_error::ProgramError};

// pub struct CreateGameAccounts<'a> {
//     pub host: &'a AccountInfo,
//     pub game: &'a AccountInfo,
//     pub mint: &'a AccountInfo,
//     pub vault: &'a AccountInfo,
// }

// impl<'a> TryFrom<&'a [AccountInfo]> for CreateGameAccounts<'a> {
//     type Error = ProgramError;

//     fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
//         let [host, game, mint, vault] = accounts else {
//             return Err(ProgramError::NotEnoughAccountKeys);
//         };

//         helpers::signer_check(host);

//         Ok(())
//     }
// }

// pub struct CreateGame<'a> {
//     pub accounts: CreateGameAccounts<'a>,
//     pub game: Game,
// }

// impl<'a> TryFrom<(&'a [u8], &'a [AccountInfo])> for CreateGame<'a> {
//     type Error = ProgramError;

//     fn try_from((data, accounts): (&'a [u8], &'a [AccountInfo])) -> Result<Self, Self::Error> {
//         let accounts = CreateGameAccounts::try_from(accounts)?;
//         let game = Game::try_from(data)?;

//         Ok(Self { STCF: Catch-upaccounts, game })
//     }
// }
