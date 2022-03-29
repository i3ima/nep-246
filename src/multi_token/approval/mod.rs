mod approval_impl;
mod receiver;

pub use approval_impl::*;
pub use receiver::*;

use crate::multi_token::token::TokenId;
use near_sdk::{AccountId, Balance, Promise};

/// Trait used in approval management
/// Specs - https://github.com/shipsgold/NEPs/blob/master/specs/Standards/MultiToken/ApprovalManagement.md
pub trait MultiTokenApproval {
    /// Add an approved account for a specific set of tokens
    fn mt_approve(
        &mut self,
        account_id: AccountId,
        token_id: TokenId,
        amount: Balance,
        msg: Option<String>
    ) -> Option<Promise>;

    /// Revoke an approve for specific account
    fn mt_revoke(&mut self, token: TokenId, account: AccountId);

    /// Revoke all approves for a token
    fn mt_revoke_all(&mut self, token: TokenId);

    /// Check if account has access to transfer tokens
    fn mt_is_approved(
        &self,
        token: TokenId,
        approved_account: AccountId,
        amount: Balance,
        approval: Option<u64>,
    ) -> bool;
}
