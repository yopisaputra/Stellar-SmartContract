#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, Env, Vec};

const XLM_CONTRACT_TESTNET: &str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";

// Helper function to setup a campaign
fn setup_campaign(env: &Env) -> (CrowdfundingContractClient<'_>, Address, i128, u64, Address) {
    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(env, &contract_id);

    let owner = Address::generate(env);
    let goal = 1000_000_000i128; // 100 XLM
    let deadline = env.ledger().timestamp() + 86400; // 24 hours from now
    let xlm_token_address =
        Address::from_string(&soroban_sdk::String::from_str(env, XLM_CONTRACT_TESTNET));

    env.mock_all_auths();

    client.initialize(&owner, &goal, &deadline, &xlm_token_address);

    (client, owner, goal, deadline, xlm_token_address)
}


#[test]
fn test_initialize_campaign() {
    let env = Env::default();
    let (client, _, goal, deadline, _) = setup_campaign(&env);

    assert_eq!(client.get_total_raised(), 0);
    assert_eq!(client.get_goal(), goal);
    assert_eq!(client.get_deadline(), deadline);
    assert!(client.get_is_already_init());
}

#[test]
#[should_panic(expected = "Donation amount must be positive")]
fn test_donate_zero_amount() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);
    let donor = Address::generate(&env);
    client.donate(&donor, &0);
}

#[test]
#[should_panic(expected = "Campaign has ended")]
fn test_donate_after_deadline() {
    let env = Env::default();
    let (client, _, _, deadline, _) = setup_campaign(&env);
    let donor = Address::generate(&env);

    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    client.donate(&donor, &100_000_000);
}

#[test]
fn test_is_ended() {
    let env = Env::default();
    let (client, _, _, deadline, _) = setup_campaign(&env);

    assert!(!client.is_ended());

    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    assert!(client.is_ended());
}

// ------------------------------------------------------------------
// TESTS FOR NEW AND UPDATED FEATURES
// ------------------------------------------------------------------

#[test]
fn test_get_donation_history_is_initially_empty() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);

    let history: Vec<DonationRecord> = client.get_donation_history();
    assert_eq!(history.len(), 0);
}

#[test]
fn test_leaderboard_is_initially_empty() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);

    let leaderboard: Vec<TopDonor> = client.get_leaderboard();
    assert_eq!(leaderboard.len(), 0);
}

#[test]
fn test_streak_info_is_initially_zero() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);
    let new_donor = Address::generate(&env);

    let streak_info: StreakInfo = client.get_streak_info(&new_donor);
    assert_eq!(streak_info.last_donation_day, 0);
    assert_eq!(streak_info.streak_days, 0);
}


#[test]
fn test_get_progress_percentage() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);

    assert_eq!(client.get_progress_percentage(), 0);
}

#[test]
#[should_panic(expected = "Campaign has not ended yet")]
fn test_refund_before_deadline() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);
    let donor = Address::generate(&env);

    client.refund(&donor);
}

#[test]
#[should_panic(expected = "Campaign goal was reached, no refunds")]
fn test_refund_when_goal_is_met() {
    let env = Env::default();
    let contract_id = env.register(CrowdfundingContract, ());
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let donor = Address::generate(&env);
    let goal = 0i128;
    let deadline = env.ledger().timestamp() + 100;
    let xlm_token_address =
        Address::from_string(&soroban_sdk::String::from_str(&env, XLM_CONTRACT_TESTNET));

    env.mock_all_auths();
    client.initialize(&owner, &goal, &deadline, &xlm_token_address);

    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    client.refund(&donor);
}

#[test]
#[should_panic(expected = "No donation to refund")]
fn test_refund_with_no_donation() {
    let env = Env::default();
    let (client, _, _, deadline, _) = setup_campaign(&env);
    let non_donor = Address::generate(&env);

    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    client.refund(&non_donor);
}
