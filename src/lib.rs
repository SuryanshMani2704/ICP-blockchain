use ic_cdk::export::candid::{CandidType, Deserialize, Nat};
use ic_cdk_macros::{init, query, update};
use std::collections::HashMap;

type AccountId = String;

#[derive(CandidType, Deserialize, Clone)]
struct Token {
    symbol: String,
    name: String,
    total_supply: Nat,
}

#[derive(Default)]
struct State {
    token: Token,
    balances: HashMap<AccountId, Nat>,
}

static mut STATE: Option<State> = None;

#[init]
fn init(symbol: String, name: String, total_supply: Nat) {
    let token = Token { symbol, name, total_supply: total_supply.clone() };
    let mut balances = HashMap::new();
    balances.insert("owner".to_string(), total_supply);
    unsafe {
        STATE = Some(State { token, balances });
    }
}

#[update]
fn send(from: AccountId, to: AccountId, amount: Nat) -> Result<(), String> {
    let state = unsafe { STATE.as_mut().unwrap() };
    let from_balance = state.balances.entry(from.clone()).or_insert(Nat::from(0));
    if *from_balance < amount {
        return Err("Insufficient balance".to_string());
    }
    *from_balance -= amount.clone();
    let to_balance = state.balances.entry(to).or_insert(Nat::from(0));
    *to_balance += amount;
    Ok(())
}

#[query]
fn balance_of(account: AccountId) -> Nat {
    let state = unsafe { STATE.as_ref().unwrap() };
    state.balances.get(&account).cloned().unwrap_or_else(|| Nat::from(0))
}
