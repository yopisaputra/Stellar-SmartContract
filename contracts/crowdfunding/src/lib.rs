#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, token, Address, Env, Map, Symbol, Vec};

// --- Kunci Storage --- 
const CAMPAIGN_GOAL: Symbol = symbol_short!("goal");
const CAMPAIGN_DEADLINE: Symbol = symbol_short!("deadline");
const TOTAL_RAISED: Symbol = symbol_short!("raised");
const DONATIONS: Symbol = symbol_short!("donations");
const CAMPAIGN_OWNER: Symbol = symbol_short!("owner");
const XLM_TOKEN_ADDRESS: Symbol = symbol_short!("xlm_addr");
const IS_ALREADY_INIT: Symbol = symbol_short!("is_init");
const HISTORY: Symbol = symbol_short!("history");
const LEADERBOARD: Symbol = symbol_short!("leaderbrd");
const STREAKS: Symbol = symbol_short!("streaks");

// --- Konstanta Konfigurasi ---
const LEADERBOARD_SIZE: u32 = 3; // Menampilkan 3 donatur teratas
const ONE_DAY_SECONDS: u64 = 86400; // 60 detik * 60 menit * 24 jam

// --- Struct Data --- 

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DonationRecord {
    pub donor: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopDonor {
    pub donor: Address,
    pub total_donation: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StreakInfo {
    pub last_donation_day: u64,
    pub streak_days: u32,
}

#[contract]
pub struct CrowdfundingContract;

#[contractimpl]
impl CrowdfundingContract {

    pub fn initialize(
        env: Env,
        owner: Address,
        goal: i128,
        deadline: u64,
        xlm_token: Address,
    ) {
        owner.require_auth();

        env.storage().instance().set(&CAMPAIGN_OWNER, &owner);
        env.storage().instance().set(&CAMPAIGN_GOAL, &goal);
        env.storage().instance().set(&CAMPAIGN_DEADLINE, &deadline);
        env.storage().instance().set(&TOTAL_RAISED, &0i128);
        env.storage().instance().set(&XLM_TOKEN_ADDRESS, &xlm_token);
        env.storage().instance().set(&IS_ALREADY_INIT, &true);
        env.storage().instance().set(&DONATIONS, &Map::<Address, i128>::new(&env));
        env.storage().instance().set(&HISTORY, &Vec::<DonationRecord>::new(&env));
        env.storage().instance().set(&LEADERBOARD, &Vec::<TopDonor>::new(&env));
        env.storage().instance().set(&STREAKS, &Map::<Address, StreakInfo>::new(&env));
    }

    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();

        if env.ledger().timestamp() > env.storage().instance().get(&CAMPAIGN_DEADLINE).unwrap() {
            panic!("Campaign has ended");
        }
        if amount <= 0 {
            panic!("Donation amount must be positive");
        }

        let xlm_token_address: Address = env.storage().instance().get(&XLM_TOKEN_ADDRESS).unwrap();
        token::Client::new(&env, &xlm_token_address).transfer(&donor, &env.current_contract_address(), &amount);

        let mut total_raised: i128 = env.storage().instance().get(&TOTAL_RAISED).unwrap();
        total_raised += amount;
        env.storage().instance().set(&TOTAL_RAISED, &total_raised);

        let mut donations: Map<Address, i128> = env.storage().instance().get(&DONATIONS).unwrap();
        let total_donor_donation = donations.get(donor.clone()).unwrap_or(0) + amount;
        donations.set(donor.clone(), total_donor_donation);
        env.storage().instance().set(&DONATIONS, &donations);

        let mut history: Vec<DonationRecord> = env.storage().instance().get(&HISTORY).unwrap();
        history.push_back(DonationRecord { donor: donor.clone(), amount, timestamp: env.ledger().timestamp() });
        env.storage().instance().set(&HISTORY, &history);

        Self::update_leaderboard(&env, donor.clone(), total_donor_donation);
        Self::update_streak(&env, donor);
    }

    fn update_leaderboard(env: &Env, donor: Address, total_donation: i128) {
        let mut leaderboard: Vec<TopDonor> = env.storage().instance().get(&LEADERBOARD).unwrap();
        
        if let Some(index) = leaderboard.iter().position(|d| d.donor == donor) {
            leaderboard.remove(index as u32);
        }

        let mut insert_pos = leaderboard.len();
        for (i, top_donor) in leaderboard.iter().enumerate() {
            if total_donation > top_donor.total_donation {
                insert_pos = i as u32;
                break;
            }
        }

        if insert_pos < LEADERBOARD_SIZE {
            leaderboard.insert(insert_pos, TopDonor { donor, total_donation });
            if leaderboard.len() > LEADERBOARD_SIZE {
                leaderboard.pop_back();
            }
        }

        env.storage().instance().set(&LEADERBOARD, &leaderboard);
    }

    fn update_streak(env: &Env, donor: Address) {
        let mut streaks: Map<Address, StreakInfo> = env.storage().instance().get(&STREAKS).unwrap();
        let current_day = env.ledger().timestamp() / ONE_DAY_SECONDS;

        let mut streak_info = streaks.get(donor.clone()).unwrap_or(StreakInfo {
            last_donation_day: 0,
            streak_days: 0,
        });

        if streak_info.last_donation_day > 0 {
            if current_day == streak_info.last_donation_day + 1 {
                streak_info.streak_days += 1;
            } else if current_day > streak_info.last_donation_day {
                streak_info.streak_days = 1;
            }
        } else {
            streak_info.streak_days = 1;
        }

        streak_info.last_donation_day = current_day;
        streaks.set(donor, streak_info);
        env.storage().instance().set(&STREAKS, &streaks);
    }

    pub fn refund(env: Env, donor: Address) {
        donor.require_auth();

        let deadline: u64 = env.storage().instance().get(&CAMPAIGN_DEADLINE).unwrap();
        let goal: i128 = env.storage().instance().get(&CAMPAIGN_GOAL).unwrap();
        let total_raised: i128 = env.storage().instance().get(&TOTAL_RAISED).unwrap();

        if env.ledger().timestamp() <= deadline {
            panic!("Campaign has not ended yet");
        }
        if total_raised >= goal {
            panic!("Campaign goal was reached, no refunds");
        }

        let mut donations: Map<Address, i128> = env.storage().instance().get(&DONATIONS).unwrap();
        let donation_amount = donations.get(donor.clone()).unwrap_or(0);

        if donation_amount <= 0 {
            panic!("No donation to refund");
        }

        donations.set(donor.clone(), 0);
        env.storage().instance().set(&DONATIONS, &donations);
        env.storage().instance().set(&TOTAL_RAISED, &(total_raised - donation_amount));

        let xlm_token_address: Address = env.storage().instance().get(&XLM_TOKEN_ADDRESS).unwrap();
        let xlm_token = token::Client::new(&env, &xlm_token_address);
        xlm_token.transfer(&env.current_contract_address(), &donor, &donation_amount);
    }

    pub fn get_leaderboard(env: Env) -> Vec<TopDonor> {
        env.storage().instance().get(&LEADERBOARD).unwrap_or(Vec::new(&env))
    }

    pub fn get_streak_info(env: Env, donor: Address) -> StreakInfo {
        let streaks: Map<Address, StreakInfo> = env.storage().instance().get(&STREAKS).unwrap();
        streaks.get(donor).unwrap_or(StreakInfo { last_donation_day: 0, streak_days: 0 })
    }

    pub fn get_donation_history(env: Env) -> Vec<DonationRecord> {
        env.storage().instance().get(&HISTORY).unwrap_or(Vec::new(&env))
    }

    pub fn get_total_raised(env: Env) -> i128 {
        env.storage().instance().get(&TOTAL_RAISED).unwrap_or(0)
    }

    pub fn get_donation(env: Env, donor: Address) -> i128 {
        let donations: Map<Address, i128> = env.storage().instance().get(&DONATIONS).unwrap();
        donations.get(donor).unwrap_or(0)
    }

    pub fn get_is_already_init(env: Env) -> bool {
        env.storage().instance().get(&IS_ALREADY_INIT).unwrap_or(false)
    }

    pub fn get_goal(env: Env) -> i128 {
        env.storage().instance().get(&CAMPAIGN_GOAL).unwrap_or(0)
    }

    pub fn get_deadline(env: Env) -> u64 {
        env.storage().instance().get(&CAMPAIGN_DEADLINE).unwrap_or(0)
    }

    pub fn is_goal_reached(env: Env) -> bool {
        let total_raised: i128 = env.storage().instance().get(&TOTAL_RAISED).unwrap_or(0);
        let goal: i128 = env.storage().instance().get(&CAMPAIGN_GOAL).unwrap_or(0);
        total_raised >= goal
    }

    pub fn is_ended(env: Env) -> bool {
        let deadline: u64 = env.storage().instance().get(&CAMPAIGN_DEADLINE).unwrap_or(0);
        env.ledger().timestamp() > deadline
    }

    pub fn get_progress_percentage(env: Env) -> i128 {
        let total_raised: i128 = env.storage().instance().get(&TOTAL_RAISED).unwrap_or(0);
        let goal: i128 = env.storage().instance().get(&CAMPAIGN_GOAL).unwrap_or(0);
        if goal == 0 {
            return 0;
        }
        (total_raised * 100) / goal
    }
}

#[cfg(test)]
mod test;
