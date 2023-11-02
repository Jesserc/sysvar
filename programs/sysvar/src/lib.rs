use anchor_lang::prelude::*;

// replace program id
declare_id!("Et8abEv4sHEYXwjGf85o5hdQLTPqP8viVW5LmVeKZmoe");

#[program]
pub mod sysvar {

    use anchor_lang::solana_program::{epoch_rewards::EpochRewards, sysvar::recent_blockhashes::RecentBlockhashes};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Get the Clock sysvar
        let clock = Clock::get()?;
        msg!(
            "Block timestamp: {}",
            // get current timestamp from the Clock sysvar
            clock.unix_timestamp
        );

        // Get the EpochSchedule sysvar
        let epoch_schedule_var = EpochSchedule::get()?;
        let epoch = epoch_schedule_var.get_epoch(clock.slot * 1000000);
        msg!(
            "Current epoch: {:?}",
            // Our program will run on a local validator node,
            // therefore clock.slot would return a small value and epoch would also have small value.
            // we multiply the slot from Clock sysvar to get a larger epoch value
            epoch
        );

        // Get the Rent sysvar
        let rent_var = Rent::get()?;
        msg!(
            "Rent exemption threshold for an account: {} years",
            // The time duration (measured in years) that an account's balance must cover rent for,
            // in order for the account to be exempt from paying rent.
            // This is currently 2 years
            rent_var.exemption_threshold
        );

        // Accessing the RecentBlockhashes sysvar
        // This sysvar does not support the `get` method, so we'll access it via its public address
        // let arr = [ctx.accounts.slot_history.clone()];
        // let accounts_iter = &mut arr.iter();
        // let slot_history_sysvar_info = next_account_info(accounts_iter)?;
        // let sh_var = SlotHistory::from_account_info(slot_history_sysvar_info).unwrap();
        // msg!("Slot history: {:?}", sh_var);

        // let epoch_rewards_var = EpochRewards::get()?;
        // let data = stake_history.get(epoch).unwrap();

        // msg!("Stake history: {:?}", data.activating);
        // msg!("Epoch rewards: {:?}", epoch_rewards_var);


        // Accessing the RecentBlockhashes sysvar
        // This sysvar does not support the `get` method, so we'll access it via its public address
        let arr = [ctx.accounts.recent_blockhashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let recent_blockhashes = SlotHistory::from_account_info(sh_sysvar_info)?;
        // let data = recent_blockhashes.last().unwrap();

        // msg!("The recent block hash is: {:?}", data.blockhash);

        Ok(())
    }
}

// #[derive(Accounts)]
// pub struct Initialize<'info> {
//     /// CHECK: readonly
//     pub slot_history: AccountInfo<'info>,
// }


#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: readonly
    pub recent_blockhashes: AccountInfo<'info>,
}
/*

        The following sysvars support get:

            Clock
            EpochSchedule
            Fees
            Rent
            EpochRewards

        // DEPRECATED - can't be called with get()
        fees

    StakeHistory sysvar returns an empty value, I'm guessing this has to do with running on a local validator node

    // getting epoch rewards via the account call does not work cause anchor web3 client does not provide the sysvar account
    // however, i created a variable that stores the account with
    const epochRewardsPubkey = new anchor.web3.PublicKey(
    "SysvarEpochRewards1111111111111111111111111"
  );
    but this fails when i run the program
    I tried using the get method
    let epoch_rewards_var = EpochRewards::get()?;
    but got this error...
    Error: ELF error: ELF error: Unresolved symbol (sol_get_epoch_rewards_sysvar) at instruction #12494 (ELF file offset 0x18588)
    There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.

    im getting
        Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: Unsupported sysvar
    when i use the slothistory sysvar

*/

// let fees = Fees::get()?; // This fails
// let epoch_rewards = EpochRewards::get()?; // docs says it implements get but it doesn't in current anchor version

// let instruction = Instructions::from(value)

//
