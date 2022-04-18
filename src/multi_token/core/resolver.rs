use crate::multi_token::token::TokenId;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use crate::multi_token::core::ApprovalId;

//// `resolve_transfer` will be called after `on_transfer`
pub trait MultiTokenResolver {
    /// Finalize an `mt_transfer_call` or `mt_batch_transfer_call` chain of cross-contract calls. Generically
    /// referred to as `mt_transfer_call` as it applies to `mt_batch_transfer_call` as well.
    ///
    /// The `mt_transfer_call` process:
    ///
    /// 1. Sender calls `mt_transfer_call` on MT contract
    /// 2. MT contract transfers token from sender to receiver
    /// 3. MT contract calls `mt_on_transfer` on receiver contract
    /// 4+. [receiver contract may make other cross-contract calls]
    /// N. MT contract resolves promise chain with `mt_resolve_transfer`, and may
    ///    transfer token back to sender
    ///
    /// Requirements:
    /// * Contract MUST forbid calls to this function by any account except self
    /// * If promise chain failed, contract MUST revert token transfer
    /// * If promise chain resolves with `true`, contract MUST return token to
    ///   `sender_id`
    ///
    /// Arguments:
    /// * `sender_id`: the sender of `mt_transfer_call`
    /// * `receiver_id`: the `receiver_id` argument given to `mt_transfer_call`
    /// * `token_ids`: the `token_ids` argument given to `mt_transfer_call`
    /// * `amounts`: the `token_ids` argument given to `mt_transfer_call`
    /// * `approvals (optional)`: if using Approval Management, contract MUST provide
    ///   set of original approvals in this argument, and restore the
    ///   approved accounts in case of revert.
    ///   `approvals` is an array of expected `approval_list` per `token_ids`.
    ///   If a `token_id` does not have a corresponding `approvals_list` then the entry in the
    ///   array must be marked null.
    ///   `approvals_list` is an array of triplets of [`owner_id`,`approval_id`,`amount`].
    ///   `owner_id` is the valid Near account that owns the tokens.
    ///   `approval_id` is the expected approval ID. A number smaller than
    ///    2^53, and therefore representable as JSON. See Approval Management
    ///    standard for full explanation.
    ///   `amount`: the number of tokens to transfer, wrapped in quotes and treated
    ///    like a string, although the number will be stored as an unsigned integer
    ///    with 128 bits.
    ///
    ///
    ///
    /// Returns total amount spent by the `receiver_id`, corresponding to the `token_id`.
    /// The amounts returned, though wrapped in quotes and treated like strings,
    /// the numbers will be stored as an unsigned integer with 128 bits.
    /// Example: if sender_id calls `mt_transfer_call({ "amounts": ["100"], token_ids: ["55"], receiver_id: "games" })`,
    /// but `receiver_id` only uses 80, `mt_on_transfer` will resolve with `["20"]`, and `mt_resolve_transfer`
    /// will return `["80"]`.

    fn mt_resolve_transfer(
        &mut self,
        sender_id: AccountId,
        receiver: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        approvals: Option<Vec<(AccountId, ApprovalId, U128)>>,
    ) -> Vec<U128>;
}
