use std::collections::BTreeSet;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedSet;
use near_sdk::store::UnorderedMap;
use near_sdk::{env, log, near_bindgen, require, AccountId, PanicOnDefault};
use tallystick::borda::{DefaultBordaTally, Variant};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct RankedChoiceVoting {
    candidates: UnorderedSet<String>,
    votes: UnorderedMap<AccountId, Vec<String>>,
}

#[near_bindgen]
impl RankedChoiceVoting {
    #[init]
    pub fn new(candidates: BTreeSet<String>) -> Self {
        require!(!env::state_exists(), "State already initialized");
        let mut candidate_set = UnorderedSet::new(b"c");
        candidate_set.extend(candidates);

        Self {
            candidates: candidate_set,
            votes: UnorderedMap::new(b"v"),
        }
    }

    /// Cast vote for the signer.
    pub fn vote(&mut self, order: Vec<String>) {
        let unique_votes: BTreeSet<_> = order.iter().collect();

        // Ensure no duplicates
        require!(unique_votes.len() == order.len());
        for v in unique_votes {
            // Assert that vote was for a valid candidate
            require!(self.candidates.contains(v), "invalid candidate");
        }

        self.votes.insert(env::signer_account_id(), order);
        log!("Successfully cast vote for {}", env::signer_account_id());
    }

    /// Returns current winner.
    pub fn get_winner(&self) -> Option<String> {
        let mut tally = DefaultBordaTally::new(1, Variant::Borda);
        for vote in self.votes.iter().map(|(_, v)| v) {
            tally.add_ref(vote).unwrap();
        }

        let winner = tally.winners().into_unranked();

        winner.into_iter().next()
    }

    /// Returns list of candidates. Not in any particular order.
    pub fn get_candidates(&self) -> Vec<String> {
        self.candidates.iter().collect()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    fn get_context(signer: AccountId) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer.clone())
            .predecessor_account_id(signer)
            .build()
    }

    fn init_contract() -> RankedChoiceVoting {
        let context = get_context("owner".parse().unwrap());
        testing_env!(context);
        let candidates: BTreeSet<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()]
            .into_iter()
            .collect();
        RankedChoiceVoting::new(candidates)
    }

    #[test]
    fn no_voters() {
        let contract = init_contract();
        assert!(contract.get_winner().is_none());
        assert_eq!(contract.get_candidates().len(), 3);
    }

    #[test]
    #[should_panic = "invalid candidate"]
    fn invalid_candidate() {
        let mut contract = init_contract();
        contract.vote(vec!["invalid".to_string(), "a".to_string()]);
    }

    #[test]
    fn multiple_voters() {
        let mut contract = init_contract();

        testing_env!(get_context("bob".parse().unwrap()));
        contract.vote(vec!["b".to_string(), "a".to_string(), "c".to_string()]);
        assert_eq!(contract.get_winner().unwrap(), "b");

        testing_env!(get_context("alice".parse().unwrap()));
        contract.vote(vec!["a".to_string(), "c".to_string(), "b".to_string()]);

        testing_env!(get_context("joe".parse().unwrap()));
        contract.vote(vec!["b".to_string(), "a".to_string(), "c".to_string()]);

        testing_env!(get_context("john".parse().unwrap()));
        contract.vote(vec!["a".to_string(), "c".to_string()]);

        testing_env!(get_context("jane".parse().unwrap()));
        contract.vote(vec!["b".to_string(), "a".to_string(), "c".to_string()]);

        assert_eq!(&contract.get_winner().unwrap(), "a");
    }
}
