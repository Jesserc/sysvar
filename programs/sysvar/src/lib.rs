use anchor_lang::prelude::*;

// replace program id
declare_id!("9J5NWojHW5keznzYwvCDmxeJZXGoeFpZdpcg3uMiLb6i");

#[program]
pub mod sysvar {

    use anchor_lang::solana_program::{
        epoch_rewards::EpochRewards,
        last_restart_slot::LastRestartSlot,
        sysvar::{fees::Fees, recent_blockhashes::RecentBlockhashes},
    };

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

        // RecentBlockhashes sysvar
        // This sysvar does not support the `get` method, so we'll access it via its public address
        // let arr = [ctx.accounts.recent_blockhashes.clone()];
        // let accounts_iter = &mut arr.iter();
        // let sh_sysvar_info = next_account_info(accounts_iter)?;
        // let recent_blockhashes = RecentBlockhashes::from_account_info(sh_sysvar_info)?;
        // let data = recent_blockhashes.last().unwrap();

        // msg!("The recent block hash is: {:?}", data.blockhash);

        // SlotHashes sysvar
        let arr = [ctx.accounts.slot_hashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let slot_var = SlotHashes::from_account_info(sh_sysvar_info)?;
        let data = slot_var;
        msg!("The recent slot hash is: {:?}", data);

        // msg!("Slot hashes: {:?}", slh_var.get(&clock.slot));

        // SlotHistory sysvar
        // This sysvar does not support the `get` method, so we'll access it via its public address
        // let arr = [ctx.accounts.slot_history.clone()];
        // let accounts_iter = &mut arr.iter();
        // let slot_history_sysvar_info = next_account_info(accounts_iter)?;
        // let slh_var = SlotHistory::from_account_info(slot_history_sysvar_info).unwrap();
        // msg!("Slot history: {:?}", slh_var);

        /*
        // Get the EpochRewards sysvar
        // I get this error when I try to use this
        // Error: ELF error: ELF error: Unresolved symbol (sol_get_epoch_rewards_sysvar) at instruction #12494 (ELF file offset 0x18588)
        // There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.
        // let epoch_rewards_var = EpochRewards::get()?;


        // Get Fees sysvar
        let fees_sysvar = Fees::get()?;
        msg!("Fees: {:?}", fees_sysvar);
        I get this error
        Error: ELF error: ELF error: Unresolved symbol (sol_get_fees_sysvar) at instruction #13708 (ELF file offset 0x1ab78)
        There was a problem deploying: Output { status: ExitStatus(unix_wait_status(256)), stdout: "", stderr: "" }.

        SlotHistory::get() fails with
            Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: Unsupported sysvar
        SlotHistory::from_account_info(slot_history_sysvar_info)?; fails with
                Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: Unsupported sysvar

        StakeHistory sysvar returns an empty value, I'm guessing this has to do with running on a local validator node
        Instructions sysvar can't be called with both the get method and the <Sysvar>::from_account_info. It doesn't support both method, I saw somewhere that it is more complex to implement, it'll be the last thing i'll look into

        // SlotHashes sysvar does not support the get method, so i use from_account_info()
        let arr = [ctx.accounts.slot_hashes.clone()];
        let accounts_iter = &mut arr.iter();
        let sh_sysvar_info = next_account_info(accounts_iter)?;
        let recent_blockhashes = SlotHashes::from_account_info(sh_sysvar_info)?;
        let data = recent_blockhashes.last().unwrap();
        msg!("The recent slot hash is: {:?}", data.1);
        but it fails with
        Error: failed to send transaction: Transaction simulation failed: Error processing Instruction 0: Unsupported sysvar

        The Anchor web3 client does not have support for the LastRestartSlot sysvar
         */

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: readonly
    pub slot_hashes: AccountInfo<'info>,
    // CHECK: readonly
    // pub recent_blockhashes: AccountInfo<'info>,
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
