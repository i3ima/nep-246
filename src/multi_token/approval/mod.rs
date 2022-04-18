mod approval_impl;
mod receiver;

use std::collections::HashMap;
pub use approval_impl::*;
pub use receiver::*;

use crate::multi_token::token::{Approval, TokenId};
use near_sdk::{AccountId, Promise};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};

/// Limit for amount of approvals
/// See - https://github.com/shipsgold/NEPs/blob/master/specs/Standards/MultiToken/ApprovalManagement.md#why-must-mt_approve-panic-if-mt_revoke_all-would-fail-later
pub const MAX_APPROVALS_PER_TOKEN: usize = 99;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, BorshDeserialize, BorshSerialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TokenApproval {
    approval_owner_id: AccountId,
    approved_account_ids: HashMap<AccountId, Approval>,
}

/// Trait used in approval management
/// Specs - https://github.com/shipsgold/NEPs/blob/master/specs/Standards/MultiToken/ApprovalManagement.md
pub trait MultiTokenApproval {
    /// Add an approved account for a specific set of tokens.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of at least 1 yoctoⓃ for
    ///   security purposes
    /// * Contract MAY require caller to attach larger deposit, to cover cost of
    ///   storing approver data
    /// * Contract MUST panic if called by someone other than token owner
    /// * Contract MUST panic if addition would cause `mt_revoke_all` to exceed
    ///   single-block gas limit. See below for more info.
    /// * Contract MUST increment approval ID even if re-approving an account
    /// * If successfully approved or if had already been approved, and if `msg` is
    ///   present, contract MUST call `mt_on_approve` on `account_id`. See
    ///   `mt_on_approve` description below for details.
    ///
    /// # Arguments:
    /// * `token_ids`: the token ids for which to add an approval
    /// * `account_id`: the account to add to `approved_account_ids`
    /// * `amounts`: the number of tokens to approve for transfer, wrapped in quotes and treated
    ///    like an array of string, although the numbers will be stored as an array of
    ///    unsigned integer with 128 bits.
    ///
    /// * `msg`: optional string to be passed to `mt_on_approve`
    ///
    /// # Returns
    /// void, if no `msg` given. Otherwise, returns promise call to
    /// `mt_on_approve`, which can resolve with whatever it wants.
    fn mt_approve(
        &mut self,
        account_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: Option<String>,
    ) -> Option<Promise>;

    /// Revoke an approved account for a specific token.
    ///
    /// # Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
    ///   purposes
    /// * If contract requires >1yN deposit on `mt_approve`, contract
    ///   MUST refund associated storage deposit when owner revokes approval
    /// * Contract MUST panic if called by someone other than token owner
    ///
    /// # Arguments:
    /// * `token_ids`: the token for which to revoke approved_account_ids
    /// * `account_id`: the account to remove from `approvals`
    fn mt_revoke(&mut self, token_ids: Vec<TokenId>, account_id: AccountId);

    /// Revoke all approved accounts for a specific token.
    ///
    /// # Requirements:
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security
    ///   purposes
    /// * If contract requires >1yN deposit on `mt_approve`, contract
    ///   MUST refund all associated storage deposit when owner revokes approved_account_ids
    /// * Contract MUST panic if called by someone other than token owner
    ///
    /// # Arguments:
    /// * `token_ids`: the token ids with approved_account_ids to revoke
    fn mt_revoke_all(&mut self, token_ids: Vec<TokenId>);

    /// Check if tokens are approved for transfer by a given account, optionally
    /// checking an approval_id
    ///
    /// # Requirements:
    /// * Contract MUST panic if `approval_ids` is not null and the length of
    ///   `approval_ids` is not equal to `token_ids`
    ///
    /// # Arguments:
    /// * `token_ids`: the tokens for which to check an approval
    /// * `approved_account_id`: the account to check the existence of in `approved_account_ids`
    /// * `amounts`: specify the positionally corresponding amount for the `token_id`
    ///    that at least must be approved. The number of tokens to approve for transfer,
    ///    wrapped in quotes and treated like an array of string, although the numbers will be
    ///    stored as an array of unsigned integer with 128 bits.
    /// * `approval_ids`: an optional array of approval IDs to check against
    ///    current approval IDs for given account and `token_ids`.
    ///
    /// # Returns:
    /// if `approval_ids` is given, `true` if `approved_account_id` is approved with given `approval_id`
    /// and has at least the amount specified approved  otherwise, `true` if `approved_account_id`
    /// is in list of approved accounts and has at least the amount specified approved
    /// finally it returns false for all other states
    fn mt_is_approved(
        &self,
        token_ids: Vec<TokenId>,
        approved_account_id: AccountId,
        amounts: Vec<U128>,
        approval_ids: Option<Vec<u64>>,
    ) -> bool;

    /// Get a the list of approvals for a given token_id and account_id
    ///
    /// # Arguments:
    /// * `token_id`: the token for which to check an approval
    /// * `account_id`: the account to retrieve approvals for
    ///
    /// # Returns:
    /// A TokenApproval object, as described in Approval Management standard
    fn mt_token_approval(&self, token_id: TokenId, account_id: AccountId) -> TokenApproval;

    /// Get a list of all approvals for a given token_id
    ///
    /// # Arguments:
    /// * `from_index`: a string representing an unsigned 128-bit integer,
    ///    representing the starting index of tokens to return
    /// * `limit`: the maximum number of tokens to return
    ///
    /// # Returns:
    /// An array of TokenApproval objects, as described in Approval Management standard, and an empty array if there are no approvals
    fn mt_token_approvals(&self, token_id: TokenId, from_index: U128, limit: u128) -> Vec<TokenApproval>;
}
