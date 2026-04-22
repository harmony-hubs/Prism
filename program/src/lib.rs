#![no_std]
    extern crate alloc;
    
    use ika_dwallet_pinocchio::{CPI_AUTHORITY_SEED, DWalletContext};
    use pinocchio::{entrypoint, error::ProgramError, AccountView, Address, ProgramResult};
    
    entrypoint!(process_instruction);
    pinocchio::nostd_panic_handler!();
    
    pub const ID: Address = Address::new_from_array([0u8; 32]);
    
    const INIT_HOLLOW: u8 = 0;
    const APPROVE_ACTION: u8 = 1;
    const TRANSFER_AUTHORITY: u8 = 2;
    
    const SIG_ED25519: u8 = 0;
    const SIG_SECP256K1: u8 = 1;
    const SIG_SECP256R1: u8 = 2;
    
    pub fn process_instruction(
        program_id: &Address,
        accounts: &[AccountView],
        instruction_data: &[u8],
    ) -> ProgramResult {
        if instruction_data.is_empty() {
            return Err(ProgramError::InvalidInstructionData);
        }
    
        match instruction_data[0] {
            INIT_HOLLOW => init_hollow(program_id, accounts, &instruction_data[1..]),
            APPROVE_ACTION => approve_action(program_id, accounts, &instruction_data[1..]),
            TRANSFER_AUTHORITY => transfer_authority(program_id, accounts, &instruction_data[1..]),
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
    
    fn build_ctx<'a>(
        program_id: &Address,
        dwallet_program: &'a AccountView,
        cpi_authority: &'a AccountView,
        caller_program: &'a AccountView,
    ) -> Result<(DWalletContext<'a>, Address, u8), ProgramError> {
        let (expected_cpi, bump) = Address::derive_program_address(&[CPI_AUTHORITY_SEED], program_id)
            .ok_or(ProgramError::InvalidAccountData)?;
    
        if *cpi_authority.address() != expected_cpi {
            return Err(ProgramError::InvalidAccountData);
        }
    
        let ctx = DWalletContext {
            dwallet_program,
            cpi_authority,
            caller_program,
            cpi_authority_bump: bump,
        };
    
        Ok((ctx, expected_cpi, bump))
    }
    
    fn init_hollow(
        program_id: &Address,
        accounts: &[AccountView],
        _data: &[u8],
    ) -> ProgramResult {
        if accounts.len() < 6 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
    
        let owner = &accounts[0];
        let dwallet_secp = &accounts[1];
        let dwallet_ed = &accounts[2];
        let dwallet_program = &accounts[3];
        let cpi_authority = &accounts[4];
        let caller_program = &accounts[5];
    
        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
    
        let (ctx, expected_cpi, _) = build_ctx(
            program_id, dwallet_program, cpi_authority, caller_program,
        )?;
    
        ctx.transfer_dwallet(dwallet_secp, expected_cpi.to_bytes())?;
        ctx.transfer_dwallet(dwallet_ed, expected_cpi.to_bytes())?;
    
        Ok(())
    }
    
    fn approve_action(
        program_id: &Address,
        accounts: &[AccountView],
        data: &[u8],
    ) -> ProgramResult {
        if accounts.len() < 8 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        if data.len() < 66 {
            return Err(ProgramError::InvalidInstructionData);
        }
    
        let owner = &accounts[0];
        let message_approval = &accounts[1];
        let dwallet = &accounts[2];
        let payer = &accounts[3];
        let system_program = &accounts[4];
        let dwallet_program = &accounts[5];
        let cpi_authority = &accounts[6];
        let caller_program = &accounts[7];
    
        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
    
        let (ctx, _, _) = build_ctx(
            program_id, dwallet_program, cpi_authority, caller_program,
        )?;
    
        let mut message_hash = [0u8; 32];
        message_hash.copy_from_slice(&data[0..32]);
    
        let mut user_pubkey = [0u8; 32];
        user_pubkey.copy_from_slice(&data[32..64]);
    
        let signature_scheme = data[64];
        let bump = data[65];
    
        match signature_scheme {
            SIG_ED25519 | SIG_SECP256K1 | SIG_SECP256R1 => {}
            _ => return Err(ProgramError::InvalidInstructionData),
        }
    
        ctx.approve_message(
            message_approval,
            dwallet,
            payer,
            system_program,
            message_hash,
            user_pubkey,
            signature_scheme,
            bump,
        )?;
    
        Ok(())
    }
    
    fn transfer_authority(
        program_id: &Address,
        accounts: &[AccountView],
        data: &[u8],
    ) -> ProgramResult {
        if accounts.len() < 5 {
            return Err(ProgramError::NotEnoughAccountKeys);
        }
        if data.len() < 32 {
            return Err(ProgramError::InvalidInstructionData);
        }
    
        let owner = &accounts[0];
        let dwallet = &accounts[1];
        let dwallet_program = &accounts[2];
        let cpi_authority = &accounts[3];
        let caller_program = &accounts[4];
    
        if !owner.is_signer() {
            return Err(ProgramError::MissingRequiredSignature);
        }
    
        let (ctx, _, _) = build_ctx(
            program_id, dwallet_program, cpi_authority, caller_program,
        )?;
    
        let mut new_authority = [0u8; 32];
        new_authority.copy_from_slice(&data[0..32]);
        ctx.transfer_dwallet(dwallet, new_authority)?;
    
        Ok(())
    }
    
