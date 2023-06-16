use anchor_lang::prelude::*;

declare_id!("5trrJDcSRKB6eD46U2pDdugTPV3R3BZu4Y3sQjz1RYni");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        base_account.total_gifs = 0;
        Ok(())
    }

    pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let item = ItemStruct {
            gif_link: gif_link.to_string(),
            user_address: *user.to_account_info().key,
            vote_count: 0,
        };

        base_account.gif_list.push(item);
        base_account.total_gifs += 1;
        Ok(())
    }

    pub fn vote_for_gif(ctx: Context<VoteForGif>, gif_index: u32) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let _voter = &mut ctx.accounts.voter;

        let gif_index = gif_index as usize;
        let gif = &mut base_account.gif_list[gif_index];
        gif.vote_count += 1;

        Ok(())
    }

    pub fn get_vote_count(ctx: Context<GetVoteCount>, gif_index: u32) -> Result<u32> {
      let base_account = &ctx.accounts.base_account;
  
      let gif_index = gif_index as usize;
      let gif = &base_account.gif_list[gif_index];
  
      Ok(gif.vote_count)
  }
  
    #[derive(Accounts)]
    pub struct GetVoteCount<'info> {
        #[account(mut)]
        pub base_account: Account<'info, BaseAccount>,
    }

    #[derive(Accounts)]
    pub struct StartStuffOff<'info> {
        #[account(init, payer = user, space = 9000)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
        pub system_program: Program<'info, System>,
    }

    #[derive(Accounts)]
    pub struct AddGif<'info> {
        #[account(mut)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub user: Signer<'info>,
    }

    #[derive(Accounts)]
    pub struct VoteForGif<'info> {
        #[account(mut)]
        pub base_account: Account<'info, BaseAccount>,
        #[account(mut)]
        pub voter: Signer<'info>,
    }

    #[account]
    pub struct BaseAccount {
        pub total_gifs: u64,
        pub gif_list: Vec<ItemStruct>,
    }

    #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
    pub struct ItemStruct {
        pub gif_link: String,
        pub user_address: Pubkey,
        pub vote_count: u32,
    }
}
