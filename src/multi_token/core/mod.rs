/*! Multi-Token Implementation (ERC-1155)

 */

mod core_impl;
mod receiver;
mod resolver;

pub use self::core_impl::*;
pub use self::receiver::*;
pub use self::resolver::*;

pub type ApprovalId = u64;

use crate::multi_token::token::TokenId;
use near_sdk::{AccountId, Balance, PromiseOrValue};
use near_sdk::json_types::U128;

use super::token::Token;

/// Describes functionality according to this - https://eips.ethereum.org/EIPS/eip-1155
/// And this - <https://github.com/shipsgold/NEPs/blob/master/specs/Standards/MultiToken/Core.md>
pub trait MultiTokenCore {
    /// Make a single transfer
    ///
    /// # Arguments
    ///
    /// * `receiver_id`: Receiver of tokens
    /// * `token_id`: ID of token to send from
    /// * `amount`: How much to send
    ///
    /// returns: ()
    ///
    fn mt_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: Balance,
        approval: Option<u64>,
    );


    /// Simple batch transfer. Transfer a given `token_ids` from current owner to
    /// `receiver_id`
    /// # Arguments
    ///
    /// * `receiver_id`: the valid NEAR account receiving the token
    /// * `token_ids`: the tokens to transfer
    /// * `amounts`: the number of tokens to transfer, wrapped in quotes and treated
    ///    like an array of strings, although the numbers will be stored as an array of unsigned integer
    ///    with 128 bits.
    /// * `approval`: expected approval IDs per `token_ids`. If a `token_id` does
    ///    not have a corresponding approval id then the entry in the array must be marked null.
    ///    The `approval_ids` are numbers smaller than 2^5
    ///
    /// returns: ()
    ///
    fn mt_batch_transfer(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<Balance>,
        approvals: Vec<Option<u64>>);


    /// Transfer MT and call a method on receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MT
    /// contract at the method `resolve_transfer`.
    ///
    /// # Arguments
    ///
    /// * `receiver_id`: NEAR account receiving MT
    /// * `token_id`: Token to send
    /// * `amount`: How much to send
    /// * `approval_id`: ID of approval for signer
    /// * `memo`: Used as context
    /// * `msg`: Additional msg that will be passed to receiving contract
    ///
    /// returns: PromiseOrValue<bool>
    ///
    fn mt_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        amount: Balance,
        approval_id: Option<u64>,
        msg: String,
    ) -> PromiseOrValue<bool>;

    /// Transfer tokens and call a method on a receiver contract. A successful
    /// workflow will end in a success execution outcome to the callback on the MT
    /// contract at the method `mt_resolve_transfer`.
    ///
    /// You can think of this as being similar to attaching native NEAR tokens to a
    /// function call. It allows you to attach any Multi Token, token in a call to a
    /// receiver contract.
    ///
    /// # Arguments
    ///
    /// * `receiver_id`: NEAR account receiving MT
    /// * `token_ids`: Tokens to transfer
    /// * `amounts`: the number of tokens to transfer, wrapped in quotes and treated
    ///    like an array of string, although the numbers will be stored as an array of
    ///    unsigned integer with 128 bits.
    /// * `approval_ids`: expected approval IDs per `token_ids`. If a `token_id` does
    ///    not have a corresponding approval id then the entry in the array must be marked null.
    ///    The `approval_ids` are numbers smaller than 2^53, and therefore representable as JSON.
    ///    ApprovalId See Approval Management standard for full explanation.
    /// * `memo`: Used as context
    /// * `msg`: Additional msg that will be passed to receiving contract
    ///
    /// returns: PromiseOrValue<bool>
    ///
    fn mt_batch_transfer_call(
        &mut self,
        receiver_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        approval_ids: Vec<Option<u64>>,
        msg: String,
    ) -> PromiseOrValue<bool>;


    fn mt_approval_for_all(&mut self, owner: AccountId, approved: bool);

    /// Get balance of user in specified tokens
    ///
    /// # Arguments
    ///
    /// * `owner`: Account to check
    /// # `id`: Vector of token IDs
    fn mt_balance_of(&self, owner: AccountId, id: Vec<TokenId>) -> Vec<u128>;


    /// Get all info about token
    fn mt_token(&self, token_id: TokenId) -> Option<Token>;
}
