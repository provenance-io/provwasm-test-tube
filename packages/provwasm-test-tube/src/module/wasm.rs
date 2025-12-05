use std::io::Write;

use cosmwasm_std::Coin;
use flate2::write::GzEncoder;
use flate2::Compression;
use provwasm_std::types::cosmwasm::wasm::v1::{
    AccessConfig, MsgExecuteContract, MsgExecuteContractResponse, MsgInstantiateContract,
    MsgInstantiateContractResponse, MsgMigrateContract, MsgMigrateContractResponse, MsgStoreCode,
    MsgStoreCodeResponse, QueryCodeRequest, QueryCodeResponse, QueryCodesRequest,
    QueryCodesResponse, QueryContractHistoryRequest, QueryContractHistoryResponse,
    QueryContractsByCodeRequest, QueryContractsByCodeResponse, QueryContractsByCreatorRequest,
    QueryContractsByCreatorResponse, QueryParamsRequest, QueryParamsResponse,
    QueryPinnedCodesRequest, QueryPinnedCodesResponse, QuerySmartContractStateRequest,
    QuerySmartContractStateResponse,
};
use serde::{de::DeserializeOwned, Serialize};

use test_tube_prov::fn_query;
use test_tube_prov::{
    Account, DecodeError, EncodeError, Runner, RunnerError, RunnerExecuteResult, RunnerResult,
    SigningAccount,
};

use super::Module;

/// Gzip magic number bytes used to detect if data is already gzip-compressed.
const GZIP_MAGIC: [u8; 2] = [0x1f, 0x8b];

/// Checks if the given bytes are gzip-compressed by looking for the gzip magic number.
fn is_gzip(data: &[u8]) -> bool {
    data.len() >= 2 && data[0] == GZIP_MAGIC[0] && data[1] == GZIP_MAGIC[1]
}

/// Compresses the given bytes using gzip compression.
/// This matches the behavior of the provenanced CLI which automatically gzips wasm
/// files before submitting them in transactions, reducing transaction size and gas costs.
fn gzip_bytes(data: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    encoder.finish()
}

pub struct Wasm<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Wasm<'a, R> {
    fn new(runner: &'a R) -> Self {
        Wasm { runner }
    }
}

impl<'a, R> Wasm<'a, R>
where
    R: Runner<'a>,
{
    /// Stores wasm code on chain. The wasm bytes are automatically gzip-compressed
    /// if not already compressed, matching the behavior of the provenanced CLI.
    /// This significantly reduces transaction size and gas costs.
    pub fn store_code(
        &self,
        wasm_byte_code: &[u8],
        instantiate_permission: Option<AccessConfig>,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgStoreCodeResponse> {
        // Gzip the wasm if not already compressed, matching CLI behavior.
        // This reduces transaction size and associated gas costs significantly.
        let compressed_code = if is_gzip(wasm_byte_code) {
            wasm_byte_code.to_vec()
        } else {
            gzip_bytes(wasm_byte_code).map_err(|e| {
                RunnerError::EncodeError(EncodeError::JsonEncodeError(serde_json::Error::io(e)))
            })?
        };

        self.runner.execute(
            MsgStoreCode {
                sender: signer.address(),
                wasm_byte_code: compressed_code,
                instantiate_permission,
            },
            "/cosmwasm.wasm.v1.MsgStoreCode",
            signer,
        )
    }

    pub fn instantiate<M>(
        &self,
        code_id: u64,
        msg: &M,
        admin: Option<&str>,
        label: Option<&str>,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgInstantiateContractResponse>
    where
        M: ?Sized + Serialize,
    {
        self.runner.execute(
            MsgInstantiateContract {
                sender: signer.address(),
                admin: admin.unwrap_or_default().to_string(),
                code_id,
                label: label.unwrap_or(" ").to_string(), // empty string causes panic
                msg: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
                funds: funds
                    .iter()
                    .map(|c| provwasm_std::types::cosmos::base::v1beta1::Coin {
                        denom: c.denom.parse().unwrap(),
                        amount: format!("{}", c.amount.u128()),
                    })
                    .collect(),
            },
            "/cosmwasm.wasm.v1.MsgInstantiateContract",
            signer,
        )
    }

    pub fn execute<M>(
        &self,
        contract: &str,
        msg: &M,
        funds: &[Coin],
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgExecuteContractResponse>
    where
        M: ?Sized + Serialize,
    {
        self.runner.execute(
            MsgExecuteContract {
                sender: signer.address(),
                msg: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
                funds: funds
                    .iter()
                    .map(|c| provwasm_std::types::cosmos::base::v1beta1::Coin {
                        denom: c.denom.parse().unwrap(),
                        amount: format!("{}", c.amount.u128()),
                    })
                    .collect(),
                contract: contract.to_owned(),
            },
            "/cosmwasm.wasm.v1.MsgExecuteContract",
            signer,
        )
    }

    pub fn migrate<M>(
        &self,
        code_id: u64,
        contract: &str,
        msg: &M,
        signer: &SigningAccount,
    ) -> RunnerExecuteResult<MsgMigrateContractResponse>
    where
        M: ?Sized + Serialize,
    {
        self.runner.execute(
            MsgMigrateContract {
                sender: signer.address(),
                contract: contract.to_owned(),
                code_id,
                msg: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
            },
            "/cosmwasm.wasm.v1.MsgMigrateContract",
            signer,
        )
    }

    pub fn query<M, Res>(&self, contract: &str, msg: &M) -> RunnerResult<Res>
    where
        M: ?Sized + Serialize,
        Res: ?Sized + DeserializeOwned,
    {
        let res = self
            .runner
            .query::<QuerySmartContractStateRequest, QuerySmartContractStateResponse>(
                "/cosmwasm.wasm.v1.Query/SmartContractState",
                &QuerySmartContractStateRequest {
                    address: contract.to_owned(),
                    query_data: serde_json::to_vec(msg).map_err(EncodeError::JsonEncodeError)?,
                },
            )?;

        serde_json::from_slice(&res.data)
            .map_err(DecodeError::JsonDecodeError)
            .map_err(RunnerError::DecodeError)
    }

    fn_query! {
        pub query_contract_history ["/cosmwasm.wasm.v1.Query/ContractHistory"]: QueryContractHistoryRequest => QueryContractHistoryResponse
    }

    fn_query! {
        pub query_contracts_by_code ["/cosmwasm.wasm.v1.Query/ContractsByCode"]: QueryContractsByCodeRequest => QueryContractsByCodeResponse
    }

    fn_query! {
        pub query_code ["/cosmwasm.wasm.v1.Query/Code"]: QueryCodeRequest => QueryCodeResponse
    }

    fn_query! {
        pub query_codes ["/cosmwasm.wasm.v1.Query/Codes"]: QueryCodesRequest => QueryCodesResponse
    }

    fn_query! {
        pub query_pinned_codes ["/cosmwasm.wasm.v1.Query/PinnedCodes"]: QueryPinnedCodesRequest => QueryPinnedCodesResponse
    }

    fn_query! {
        pub query_params ["/cosmwasm.wasm.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_contracts_by_creator ["/cosmwasm.wasm.v1.Query/ContractsByCreator"]: QueryContractsByCreatorRequest => QueryContractsByCreatorResponse
    }
}
