/// The core methods for a basic multi token. Extension standards may be
/// added in addition to this macro.
#[macro_export]
macro_rules! impl_multi_token_core {
    ($contract: ident, $token: ident) => {
        use $crate::multi_token::core::MultiTokenCore;
        use $crate::multi_token::core::MultiTokenResolver;

        #[near_bindgen]
        impl MultiTokenCore for $contract {
            #[payable]
            fn mt_transfer(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                amount: Balance,
                approval: Option<u64>,
            ) {
                self.$token.mt_transfer(receiver_id, token_id, amount, approval)
            }

            #[payable]
            fn mt_batch_transfer(
                &mut self,
                receiver_id: AccountId,
                token_ids: Vec<TokenId>,
                amounts: Vec<Balance>,
                approval: Vec<Option<u64>>,
            ) {
                self.$token.mt_batch_transfer(receiver_id, token_ids, amounts, approval)
            }

            #[payable]
            fn mt_batch_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_ids: TokenId,
                amounts: Balance,
                approval_ids: Vec<Option<u64>>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                self.$token.mt_batch_transfer_call(receiver_id, token_ids, amounts, approval_ids, msg)
            }

            #[payable]
            fn mt_transfer_call(
                &mut self,
                receiver_id: AccountId,
                token_id: TokenId,
                amount: Balance,
                approval_id: Option<u64>,
                msg: String,
            ) -> PromiseOrValue<bool> {
                self.$token.mt_transfer_call(receiver_id, token_id, amount, approval_id, msg)
            }

            fn mt_token(&self, token_id: TokenId) -> Option<Token> {
                self.$token.mt_token(token_id)
            }
            
            fn mt_balance_of(&self, owner: AccountId, id: Vec<TokenId>) -> Vec<u128> {
                self.$token.mt_balance_of(owner, id)
             }
            
            fn mt_approval_for_all(&mut self, owner_id: AccountId, approved: bool) { todo!() }
        }



        #[near_bindgen]
        impl MultiTokenResolver for $contract {
            #[private]
            fn mt_resolve_transfer(
                &mut self,
                sender_id: AccountId,
                receiver_id: AccountId,
                token_id: TokenId,
                amount: U128
            ) -> U128 {
                self.$token.mt_resolve_transfer(
                    sender_id,
                    receiver_id,
                    token_id,
                    amount
                )
            }
        }
    };
}

/// Multi token approval management allows for an escrow system where
/// multiple approvals per token exist.
#[macro_export]
macro_rules! impl_multi_token_approval {
    ($contract: ident, $token: ident) => {
        use $crate::multi_token::approval::MultiTokenApproval;

        #[near_bindgen]
        impl MultiTokenApproval for $contract {
            #[payable]
            fn mt_approve(
                &mut self,
                account_id: AccountId,
                token_id: TokenId,
                amount: Balance,
                msg: Option<String>,
            ) -> Option<Promise> {
                self.$token.mt_approve(account_id, token_id, amount, msg)
            }

            #[payable]
            fn mt_revoke(&mut self, token_id: TokenId, account_id: AccountId) {
                self.$token.mt_revoke(token_id, account_id)
            }

            #[payable]
            fn mt_revoke_all(&mut self, token_id: TokenId) {
                self.$token.mt_revoke_all(token_id)
            }

            fn mt_is_approved(
                &self,
                token_id: TokenId,
                approved_account_id: AccountId,
                amount: Balance,
                approval: Option<u64>,
            ) -> bool {
                self.$token.mt_is_approved(token_id, approved_account_id, amount, approval)
            }
        }
    };
}

/// Multi-token enumeration adds the extension standard offering several
/// view-only methods to get token supply, tokens per owner, etc.
#[macro_export]
macro_rules! impl_multi_token_enumeration {
    ($contract: ident, $token: ident) => {
        use $crate::multi_token::enumeration::MultiTokenEnumeration;

        #[near_bindgen]
        impl MultiTokenEnumeration for $contract {
            fn mt_tokens(&self, from_index: Option<u64>, limit: u64) -> Vec<Token> {
                self.$token.mt_tokens(from_index, limit)
            }

            fn mt_tokens_for_owner(&self, account_id: AccountId, from_index: Option<u64>, limit: u64) -> Vec<Token> {
                self.$token.mt_tokens_for_owner(account_id, from_index, limit)
            }
        }
    };
}