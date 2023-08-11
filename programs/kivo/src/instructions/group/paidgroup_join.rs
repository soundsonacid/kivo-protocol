use anchor_lang::{
    prelude::*,
    InstructionData,
    solana_program::{
        system_program,
        native_token::LAMPORTS_PER_SOL,
        instruction::Instruction,
    },
};
use anchor_spl::token::*;
use crate::{
    state::{
        user::User,
        group::PaidGroup,
        group::Invite,
        group::Membership,
    },
    error::KivoError,
    constants::{ INVITE, MEMBERSHIP },
    instruction::HandlePgroupMakepayment,
};
use clockwork_cron::Schedule;
use std::str::FromStr;

pub fn process(ctx: Context<PaidGroupJoin>, schedule: Option<String>) -> Result<()> {

    require!(ctx.accounts.group.num_members < 24, KivoError::TooManyGroupMembers);

    if ctx.accounts.group.recurring {

        let instruction = Instruction {
            program_id: crate::ID,
            accounts: vec![
                AccountMeta::new(ctx.accounts.new_member.key(), false),
                AccountMeta::new(ctx.accounts.new_member_token_account.key(), false),
                AccountMeta::new(ctx.accounts.group_or_admin_token_account.key(), false),
                AccountMeta::new(ctx.accounts.group.key(), false),
                AccountMeta::new_readonly(ctx.accounts.thread_program.as_ref().unwrap().key(), false),
                AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
            ],
            data: HandlePgroupMakepayment.data(),
        };

        let schedule = Schedule::from_str(&schedule.unwrap()).unwrap();

        let trigger = clockwork_sdk::state::Trigger::Cron {
            schedule: schedule.to_string(),
            skippable: false,
        };

        clockwork_sdk::cpi::thread_create(
            CpiContext::new(
                ctx.accounts.thread_program.as_ref().unwrap().to_account_info(),
                clockwork_sdk::cpi::ThreadCreate {
                    authority: ctx.accounts.payer.to_account_info(),
                    payer: ctx.accounts.payer.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    thread: ctx.accounts.thread.as_ref().unwrap().to_account_info(),
                }
            ),
            LAMPORTS_PER_SOL / 100 as u64,
            ctx.accounts.membership.key().to_bytes().to_vec(),
            vec![instruction.into()],
            trigger,
        )?;

    } else {
        let bump = User::get_user_address(ctx.accounts.new_member.key()).1;
        let key = &ctx.accounts.new_member.key();
        let signature_seeds = User::get_user_signer_seeds(key, &bump);
        let signer_seeds = &[&signature_seeds[..]];
        transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.new_member_token_account.to_account_info(),
                    to: ctx.accounts.group_or_admin_token_account.to_account_info(),
                    authority: ctx.accounts.new_member.to_account_info(),
                },
                signer_seeds,
            ),
            ctx.accounts.group.fee,
        )?;
    }

    ctx.accounts.membership.new(
        ctx.accounts.new_member.key(),
        ctx.accounts.group.key(),
    )?;

    ctx.accounts.group.increment_members();

    Ok(())
}

#[derive(Accounts)]
pub struct PaidGroupJoin<'info> {
    #[account(address = User::get_user_address(payer.key()).0)]
    pub new_member: Box<Account<'info, User>>,

    #[account(
        seeds = [
            INVITE,
            new_member.key().as_ref(),
            group.key().as_ref(),
        ],
        bump
    )]
    pub invite: Box<Account<'info, Invite>>,

    #[account(mut, address = membership.group)]
    pub group: Box<Account<'info, PaidGroup>>,

    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<Membership>(),
        seeds = [
            MEMBERSHIP,
            new_member.key().as_ref(),
            invite.group.as_ref(),
        ],
        bump
    )]
    pub membership: Box<Account<'info, Membership>>,

    #[account(mut, associated_token::authority = group.admin, associated_token::mint = mint)]
    pub group_or_admin_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut, associated_token::authority = new_member, associated_token::mint = mint)]
    pub new_member_token_account: Box<Account<'info, TokenAccount>>,

    pub mint: Box<Account<'info, Mint>>,

    /// CHECK: Thread initialized via CPI
    #[account(mut, address = clockwork_sdk::state::Thread::pubkey(membership.key(), group.group_id.to_le_bytes().to_vec()))]
    pub thread: Option<UncheckedAccount<'info>>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub thread_program: Option<Program<'info, clockwork_sdk::ThreadProgram>>,

    #[account(address = system_program::ID)]
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
}