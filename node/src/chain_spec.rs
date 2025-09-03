use p2p_runtime::{
    pallet_grid_token, pallet_oracle_client, pallet_registry, pallet_trading, AccountId,
    AuraConfig, Balance, BalancesConfig, GenesisConfig, GrandpaConfig, Signature, SudoConfig,
    SystemConfig, WASM_BINARY,
};
use sc_service::ChainType;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

// The URL for the telemetry server.
// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig>;

/// Generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate an Aura authority key.
pub fn authority_keys_from_seed(s: &str) -> (AuraId, GrandpaId) {
    (get_from_seed::<AuraId>(s), get_from_seed::<GrandpaId>(s))
}

pub fn development_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "P2P Energy Trading Development",
        // ID
        "p2p_dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities (Thai energy authorities)
                vec![
                    authority_keys_from_seed("ERC"),  // Energy Regulatory Commission
                    authority_keys_from_seed("MEA"),  // Metropolitan Electricity Authority
                    authority_keys_from_seed("PEA"),  // Provincial Electricity Authority
                    authority_keys_from_seed("EGAT"), // Electricity Generating Authority of Thailand
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        None,
        None,
        // Properties
        Some(
            serde_json::json!({
                "tokenDecimals": 18,
                "tokenSymbol": "GRID"
            })
            .as_object()
            .expect("Map given; qed")
            .clone(),
        ),
        // Extensions
        None,
    ))
}

pub fn local_testnet_config() -> Result<ChainSpec, String> {
    let wasm_binary = WASM_BINARY.ok_or_else(|| "Development wasm not available".to_string())?;

    Ok(ChainSpec::from_genesis(
        // Name
        "P2P Energy Trading Local Testnet",
        // ID
        "p2p_local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                wasm_binary,
                // Initial PoA authorities (Thai energy authorities)
                vec![
                    authority_keys_from_seed("ERC"),  // Energy Regulatory Commission
                    authority_keys_from_seed("MEA"),  // Metropolitan Electricity Authority
                    authority_keys_from_seed("PEA"),  // Provincial Electricity Authority
                    authority_keys_from_seed("EGAT"), // Electricity Generating Authority of Thailand
                ],
                // Sudo account
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // Pre-funded accounts
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie"),
                    get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
                    get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
                ],
                true,
            )
        },
        // Bootnodes
        vec![],
        // Telemetry
        None,
        // Protocol ID
        Some("p2p"),
        None,
        // Properties
        Some(
            serde_json::json!({
                "tokenDecimals": 18,
                "tokenSymbol": "GRID"
            })
            .as_object()
            .expect("Map given; qed")
            .clone(),
        ),
        // Extensions
        None,
    ))
}

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
    wasm_binary: &[u8],
    initial_authorities: Vec<(AuraId, GrandpaId)>,
    root_key: AccountId,
    endowed_accounts: Vec<AccountId>,
    _enable_println: bool,
) -> GenesisConfig {
    GenesisConfig {
        system: SystemConfig {
            // Add Wasm runtime to storage.
            code: wasm_binary.to_vec(),
        },
        balances: BalancesConfig {
            // Configure endowed accounts with initial balance of 1 << 60.
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|k| (k, 1 << 60))
                .collect(),
        },
        aura: AuraConfig {
            authorities: initial_authorities.iter().map(|x| (x.0.clone())).collect(),
        },
        grandpa: GrandpaConfig {
            authorities: initial_authorities
                .iter()
                .map(|x| (x.1.clone(), 1))
                .collect(),
        },
        sudo: SudoConfig {
            // Assign network admin rights.
            key: Some(root_key.clone()),
        },
        transaction_payment: Default::default(),

        // P2P Energy Trading Pallet Configurations
        registry: pallet_registry::GenesisConfig {
            admins: vec![
                root_key.clone(),                                   // Alice as main admin
                get_account_id_from_seed::<sr25519::Public>("Bob"), // Bob as secondary admin
            ],
            users: vec![
                // Pre-register some test users
                (
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    pallet_registry::UserInfo {
                        user_type: pallet_registry::UserType::Prosumer,
                        location: b"Building A, Floor 3".to_vec(),
                        status: pallet_registry::UserStatus::Active,
                        registered_at: 0,
                    },
                ),
                (
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    pallet_registry::UserInfo {
                        user_type: pallet_registry::UserType::Consumer,
                        location: b"Building B, Floor 1".to_vec(),
                        status: pallet_registry::UserStatus::Active,
                        registered_at: 0,
                    },
                ),
                (
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    pallet_registry::UserInfo {
                        user_type: pallet_registry::UserType::Prosumer,
                        location: b"Building C, Rooftop".to_vec(),
                        status: pallet_registry::UserStatus::Active,
                        registered_at: 0,
                    },
                ),
            ],
            meters: vec![
                // Assign meters to users
                (
                    b"METER_001_CHARLIE".to_vec(),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ),
                (
                    b"METER_002_DAVE".to_vec(),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                ),
                (
                    b"METER_003_EVE_SOLAR".to_vec(),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                ),
                (
                    b"METER_004_EVE_LOAD".to_vec(),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                ),
            ],
        },

        grid_token: pallet_grid_token::GenesisConfig {
            balances: vec![
                // Give some initial GRID tokens for testing
                (
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    10000 * 10_u128.pow(18),
                ),
                (
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    5000 * 10_u128.pow(18),
                ),
                (
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    8000 * 10_u128.pow(18),
                ),
            ],
            minters: vec![
                root_key.clone(),                                   // Alice can mint tokens
                get_account_id_from_seed::<sr25519::Public>("Bob"), // Bob as AMI service account
            ],
            burners: vec![
                root_key.clone(),                                   // Alice can burn tokens
                get_account_id_from_seed::<sr25519::Public>("Bob"), // Bob as AMI service account
            ],
        },

        trading: pallet_trading::GenesisConfig {
            market_makers: vec![
                root_key.clone(),                                   // Alice as market maker
                get_account_id_from_seed::<sr25519::Public>("Bob"), // Bob as oracle service account
            ],
            epoch_start: 0, // Start at genesis
        },

        oracle_client: pallet_oracle_client::GenesisConfig {
            oracle_operators: vec![
                root_key.clone(),                                      // Alice as oracle operator
                get_account_id_from_seed::<sr25519::Public>("Bob"), // Bob as oracle service account
                get_account_id_from_seed::<sr25519::Public>("Ferdie"), // Ferdie as Chainlink operator
            ],
            oracle_balance: 1000 * 10_u128.pow(18), // 1000 LINK tokens equivalent
            auto_market_clearing: true,
        },
    }
}
