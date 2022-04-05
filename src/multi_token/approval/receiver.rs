use crate::multi_token::token::TokenId;
use near_sdk::AccountId;
use near_sdk::json_types::U128;

/// Approval receiver is the trait for the method called (or attempted to be called) when an MT contract adds an approval for an account.
pub trait MultiTokenApprovalReceiver {
    /// Respond to notification that contract has been granted approval for a token.
    ///
    /// # Notes:
    /// * Contract knows the token contract ID from `predecessor_account_id`
    ///
    /// # Arguments:
    /// * `token_ids`: the token_ids to which this contract has been granted approval
    /// * `amounts`: the ositionally corresponding amount for the token_id
    ///    that at must be approved. The number of tokens to approve for transfer,
    ///    wrapped in quotes and treated like an array of string, although the numbers will be
    ///    stored as an array of unsigned integer with 128 bits.
    /// * `owner_id`: the owner of the token
    /// * `approval_ids`: the approval ID stored by NFT contract for this approval.
    ///    Expected to be a number within the 2^53 limit representable by JSON.
    /// * `msg`: specifies information needed by the approved contract in order to
    ///    handle the approval. Can indicate both a function to call and the
    ///    parameters to pass to that function.
    fn mt_on_approve(
        &mut self,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        owner_id: AccountId,
        approval_ids: Vec<u64>,
        msg: String,
    ) -> near_sdk::PromiseOrValue<String>;
}
