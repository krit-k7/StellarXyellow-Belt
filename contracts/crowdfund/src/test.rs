#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::Address as _,
    Address, Env, String,
};

fn create_env() -> Env {
    Env::default()
}

fn register_contract(env: &Env) -> Address {
    env.register_contract(None, CrowdfundContract)
}

fn make_string(env: &Env, s: &str) -> String {
    String::from_str(env, s)
}


// ─── init ────────────────────────────────────────────────────────────────────

#[test]
fn test_init() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);

    client.init();
    assert_eq!(client.get_count(), 0);
}

#[test]
fn test_double_init_resets_count() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);

    client.init();
    assert_eq!(client.get_count(), 0);

    client.init();
    assert_eq!(client.get_count(), 0);
}

// ─── create ──────────────────────────────────────────────────────────────────

#[test]
fn test_create_campaign() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    client.init();

    let creator = Address::generate(&env);
    let deadline: u64 = env.ledger().timestamp() + 86400; // 1 day from now

    let id = client.create(
        &creator,
        &make_string(&env, "Test Campaign"),
        &make_string(&env, "A test description"),
        &1_000_000_000i128, // 100 XLM in stroops
        &deadline,
    );

    assert_eq!(id, 0);
    assert_eq!(client.get_count(), 1);
}

#[test]
fn test_create_multiple_campaigns() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    client.init();

    let creator = Address::generate(&env);
    let deadline: u64 = env.ledger().timestamp() + 86400;

    let id0 = client.create(
        &creator,
        &make_string(&env, "Campaign 0"),
        &make_string(&env, "Desc 0"),
        &500_000_000i128,
        &deadline,
    );
    let id1 = client.create(
        &creator,
        &make_string(&env, "Campaign 1"),
        &make_string(&env, "Desc 1"),
        &1_000_000_000i128,
        &deadline,
    );

    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
    assert_eq!(client.get_count(), 2);
}

// ─── get_campaign ─────────────────────────────────────────────────────────────

#[test]
fn test_get_campaign() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    client.init();

    let creator = Address::generate(&env);
    let deadline: u64 = env.ledger().timestamp() + 86400;
    let target: i128 = 2_000_000_000;

    client.create(
        &creator,
        &make_string(&env, "School Fund"),
        &make_string(&env, "Build a school"),
        &target,
        &deadline,
    );

    let campaign = client.get_campaign(&0u32);
    assert_eq!(campaign.creator, creator);
    assert_eq!(campaign.target, target);
    assert_eq!(campaign.raised, 0);
    assert!(!campaign.claimed);
}

#[test]
#[should_panic(expected = "Campaign not found")]
fn test_get_nonexistent_campaign_panics() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    env.mock_all_auths();
    client.init();

    client.get_campaign(&99u32); // should panic
}

// ─── get_count ────────────────────────────────────────────────────────────────

#[test]
fn test_get_count_starts_at_zero() {
    let env = create_env();
    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    env.mock_all_auths();
    client.init();

    assert_eq!(client.get_count(), 0);
}

// ─── claim ────────────────────────────────────────────────────────────────────

#[test]
#[should_panic(expected = "Campaign still active")]
fn test_claim_before_deadline_panics() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    client.init();

    let creator = Address::generate(&env);
    let deadline: u64 = env.ledger().timestamp() + 86400;

    client.create(
        &creator,
        &make_string(&env, "Active Campaign"),
        &make_string(&env, "Still running"),
        &1_000_000_000i128,
        &deadline,
    );

    client.claim(&0u32); // should panic: campaign still active
}

#[test]
#[should_panic(expected = "Campaign not found")]
fn test_claim_nonexistent_panics() {
    let env = create_env();
    env.mock_all_auths();

    let contract_id = register_contract(&env);
    let client = CrowdfundContractClient::new(&env, &contract_id);
    client.init();

    client.claim(&0u32);
}
