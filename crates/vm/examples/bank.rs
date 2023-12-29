use {
    cw_bank::ExecuteMsg,
    cw_sdk::{Map, MockStorage, Order},
    cw_vm::{call_execute, db_read, db_remove, db_write, Host, InstanceBuilder},
    std::{env, ops::Bound, path::PathBuf},
};

const BALANCES: Map<(&str, &str), u64> = Map::new("b");

const INITIAL_BALANCES: &[(&str, &str, u64)] = &[
    ("alice",   "uatom", 100),
    ("alice",   "uosmo", 888),
    ("bob",     "uatom",  50),
    ("charlie", "uatom", 123),
];

fn build_initial_state() -> anyhow::Result<MockStorage> {
    let mut store = MockStorage::default();
    for (name, denom, balance) in INITIAL_BALANCES {
        BALANCES.save(&mut store, (name, denom), balance)?;
    }
    Ok(store)
}

fn main() -> anyhow::Result<()> {
    let wasm_file = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?)
        .join("../../target/wasm32-unknown-unknown/debug/cw_bank.wasm");
    let (instance, mut store) = InstanceBuilder::default()
        .with_wasm_file(wasm_file)?
        .with_host_state(build_initial_state()?)
        .with_host_function("db_read", db_read)?
        .with_host_function("db_write", db_write)?
        .with_host_function("db_remove", db_remove)?
        .finalize()?;
    let mut host = Host::new(&instance, &mut store);

    // make three transfers
    call_send(&mut host, "alice", "dave", "uatom", 75)?;
    call_send(&mut host, "bob", "charlie", "uatom", 50)?;
    call_send(&mut host, "charlie", "alice", "uatom", 69)?;

    // consume the wasm store, recover the host state it contains
    let state = store.into_data();
    dbg!(state);

    // end state:
    // ----------
    // alice:   100 - 75 + 69 = 94
    // bob:     50  - 50      = 0 (deleted from host state)
    // charlie: 123 + 50 - 69 = 104
    // dave:    0   + 75      = 75
    println!("Balances after the transfers:");
    // for item in BALANCES.range(&state, Bound::Unbounded, Bound::Unbounded, Order::Ascending) {
    //     let (name, balance) = item?;
    //     println!("name = {name}, balance = {balance}");
    // }

    Ok(())
}

fn call_send<T>(
    host:   &mut Host<T>,
    from:   &str,
    to:     &str,
    denom:  &str,
    amount: u64,
) -> anyhow::Result<()> {
    println!("Sending... from: {from}, to: {to}, denom: {denom}, amount: {amount}");

    let res = call_execute(host, &ExecuteMsg::Send {
        from:  from.into(),
        to:    to.into(),
        denom: denom.into(),
        amount,
    })?;

    println!("Contract response: {res:?}");

    Ok(())
}
