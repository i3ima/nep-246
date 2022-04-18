use std::collections::HashMap;
use near_sdk::{assert_one_yocto, env, ext_contract, AccountId, Balance, Promise, require};
use near_sdk::json_types::U128;


use crate::multi_token::{
    core::{MultiToken, GAS_FOR_MT_TRANSFER_CALL},
    token::{Approval, TokenId},
    utils::{bytes_for_approved_account_id, expect_extension, refund_deposit, Entity, unauthorized_assert},
};
use crate::multi_token::approval::{MAX_APPROVALS_PER_TOKEN, TokenApproval};

use super::MultiTokenApproval;

const NO_DEPOSIT: Balance = 0;

#[ext_contract(ext_approval_receiver)]
pub trait MultiTokenReceiver {
    fn mt_on_approve(&mut self,
                     token_ids: Vec<TokenId>,
                     amounts: Vec<U128>,
                     owner_id: AccountId,
                     approval_ids: Vec<u64>,
                     msg: String);
}

impl MultiToken {
    fn internal_approve(&mut self, account_id: &AccountId, token_id: TokenId, amount: &Balance) -> Approval {
        // Unwrap to check if approval supported
        let approvals_by_id = expect_extension(self.approvals_by_id.as_mut(), Entity::Token);
        let approvals_number_by_id = self.approvals_number_by_id.as_mut().unwrap();

        let approvals_number = approvals_number_by_id.get(&token_id).unwrap_or_default();

        // Check for approvals limit
        assert!(approvals_number + 1 < MAX_APPROVALS_PER_TOKEN, "Token reached approvals limit");

        // Get owner & caller
        let owner_id = self.owner_by_id.get(&token_id).expect("This token does not exist");

        // Check if caller is authorized
        unauthorized_assert(&owner_id);

        // Get the balance to check if user have enough tokens
        let balance = self.balances_per_token.get(&token_id).unwrap().get(&owner_id).unwrap_or(0);

        require!(&balance >= amount, "Not enough balance to approve");

        // Get some IDs and check if approval management supported both for contract & token
        let next_id = expect_extension(self.next_approval_id_by_id.as_mut(), Entity::Token);
        let mut current_next_id =
            expect_extension(next_id.get(&token_id), Entity::Token);

        let new_approval = Approval { amount: amount.to_owned(), approval_id: current_next_id };
        env::log_str(format!("New approval: {:?}", new_approval).as_str());

        // Get approvals for this token
        let approvals = &mut approvals_by_id.get(&token_id).unwrap_or_default();
        let old_approval_id = approvals.insert(account_id.clone(), new_approval.clone());

        // Update count
        let old_approvals_number = approvals_number_by_id.get(&token_id).unwrap();
        approvals_number_by_id.insert(&token_id, &(old_approvals_number + 1));

        approvals_by_id.insert(&token_id, approvals);

        env::log_str(format!("Updated approvals by id: {:?}", old_approval_id).as_str());

        let used_storage =
            if old_approval_id.is_none() { bytes_for_approved_account_id(&account_id) } else { 0 };

        refund_deposit(used_storage);

        current_next_id += 1;

        new_approval
    }

    fn internal_revoke(&mut self, token_id: TokenId, account_id: &AccountId) {
        // It's impossible that token does not have owner, so i'll just unwrap the value
        let owner = self.owner_by_id.get(&token_id).unwrap();

        unauthorized_assert(&owner);

        // Get all approvals for token, will panic if approval extension is not used for contract or token
        let approvals = expect_extension(self.approvals_by_id.as_mut(), Entity::Contract);
        let mut approvals_by_token = expect_extension(approvals.get(&token_id), Entity::Token);

        let approvals_number = self.approvals_number_by_id.as_mut().unwrap();
        let old_number = approvals_number.get(&token_id).unwrap_or_default();

        // Remove approval for user & also clean map to save space it it's empty
        approvals_by_token.remove(account_id);
        approvals_number.insert(&token_id, &(old_number - 1));

        if approvals_by_token.is_empty() {
            approvals.remove(&token_id);
        }
    }
}

impl MultiTokenApproval for MultiToken {
    fn mt_approve(
        &mut self,
        account_id: AccountId,
        token_ids: Vec<TokenId>,
        amounts: Vec<U128>,
        msg: Option<String>,
    ) -> Option<Promise> {
        assert_one_yocto();

        let amounts_to: Vec<Balance> = amounts.iter().map(|a| a.0).collect();

        let approval_ids: Vec<u64> = token_ids.clone().into_iter().enumerate().map(|(id, token_id)|
            self.internal_approve(&account_id, token_id, &amounts_to[id]).approval_id
        ).collect();

        // Check if msg present and then call `mt_on_approve`
        msg.and_then(|msg| {
            Some(ext_approval_receiver::mt_on_approve(
                token_ids,
                amounts,
                account_id.clone(),
                approval_ids,
                msg,
                account_id,
                NO_DEPOSIT,
                env::prepaid_gas() - GAS_FOR_MT_TRANSFER_CALL,
            ))
        })
    }

    fn mt_revoke(&mut self, token_ids: Vec<TokenId>, account_id: AccountId) {
        assert_one_yocto();

        token_ids.into_iter()
            .for_each(|token_id| self.internal_revoke(token_id, &account_id));
    }

    fn mt_revoke_all(&mut self, token: Vec<String>) {
        todo!()
    }

    fn mt_is_approved(
        &self,
        token_ids: Vec<String>,
        approved_account_id: AccountId,
        amounts: Vec<U128>,
        approval_ids: Option<Vec<u64>>,
    ) -> bool {
        let approvals = expect_extension(self.approvals_by_id.as_ref(), Entity::Contract);

        let amounts_to: Vec<u128> = amounts.iter().map(|a| a.0).collect();

        let results: Vec<bool> = token_ids.into_iter().enumerate().map(|(idx, token_id)| {
            let by_token = approvals.get(&token_id).unwrap_or_default();

            match by_token.get(&approved_account_id) {
                Some(approve) => {
                    let approval_id = approval_ids.as_ref().unwrap().get(idx);

                    if approve.amount.eq(&amounts_to[idx]) {
                        match approval_id {
                            Some(approval) => approve.approval_id.eq(approval),
                            None => true,
                        }
                    } else {
                        false
                    }
                }
                None => {return false}
            }
        }).collect();

        results.contains(&false)

    }

    fn mt_token_approval(&self, token_id: TokenId, account_id: AccountId) -> TokenApproval {
        let approvals = expect_extension(self.approvals_by_id.as_ref(), Entity::Contract);
        let by_token = expect_extension(approvals.get(&token_id), Entity::Token);
        let by_account: (AccountId, Approval) = by_token.into_iter().find(|(account, _)| account == &account_id)
            .expect("This account does not have approvals in this token");
        let owner = self.owner_by_id.get(&token_id).unwrap();

        TokenApproval {
            approval_owner_id: owner,
            approved_account_ids: HashMap::from([by_account])
        }
    }

    fn mt_token_approvals(&self, token_id: TokenId, from_index: U128, limit: u128) -> Vec<TokenApproval> {
        let approvals = expect_extension(self.approvals_by_id.as_ref(), Entity::Contract);
        let owner = self.owner_by_id.get(&token_id).unwrap();
        approvals.get(&token_id).unwrap().into_iter().skip(from_index.0 as usize).take(limit as usize)
            .map(|approval| {
                TokenApproval {
                    approval_owner_id: owner.clone(),
                    approved_account_ids: HashMap::from([approval]),
                }
            }).collect()
    }
}
