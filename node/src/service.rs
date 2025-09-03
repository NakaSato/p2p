//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::{sync::Arc, time::Duration};

use p2p_runtime::{self, opaque::Block, RuntimeApi};
use sc_client_api::{ExecutorProvider, RemoteBackend};
use sc_consensus_aura::{ImportQueueParams, SlotProportion, StartAuraParams};
use sc_consensus_grandpa::SharedVoterState;
use sc_executor::NativeElseWasmExecutor;
use sc_service::{error::Error as ServiceError, Configuration, TaskManager};
use sc_telemetry::{Telemetry, TelemetryWorker};
use sp_consensus_aura::sr25519::AuthorityPair as AuraPair;
use sp_runtime::traits::{AccountIdConversion, BlakeTwo256, Block as BlockT};

// Our native executor instance.
pub struct ExecutorDispatch;

impl sc_executor::NativeExecutionDispatch for ExecutorDispatch {
    /// Only enable the benchmarking host functions when we actually want to benchmark.
    #[cfg(feature = "runtime-benchmarks")]
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;
    /// Otherwise we only use the default Substrate host functions.
    #[cfg(not(feature = "runtime-benchmarks"))]
    type ExtendHostFunctions = ();

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        p2p_runtime::api::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        p2p_runtime::native_version()
    }
}

pub(crate) type FullClient =
    sc_service::TFullClient<Block, RuntimeApi, NativeElseWasmExecutor<ExecutorDispatch>>;
type FullBackend = sc_service::TFullBackend<Block>;
type FullSelectChain = sc_consensus::LongestChain<FullBackend, Block>;

pub fn new_partial(
    config: &Configuration,
) -> Result<
    sc_service::PartialComponents<
        FullClient,
        FullBackend,
        FullSelectChain,
        sc_consensus::DefaultImportQueue<Block, FullClient>,
        sc_transaction_pool::FullPool<Block, FullClient>,
        (
            sc_consensus_aura::AuraBlockImport<
                Block,
                FullClient,
                sc_consensus_grandpa::GrandpaBlockImport<FullBackend, Block, FullClient, FullSelectChain>,
                AuraPair,
            >,
            sc_consensus_grandpa::LinkHalf<Block, FullClient, FullSelectChain>,
            Option<Telemetry>,
        ),
    >,
    ServiceError,
> {
    if config.keystore_remote.is_some() {
        return Err(ServiceError::Other("Remote Keystores are not supported.".into()));
    }

    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let executor = NativeElseWasmExecutor::<ExecutorDispatch>::new(
        config.wasm_method,
        config.default_heap_pages,
        config.max_runtime_instances,
        config.runtime_cache_size,
    );

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });

    let select_chain = sc_consensus::LongestChain::new(backend.clone());

    let transaction_pool = sc_transaction_pool::BasicPool::new_full(
        config.transaction_pool.clone(),
        config.role.is_authority(),
        config.prometheus_registry(),
        task_manager.spawn_essential_handle(),
        client.clone(),
    );

    let (grandpa_block_import, grandpa_link) = sc_consensus_grandpa::block_import(
        client.clone(),
        &(client.clone() as Arc<_>),
        select_chain.clone(),
        telemetry.as_ref().map(|x| x.handle()),
    )?;

    let aura_block_import =
        sc_consensus_aura::AuraBlockImport::<_, _, _, AuraPair>::new(grandpa_block_import, client.clone());

    let import_queue = sc_consensus_aura::import_queue::<AuraPair, _, _, _, _, _>(ImportQueueParams {
        block_import: aura_block_import.clone(),
        justification_import: Some(Box::new(grandpa_link.clone())),
        client: client.clone(),
        select_chain: select_chain.clone(),
        spawner: &task_manager.spawn_essential_handle(),
        registry: config.prometheus_registry(),
        check_for_equivocation: SlotProportion::new(2f32 / 3f32),
        telemetry: telemetry.as_ref().map(|x| x.handle()),
        compatibility_mode: Default::default(),
    })?;

    Ok(sc_service::PartialComponents {
        client,
        backend,
        task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (aura_block_import, grandpa_link, telemetry),
    })
}

/// Builds a new service for a full client.
pub fn new_full(config: Configuration) -> Result<TaskManager, ServiceError> {
    let sc_service::PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        select_chain,
        transaction_pool,
        other: (block_import, grandpa_link, mut telemetry),
    } = new_partial(&config)?;

    let mut net_config = sc_network::config::FullNetworkConfiguration::new(&config.network);

    let grandpa_protocol_name = sc_consensus_grandpa::protocol_standard_name(
        &client.block_hash(0)?.expect("Genesis block exists; qed"),
        &config.chain_spec,
    );

    net_config.add_notification_protocol(sc_consensus_grandpa::grandpa_peers_set_config(
        grandpa_protocol_name.clone(),
    ));

    let warp_sync = Arc::new(sc_consensus_grandpa::warp_proof::NetworkProvider::new(
        backend.clone(),
        grandpa_link.shared_authority_set().clone(),
        Vec::default(),
    ));

    let (network, system_rpc_tx, tx_handler_controller, network_starter) =
        sc_service::build_network(sc_service::BuildNetworkParams {
            config: &config,
            net_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            spawn_handle: task_manager.spawn_handle(),
            import_queue,
            block_announce_validator_builder: None,
            warp_sync_provider: Some(warp_sync),
        })?;

    if config.offchain_worker.enabled {
        sc_service::build_offchain_workers(
            &config,
            task_manager.spawn_handle(),
            client.clone(),
            network.clone(),
        );
    }

    let role = config.role.clone();
    let force_authoring = config.force_authoring;
    let backoff_authoring_blocks =
        Some(sc_consensus::BackoffAuthoringOnFinalizedHeadLagging::default());
    let name = config.network.node_name.clone();
    let enable_grandpa = !config.disable_grandpa;
    let prometheus_registry = config.prometheus_registry().cloned();

    let rpc_extensions_builder = {
        let client = client.clone();
        let pool = transaction_pool.clone();

        Box::new(move |deny_unsafe, _| {
            let deps =
                crate::rpc::FullDeps { client: client.clone(), pool: pool.clone(), deny_unsafe };

            crate::rpc::create_full(deps).map_err(Into::into)
        })
    };

    let _rpc_handlers = sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        network: network.clone(),
        client: client.clone(),
        keystore: keystore_container.sync_keystore(),
        task_manager: &mut task_manager,
        transaction_pool: transaction_pool.clone(),
        rpc_builder: rpc_extensions_builder,
        backend,
        system_rpc_tx,
        tx_handler_controller,
        config,
        telemetry: telemetry.as_mut(),
    })?;

    if role.is_authority() {
        let proposer_factory = sc_basic_authorship::ProposerFactory::new(
            task_manager.spawn_handle(),
            client.clone(),
            transaction_pool.clone(),
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|x| x.handle()),
        );

        let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
        let raw_slot_duration = slot_duration.slot_duration();

        let aura = sc_consensus_aura::start_aura::<AuraPair, _, _, _, _, _, _, _, _, _, _>(
            StartAuraParams {
                slot_duration,
                client,
                select_chain,
                block_import,
                proposer_factory,
                create_inherent_data_providers: move |_, ()| async move {
                    let timestamp = sp_timestamp::InherentDataProvider::from_system_time();

                    let slot =
                        sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                            *timestamp,
                            raw_slot_duration,
                        );

                    Ok((slot, timestamp))
                },
                force_authoring,
                backoff_authoring_blocks,
                keystore_container.sync_keystore(),
                sync_oracle: network.clone(),
                justification_sync_link: network.clone(),
                block_proposal_slot_portion: SlotProportion::new(2f32 / 3f32),
                max_block_proposal_slot_portion: None,
                telemetry: telemetry.as_ref().map(|x| x.handle()),
                compatibility_mode: Default::default(),
            },
        )?;

        // the AURA authoring task is considered essential, i.e. if it
        // fails we take down the service with it.
        task_manager
            .spawn_essential_handle()
            .spawn_blocking("aura", Some("block-authoring"), aura);
    }

    // if the node isn't running as a validator, we can run the GRANDPA observer protocol instead
    if enable_grandpa {
        let grandpa_config = sc_consensus_grandpa::Config {
            // FIXME #1578 make this available through chainspec
            gossip_duration: Duration::from_millis(333),
            justification_period: 512,
            name: Some(name),
            observer_enabled: false,
            keystore: Some(keystore_container.sync_keystore()),
            local_role: role,
            telemetry: telemetry.as_ref().map(|x| x.handle()),
            protocol_name: grandpa_protocol_name,
        };

        if role.is_authority() {
            // start the full GRANDPA voter
            // NOTE: non-authorities could run the GRANDPA observer protocol, but at
            // this point the full voter should provide better guarantees of block
            // and vote data availability than the observer. The observer has not
            // been tested extensively yet and having most nodes in a network run it
            // could lead to finality stalls.
            let grandpa_config = sc_consensus_grandpa::GrandpaParams {
                config: grandpa_config,
                link: grandpa_link,
                network: network.clone(),
                sync: Arc::new(network.clone()),
                voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
                prometheus_registry,
                shared_voter_state: SharedVoterState::empty(),
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            };

            // the GRANDPA voter task is considered infallible, i.e.
            // if it fails we take down the service with it.
            task_manager.spawn_essential_handle().spawn_blocking(
                "grandpa-voter",
                None,
                sc_consensus_grandpa::run_grandpa_voter(grandpa_config)?,
            );
        } else {
            let grandpa_config = sc_consensus_grandpa::GrandpaParams {
                config: grandpa_config,
                link: grandpa_link,
                network: network.clone(),
                sync: Arc::new(network.clone()),
                voting_rule: sc_consensus_grandpa::VotingRulesBuilder::default().build(),
                prometheus_registry,
                shared_voter_state: SharedVoterState::empty(),
                telemetry: telemetry.as_ref().map(|x| x.handle()),
            };
            task_manager.spawn_essential_handle().spawn_blocking(
                "grandpa-observer",
                None,
                sc_consensus_grandpa::run_grandpa_observer(grandpa_config)?,
            );
        }
    }

    network_starter.start_network();
    Ok(task_manager)
}

pub struct RemarkBuilder {
    client: Arc<FullClient>,
}

impl RemarkBuilder {
    pub fn new(client: Arc<FullClient>) -> Self {
        Self { client }
    }
}

impl sc_cli::ExtrinsicBuilder for RemarkBuilder {
    fn pallet(&self) -> &str {
        "system"
    }

    fn extrinsic(&self) -> &str {
        "remark"
    }

    fn build(&self, nonce: u32) -> std::result::Result<p2p_runtime::UncheckedExtrinsic, sc_cli::Error> {
        with_client! {
            self.client.as_ref(), client, {
                use p2p_runtime::{Call, SystemCall};

                let call = Call::System(SystemCall::remark { remark: vec![] });
                let signer = sp_keyring::Sr25519Keyring::Bob.pair();
                let period = sp_runtime::generic::Era::mortal(256, client.usage_info().chain.best_number);
                let extra = |i: sp_runtime::generic::Index| {
                    (
                        frame_system::CheckSpecVersion::<p2p_runtime::Runtime>::new(),
                        frame_system::CheckTxVersion::<p2p_runtime::Runtime>::new(),
                        frame_system::CheckGenesis::<p2p_runtime::Runtime>::new(),
                        frame_system::CheckEra::<p2p_runtime::Runtime>::from(period),
                        frame_system::CheckNonce::<p2p_runtime::Runtime>::from(i),
                        frame_system::CheckWeight::<p2p_runtime::Runtime>::new(),
                        pallet_transaction_payment::ChargeTransactionPayment::<p2p_runtime::Runtime>::from(0),
                    )
                };

                let raw_payload = p2p_runtime::SignedPayload::from_raw(
                    call,
                    extra(nonce),
                    (
                        p2p_runtime::VERSION.spec_version,
                        p2p_runtime::VERSION.transaction_version,
                        client.usage_info().chain.best_hash,
                        client.usage_info().chain.best_hash,
                        (),
                        (),
                        (),
                    ),
                );
                let signature = raw_payload.using_encoded(|payload| signer.sign(payload));

                p2p_runtime::UncheckedExtrinsic::new_signed(
                    call.clone(),
                    sp_runtime::MultiAddress::Id(signer.public().into()),
                    sp_runtime::MultiSignature::Sr25519(signature.clone()),
                    extra(nonce),
                )
                .ok_or_else(|| "Failed to construct extrinsic".into())
            }
        }
    }
}

pub struct TransferKeepAliveBuilder {
    client: Arc<FullClient>,
    dest: sp_keyring::AccountKeyring,
    value: p2p_runtime::Balance,
}

impl TransferKeepAliveBuilder {
    pub fn new(
        client: Arc<FullClient>,
        dest: sp_keyring::AccountKeyring,
        value: p2p_runtime::Balance,
    ) -> Self {
        Self {
            client,
            dest,
            value,
        }
    }
}

impl sc_cli::ExtrinsicBuilder for TransferKeepAliveBuilder {
    fn pallet(&self) -> &str {
        "balances"
    }

    fn extrinsic(&self) -> &str {
        "transfer_keep_alive"
    }

    fn build(&self, nonce: u32) -> std::result::Result<p2p_runtime::UncheckedExtrinsic, sc_cli::Error> {
        with_client! {
            self.client.as_ref(), client, {
                use p2p_runtime::{Call, BalancesCall};

                let call = Call::Balances(BalancesCall::transfer_keep_alive {
                    dest: sp_runtime::MultiAddress::Id(self.dest.to_account_id()),
                    value: self.value,
                });
                let signer = sp_keyring::Sr25519Keyring::Alice.pair();
                let period = sp_runtime::generic::Era::mortal(256, client.usage_info().chain.best_number);
                let extra = |i: sp_runtime::generic::Index| {
                    (
                        frame_system::CheckSpecVersion::<p2p_runtime::Runtime>::new(),
                        frame_system::CheckTxVersion::<p2p_runtime::Runtime>::new(),
                        frame_system::CheckGenesis::<p2p_runtime::Runtime>::new(),
                        frame_system::CheckEra::<p2p_runtime::Runtime>::from(period),
                        frame_system::CheckNonce::<p2p_runtime::Runtime>::from(i),
                        frame_system::CheckWeight::<p2p_runtime::Runtime>::new(),
                        pallet_transaction_payment::ChargeTransactionPayment::<p2p_runtime::Runtime>::from(0),
                    )
                };

                let raw_payload = p2p_runtime::SignedPayload::from_raw(
                    call,
                    extra(nonce),
                    (
                        p2p_runtime::VERSION.spec_version,
                        p2p_runtime::VERSION.transaction_version,
                        client.usage_info().chain.best_hash,
                        client.usage_info().chain.best_hash,
                        (),
                        (),
                        (),
                    ),
                );
                let signature = raw_payload.using_encoded(|payload| signer.sign(payload));

                p2p_runtime::UncheckedExtrinsic::new_signed(
                    call.clone(),
                    sp_runtime::MultiAddress::Id(signer.public().into()),
                    sp_runtime::MultiSignature::Sr25519(signature.clone()),
                    extra(nonce),
                )
                .ok_or_else(|| "Failed to construct extrinsic".into())
            }
        }
    }
}
