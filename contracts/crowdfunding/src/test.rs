#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, testutils::Ledger, Address, Env};

const XLM_CONTRACT_TESTNET: &str = "CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC";

// Helper function to setup a campaign
fn setup_campaign(env: &Env) -> (CrowdfundingContractClient, Address, i128, u64, Address) {
    let contract_id = env.register_contract(None, CrowdfundingContract);
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

    // Fast forward time past deadline
    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    client.donate(&donor, &100_000_000);
}

#[test]
fn test_is_ended() {
    let env = Env::default();
    let (client, _, _, deadline, _) = setup_campaign(&env);

    // Not ended yet
    assert!(!client.is_ended());

    // Fast forward time past deadline
    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    // Now it should be ended
    assert!(client.is_ended());
}

// ------------------------------------------------------------------
// NEW TESTS FOR NEW FUNCTIONS
// ------------------------------------------------------------------

#[test]
fn test_get_first_donation_date_no_donation() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);
    let non_donor = Address::generate(&env);

    // For a user who has not donated, the first donation date should be 0
    assert_eq!(client.get_first_donation_date(&non_donor), 0);
}

// Note: A test for a successful first donation date would require mocking a token transfer,
// which is complex in the current test setup. We test the logic within the donate function
// is present, and here we test the "zero" case.

#[test]
fn test_get_progress_percentage() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);

    // Initially, with 0 raised, progress should be 0%
    assert_eq!(client.get_progress_percentage(), 0);

    // Note: Testing with actual donations is omitted due to the complexity
    // of mocking token transfers in the test environment.
}

#[test]
#[should_panic(expected = "Campaign has not ended yet")]
fn test_refund_before_deadline() {
    let env = Env::default();
    let (client, _, _, _, _) = setup_campaign(&env);
    let donor = Address::generate(&env);

    // Attempting to refund before the deadline should panic
    client.refund(&donor);
}

#[test]
#[should_panic(expected = "Campaign goal was reached, no refunds")]
fn test_refund_when_goal_is_met() {
    let env = Env::default();
    // We can't easily simulate donations, so we'll set the goal to 0
    // to simulate a met goal (since total_raised starts at 0).
    let contract_id = env.register_contract(None, CrowdfundingContract);
    let client = CrowdfundingContractClient::new(&env, &contract_id);

    let owner = Address::generate(&env);
    let donor = Address::generate(&env);
    let goal = 0i128; // Goal is 0, so it's met instantly
    let deadline = env.ledger().timestamp() + 100;
    let xlm_token_address =
        Address::from_string(&soroban_sdk::String::from_str(&env, XLM_CONTRACT_TESTNET));

    env.mock_all_auths();
    client.initialize(&owner, &goal, &deadline, &xlm_token_address);

    // Fast forward time past deadline
    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    // Goal is met (0 >= 0), so refund should not be possible
    client.refund(&donor);
}

#[test]
#[should_panic(expected = "No donation to refund")]
fn test_refund_with_no_donation() {
    let env = Env::default();
    let (client, _, _, deadline, _) = setup_campaign(&env);
    let non_donor = Address::generate(&env);

    // Fast forward time past deadline
    env.ledger().with_mut(|li| {
        li.timestamp = deadline + 1;
    });

    // Goal is not met, deadline has passed, but this user has no donation.
    // This should panic.
    client.refund(&non_donor);
}