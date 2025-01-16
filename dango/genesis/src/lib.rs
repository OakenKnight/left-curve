use {
    dango_types::{
        account_factory::{self, AccountType, NewUserSalt, Username},
        auth::Key,
        bank,
        config::{AppAddresses, AppConfig},
        ibc,
        lending::{self, MarketUpdates},
        oracle::{
            self, GuardianSet, PriceSource, ETH_USD_ID, GUARDIANS_ADDRESSES, GUARDIAN_SETS_INDEX,
            USDC_USD_ID, WBTC_USD_ID,
        },
        orderbook, taxman, vesting,
    },
    grug::{
        btree_map, btree_set, Addr, Binary, Coins, Config, ContractBuilder, ContractWrapper, Denom,
        Duration, GenesisState, Hash160, Hash256, HashExt, Inner, JsonSerExt, Message, Permission,
        Permissions, ResultExt, StdResult, Udec128, GENESIS_SENDER,
    },
    hyperlane_types::{hooks, isms, mailbox, recipients::warp},
    serde::Serialize,
    std::{collections::BTreeMap, error::Error, fs, io, path::Path, str::FromStr},
};

pub type GenesisUsers = BTreeMap<Username, GenesisUser>;

pub type Addresses = BTreeMap<Username, Addr>;

#[grug::derive(Serde)]
pub struct Contracts {
    pub account_factory: Addr,
    pub bank: Addr,
    pub hyperlane: Hyperlane<Addr>,
    pub ibc_transfer: Addr,
    pub lending: Addr,
    pub oracle: Addr,
    pub orderbook: Addr,
    pub taxman: Addr,
    pub vesting: Addr,
}

#[derive(Clone, Copy)]
pub struct Codes<T> {
    pub account_factory: T,
    pub account_margin: T,
    pub account_safe: T,
    pub account_spot: T,
    pub bank: T,
    pub hyperlane: Hyperlane<T>,
    pub ibc_transfer: T,
    pub lending: T,
    pub oracle: T,
    pub orderbook: T,
    pub taxman: T,
    pub vesting: T,
}

#[grug::derive(Serde)]
#[derive(Copy)]
pub struct Hyperlane<T> {
    pub fee: T,
    pub ism: T,
    pub mailbox: T,
    pub merkle: T,
    pub warp: T,
}

pub struct GenesisUser {
    pub key: Key,
    pub key_hash: Hash256,
    pub balances: Coins,
}

pub fn build_rust_codes() -> Codes<ContractWrapper> {
    let account_factory = ContractBuilder::new(Box::new(dango_account_factory::instantiate))
        .with_execute(Box::new(dango_account_factory::execute))
        .with_query(Box::new(dango_account_factory::query))
        .with_authenticate(Box::new(dango_account_factory::authenticate))
        .build();

    let account_margin = ContractBuilder::new(Box::new(dango_account_margin::instantiate))
        .with_authenticate(Box::new(dango_account_margin::authenticate))
        .with_backrun(Box::new(dango_account_margin::backrun))
        .with_receive(Box::new(dango_account_margin::receive))
        .with_query(Box::new(dango_account_margin::query))
        .build();

    let account_safe = ContractBuilder::new(Box::new(dango_account_safe::instantiate))
        .with_authenticate(Box::new(dango_account_safe::authenticate))
        .with_receive(Box::new(dango_account_safe::receive))
        .with_execute(Box::new(dango_account_safe::execute))
        .with_query(Box::new(dango_account_safe::query))
        .build();

    let account_spot = ContractBuilder::new(Box::new(dango_account_spot::instantiate))
        .with_authenticate(Box::new(dango_account_spot::authenticate))
        .with_receive(Box::new(dango_account_spot::receive))
        .with_query(Box::new(dango_account_spot::query))
        .build();

    let bank = ContractBuilder::new(Box::new(dango_bank::instantiate))
        .with_execute(Box::new(dango_bank::execute))
        .with_query(Box::new(dango_bank::query))
        .with_bank_execute(Box::new(dango_bank::bank_execute))
        .with_bank_query(Box::new(dango_bank::bank_query))
        .build();

    let fee = ContractBuilder::new(Box::new(hyperlane_fee::instantiate))
        .with_execute(Box::new(hyperlane_fee::execute))
        .with_query(Box::new(hyperlane_fee::query))
        .build();

    let ism = ContractBuilder::new(Box::new(hyperlane_ism::instantiate))
        .with_execute(Box::new(hyperlane_ism::execute))
        .with_query(Box::new(hyperlane_ism::query))
        .build();

    let mailbox = ContractBuilder::new(Box::new(hyperlane_mailbox::instantiate))
        .with_execute(Box::new(hyperlane_mailbox::execute))
        .with_query(Box::new(hyperlane_mailbox::query))
        .build();

    let merkle = ContractBuilder::new(Box::new(hyperlane_merkle::instantiate))
        .with_execute(Box::new(hyperlane_merkle::execute))
        .with_query(Box::new(hyperlane_merkle::query))
        .build();

    let warp = ContractBuilder::new(Box::new(hyperlane_warp::instantiate))
        .with_execute(Box::new(hyperlane_warp::execute))
        .with_query(Box::new(hyperlane_warp::query))
        .build();

    let ibc_transfer = ContractBuilder::new(Box::new(dango_ibc_transfer::instantiate))
        .with_execute(Box::new(dango_ibc_transfer::execute))
        .build();

    let oracle = ContractBuilder::new(Box::new(dango_oracle::instantiate))
        .with_execute(Box::new(dango_oracle::execute))
        .with_authenticate(Box::new(dango_oracle::authenticate))
        .with_query(Box::new(dango_oracle::query))
        .build();

    let orderbook = ContractBuilder::new(Box::new(dango_orderbook::instantiate))
        .with_execute(Box::new(dango_orderbook::execute))
        .with_cron_execute(Box::new(dango_orderbook::cron_execute))
        .with_query(Box::new(dango_orderbook::query))
        .build();

    let lending = ContractBuilder::new(Box::new(dango_lending::instantiate))
        .with_execute(Box::new(dango_lending::execute))
        .with_query(Box::new(dango_lending::query))
        .build();

    let taxman = ContractBuilder::new(Box::new(dango_taxman::instantiate))
        .with_execute(Box::new(dango_taxman::execute))
        .with_query(Box::new(dango_taxman::query))
        .with_withhold_fee(Box::new(dango_taxman::withhold_fee))
        .with_finalize_fee(Box::new(dango_taxman::finalize_fee))
        .build();

    let vesting = ContractBuilder::new(Box::new(dango_vesting::instantiate))
        .with_execute(Box::new(dango_vesting::execute))
        .with_query(Box::new(dango_vesting::query))
        .build();

    Codes {
        account_factory,
        account_margin,
        account_safe,
        account_spot,
        bank,
        hyperlane: Hyperlane {
            fee,
            ism,
            mailbox,
            merkle,
            warp,
        },
        ibc_transfer,
        lending,
        oracle,
        orderbook,
        taxman,
        vesting,
    }
}

pub fn read_wasm_files(artifacts_dir: &Path) -> io::Result<Codes<Vec<u8>>> {
    let account_factory = fs::read(artifacts_dir.join("dango_account_factory.wasm"))?;
    let account_margin = fs::read(artifacts_dir.join("dango_account_margin.wasm"))?;
    let account_safe = fs::read(artifacts_dir.join("dango_account_safe.wasm"))?;
    let account_spot = fs::read(artifacts_dir.join("dango_account_spot.wasm"))?;
    let bank = fs::read(artifacts_dir.join("dango_bank.wasm"))?;
    let fee = fs::read(artifacts_dir.join("hyperlane_fee.wasm"))?;
    let ism = fs::read(artifacts_dir.join("hyperlane_ism.wasm"))?;
    let mailbox = fs::read(artifacts_dir.join("hyperlane_mailbox.wasm"))?;
    let merkle = fs::read(artifacts_dir.join("hyperlane_merkle.wasm"))?;
    let warp = fs::read(artifacts_dir.join("hyperlane_warp.wasm"))?;
    let ibc_transfer = fs::read(artifacts_dir.join("dango_ibc_transfer.wasm"))?;
    let lending = fs::read(artifacts_dir.join("dango_lending.wasm"))?;
    let oracle = fs::read(artifacts_dir.join("dango_oracle.wasm"))?;
    let orderbook = fs::read(artifacts_dir.join("dango_orderbook.wasm"))?;
    let taxman = fs::read(artifacts_dir.join("dango_taxman.wasm"))?;
    let vesting = fs::read(artifacts_dir.join("dango_vesting.wasm"))?;

    Ok(Codes {
        account_factory,
        account_margin,
        account_safe,
        account_spot,
        bank,
        hyperlane: Hyperlane {
            fee,
            ism,
            mailbox,
            merkle,
            warp,
        },
        ibc_transfer,
        lending,
        oracle,
        orderbook,
        taxman,
        vesting,
    })
}

pub fn build_genesis<T, D>(
    codes: Codes<T>,
    genesis_users: GenesisUsers,
    owner: &Username,
    fee_denom: D,
    fee_rate: Udec128,
    max_orphan_age: Duration,
) -> anyhow::Result<(GenesisState, Contracts, Addresses)>
where
    T: Into<Binary>,
    D: TryInto<Denom>,
    D::Error: Error + Send + Sync + 'static,
{
    let mut msgs = Vec::new();

    let fee_denom = fee_denom.try_into()?;

    // Upload all the codes and compute code hashes.
    let account_factory_code_hash = upload(&mut msgs, codes.account_factory);
    let account_margin_code_hash = upload(&mut msgs, codes.account_margin);
    let account_safe_code_hash = upload(&mut msgs, codes.account_safe);
    let account_spot_code_hash = upload(&mut msgs, codes.account_spot);
    let bank_code_hash = upload(&mut msgs, codes.bank);
    let hyperlane_fee_code_hash = upload(&mut msgs, codes.hyperlane.fee);
    let hyperlane_ism_code_hash = upload(&mut msgs, codes.hyperlane.ism);
    let hyperlane_mailbox_code_hash = upload(&mut msgs, codes.hyperlane.mailbox);
    let hyperlane_merkle_code_hash = upload(&mut msgs, codes.hyperlane.merkle);
    let hyperlane_warp_code_hash = upload(&mut msgs, codes.hyperlane.warp);
    let ibc_transfer_code_hash = upload(&mut msgs, codes.ibc_transfer);
    let lending_code_hash = upload(&mut msgs, codes.lending);
    let oracle_code_hash = upload(&mut msgs, codes.oracle);
    let orderbook_code_hash = upload(&mut msgs, codes.orderbook);
    let taxman_code_hash = upload(&mut msgs, codes.taxman);
    let vesting_code_hash = upload(&mut msgs, codes.vesting);

    // Instantiate account factory.
    let users = genesis_users
        .iter()
        .map(|(username, user)| (username.clone(), (user.key_hash, user.key)))
        .collect();

    let account_factory = instantiate(
        &mut msgs,
        account_factory_code_hash,
        &account_factory::InstantiateMsg {
            code_hashes: btree_map! {
                AccountType::Margin => account_margin_code_hash,
                AccountType::Safe   => account_safe_code_hash,
                AccountType::Spot   => account_spot_code_hash,
            },
            users,
        },
        "dango/account_factory",
        "dango/account_factory",
    )?;

    // Derive the addresses of the genesis accounts that were just created.
    let addresses = genesis_users
        .iter()
        .map(|(username, user)| {
            let salt = NewUserSalt {
                username,
                key: user.key,
                key_hash: user.key_hash,
            }
            .into_bytes();
            let address = Addr::derive(account_factory, account_spot_code_hash, &salt);
            Ok((username.clone(), address))
        })
        .collect::<StdResult<BTreeMap<_, _>>>()?;

    // Derive the Hyperlane mailbox contract address.
    // This is needed for the hook and recipient contracts.
    let mailbox = Addr::derive(
        GENESIS_SENDER,
        hyperlane_mailbox_code_hash,
        b"hyperlane/mailbox",
    );

    // Instantiate Hyperlane fee hook.
    let fee = instantiate(
        &mut msgs,
        hyperlane_fee_code_hash,
        &hooks::fee::InstantiateMsg { mailbox },
        "hyperlane/hook/fee",
        "hyperlane/hook/fee",
    )?;

    // Instantiate Hyperlane merkle hook.
    let merkle = instantiate(
        &mut msgs,
        hyperlane_merkle_code_hash,
        &hooks::merkle::InstantiateMsg { mailbox },
        "hyperlane/hook/merkle",
        "hyperlane/hook/merkle",
    )?;

    // Instantiate Hyperlane message ID multisig ISM.
    let ism = instantiate(
        &mut msgs,
        hyperlane_ism_code_hash,
        &isms::multisig::InstantiateMsg {
            validator_sets: btree_map! {},
        },
        "hyperlane/ism/multisig",
        "hyperlane/ism/multisig",
    )?;

    // Instantiate Hyperlane Warp contract.
    let warp = instantiate(
        &mut msgs,
        hyperlane_warp_code_hash,
        &warp::InstantiateMsg { mailbox },
        "hyperlane/warp",
        "hyperlane/warp",
    )?;

    // Instantiate Hyperlane mailbox. Ensure address is the same as the predicted.
    instantiate(
        &mut msgs,
        hyperlane_mailbox_code_hash,
        &mailbox::InstantiateMsg {
            config: mailbox::Config {
                local_domain: 88888888, // TODO
                default_ism: ism,
                default_hook: fee,
                required_hook: merkle,
            },
        },
        "hyperlane/mailbox",
        "hyperlane/mailbox",
    )
    .should_succeed_and_equal(mailbox);

    // Instantiate the IBC transfer contract.
    let ibc_transfer = instantiate(
        &mut msgs,
        ibc_transfer_code_hash,
        &ibc::transfer::InstantiateMsg {},
        "dango/ibc_transfer",
        "dango/ibc_transfer",
    )?;

    // Instantiate the orderbook contract.
    let orderbook = instantiate(
        &mut msgs,
        orderbook_code_hash,
        &orderbook::InstantiateMsg {},
        "dango/orderbook",
        "dango/orderbook",
    )?;

    // Instantiate the lending pool contract.
    let lending = instantiate(
        &mut msgs,
        lending_code_hash,
        &lending::InstantiateMsg {
            markets: btree_map! {
                fee_denom.clone() => MarketUpdates {
                    // TODO
                },
            },
        },
        "dango/lending",
        "dango/lending",
    )?;

    // Create the `balances` map needed for instantiating bank.
    let balances = genesis_users
        .into_iter()
        .zip(&addresses)
        .filter_map(|((_, user), (_, address))| {
            if user.balances.is_empty() {
                None
            } else {
                Some((*address, user.balances))
            }
        })
        .collect();

    // Create the `namespaces` map needed for instantiating bank.
    // Token factory gets the "factory" namespace.
    // IBC trasfer gets the "ibc" namespace.
    let namespaces = btree_map! {
        ibc::transfer::NAMESPACE.clone() => ibc_transfer,
        lending::NAMESPACE.clone()       => lending,
        warp::NAMESPACE.clone()          => warp,
    };

    // Instantiate the bank contract.
    let bank = instantiate(
        &mut msgs,
        bank_code_hash,
        &bank::InstantiateMsg {
            balances,
            namespaces,
            metadatas: btree_map! {
                // TODO: add dango token metadata
            },
        },
        "dango/bank",
        "dango/bank",
    )?;

    // Instantiate the taxman contract.
    let taxman = instantiate(
        &mut msgs,
        taxman_code_hash,
        &taxman::InstantiateMsg {
            config: taxman::Config {
                fee_denom,
                fee_rate,
            },
        },
        "dango/taxman",
        "dango/taxman",
    )?;

    // Instantiate the oracle contract.
    let oracle = instantiate(
        &mut msgs,
        oracle_code_hash,
        &oracle::InstantiateMsg {
            guardian_sets: btree_map! {
                GUARDIAN_SETS_INDEX => GuardianSet {
                    addresses: GUARDIANS_ADDRESSES
                        .into_iter()
                        .map(|addr| {
                            let bytes = Binary::from_str(addr)
                                .unwrap()
                                .into_inner()
                                .try_into()
                                .unwrap();
                            Hash160::from_inner(bytes)
                        })
                        .collect(),
                    expiration_time: None,
                },
            },
            price_sources: btree_map! {
                Denom::from_str("usdc").unwrap() => PriceSource::Pyth { id: USDC_USD_ID, precision: 6 },
                Denom::from_str("btc").unwrap()  => PriceSource::Pyth { id: WBTC_USD_ID, precision: 8 },
                Denom::from_str("eth").unwrap()  => PriceSource::Pyth { id: ETH_USD_ID, precision: 18 },
            },
        },
        "dango/oracle",
        "dango/oracle",
    )?;

    let vesting = instantiate(
        &mut msgs,
        vesting_code_hash,
        &vesting::InstantiateMsg {
            unlocking_cliff: Duration::from_weeks(4 * 9),
            unlocking_period: Duration::from_weeks(4 * 27),
        },
        "dango/vesting",
        "dango/vesting",
    )?;

    let contracts = Contracts {
        account_factory,
        bank,
        hyperlane: Hyperlane {
            fee,
            ism,
            mailbox,
            merkle,
            warp,
        },
        ibc_transfer,
        lending,
        oracle,
        orderbook,
        taxman,
        vesting,
    };

    let permissions = Permissions {
        upload: Permission::Nobody,
        instantiate: Permission::Somebodies(btree_set! { account_factory }),
    };

    let config = Config {
        owner: addresses.get(owner).cloned().unwrap(),
        bank,
        taxman,
        // Important: orderbook cronjob is to be invoked at end of every block.
        cronjobs: btree_map! { orderbook => Duration::ZERO },
        permissions,
        max_orphan_age,
    };

    let app_config = AppConfig {
        addresses: AppAddresses {
            account_factory,
            ibc_transfer,
            lending,
            oracle,
        },
        collateral_powers: btree_map! {},
    };

    let genesis_state = GenesisState {
        config,
        msgs,
        app_config: app_config.to_json_value()?,
    };

    Ok((genesis_state, contracts, addresses))
}

fn upload<B>(msgs: &mut Vec<Message>, code: B) -> Hash256
where
    B: Into<Binary>,
{
    let code = code.into();
    let code_hash = code.hash256();

    msgs.push(Message::upload(code));

    code_hash
}

fn instantiate<M, S, L>(
    msgs: &mut Vec<Message>,
    code_hash: Hash256,
    msg: &M,
    salt: S,
    label: L,
) -> anyhow::Result<Addr>
where
    M: Serialize,
    S: Into<Binary>,
    L: Into<String>,
{
    let salt = salt.into();
    let address = Addr::derive(GENESIS_SENDER, code_hash, &salt);

    msgs.push(Message::instantiate(
        code_hash,
        msg,
        salt,
        Some(label),
        None,
        Coins::new(),
    )?);

    Ok(address)
}
