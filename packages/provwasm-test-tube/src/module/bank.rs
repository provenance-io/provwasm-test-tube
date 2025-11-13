use provwasm_std::types::cosmos::bank::v1beta1::{
    MsgMultiSend, MsgMultiSendResponse, MsgSend, MsgSendResponse, QueryAllBalancesRequest,
    QueryAllBalancesResponse, QueryBalanceRequest, QueryBalanceResponse,
    QueryDenomMetadataByQueryStringRequest, QueryDenomMetadataByQueryStringResponse,
    QueryDenomMetadataRequest, QueryDenomMetadataResponse, QueryDenomOwnersByQueryRequest,
    QueryDenomOwnersByQueryResponse, QueryDenomOwnersRequest, QueryDenomOwnersResponse,
    QueryDenomsMetadataRequest, QueryDenomsMetadataResponse, QueryParamsRequest,
    QueryParamsResponse, QuerySendEnabledRequest, QuerySendEnabledResponse,
    QuerySpendableBalanceByDenomRequest, QuerySpendableBalanceByDenomResponse,
    QuerySpendableBalancesRequest, QuerySpendableBalancesResponse, QuerySupplyOfRequest,
    QuerySupplyOfResponse, QueryTotalSupplyRequest, QueryTotalSupplyResponse,
};

use test_tube_prov::module::Module;
use test_tube_prov::{fn_execute, fn_query, Runner};

pub struct Bank<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Bank<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Bank<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub send: MsgSend["/cosmos.bank.v1beta1.MsgSend"] => MsgSendResponse
    }

    fn_execute! {
        pub multi_send: MsgMultiSend["/cosmos.bank.v1beta1.MsgMultiSend"] => MsgMultiSendResponse
    }

    fn_query! {
        pub query_balance ["/cosmos.bank.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_all_balances ["/cosmos.bank.v1beta1.Query/AllBalances"]: QueryAllBalancesRequest => QueryAllBalancesResponse
    }

    fn_query! {
        pub query_spendable_balances ["/cosmos.bank.v1beta1.Query/SpendableBalances"]: QuerySpendableBalancesRequest => QuerySpendableBalancesResponse
    }

    fn_query! {
        pub query_spendable_balances_by_denom ["/cosmos.bank.v1beta1.Query/SpendableBalanceByDenom"]: QuerySpendableBalanceByDenomRequest => QuerySpendableBalanceByDenomResponse
    }

    fn_query! {
        pub query_total_supply ["/cosmos.bank.v1beta1.Query/TotalSupply"]: QueryTotalSupplyRequest => QueryTotalSupplyResponse
    }

    fn_query! {
        pub query_supply_of ["/cosmos.bank.v1beta1.Query/SupplyOf"]: QuerySupplyOfRequest => QuerySupplyOfResponse
    }

    fn_query! {
        pub query_denom_metadata ["/cosmos.bank.v1beta1.Query/DenomMetadata"]: QueryDenomMetadataRequest => QueryDenomMetadataResponse
    }

    fn_query! {
        pub query_denom_metadata_by_query_string ["/cosmos.bank.v1beta1.Query/DenomMetadataByQueryString"]: QueryDenomMetadataByQueryStringRequest => QueryDenomMetadataByQueryStringResponse
    }

    fn_query! {
        pub query_denoms_metadata ["/cosmos.bank.v1beta1.Query/DenomsMetadata"]: QueryDenomsMetadataRequest => QueryDenomsMetadataResponse
    }

    fn_query! {
        pub query_denom_owners ["/cosmos.bank.v1beta1.Query/DenomOwners"]: QueryDenomOwnersRequest => QueryDenomOwnersResponse
    }

    fn_query! {
        pub query_denom_owners_by_query ["/cosmos.bank.v1beta1.Query/DenomOwnersByQuery"]: QueryDenomOwnersByQueryRequest => QueryDenomOwnersByQueryResponse
    }

    fn_query! {
        pub query_send_enabled ["/cosmos.bank.v1beta1.Query/SendEnabled"]: QuerySendEnabledRequest => QuerySendEnabledResponse
    }

    fn_query! {
        pub query_params ["/cosmos.bank.v1beta1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }
}
