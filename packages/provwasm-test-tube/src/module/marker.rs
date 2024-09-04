use provwasm_std::types::provenance::marker::v1::{
    MsgActivateRequest, MsgActivateResponse, MsgAddAccessRequest, MsgAddAccessResponse,
    MsgAddFinalizeActivateMarkerRequest, MsgAddFinalizeActivateMarkerResponse, MsgAddMarkerRequest,
    MsgAddMarkerResponse, MsgAddNetAssetValuesRequest, MsgAddNetAssetValuesResponse,
    MsgBurnRequest, MsgBurnResponse, MsgCancelRequest, MsgCancelResponse,
    MsgChangeStatusProposalRequest, MsgChangeStatusProposalResponse, MsgDeleteAccessRequest,
    MsgDeleteAccessResponse, MsgDeleteRequest, MsgDeleteResponse, MsgFinalizeRequest,
    MsgFinalizeResponse, MsgGrantAllowanceRequest, MsgGrantAllowanceResponse,
    MsgIbcTransferRequest, MsgIbcTransferResponse, MsgMintRequest, MsgMintResponse,
    MsgRemoveAdministratorProposalRequest, MsgRemoveAdministratorProposalResponse,
    MsgSetAccountDataRequest, MsgSetAccountDataResponse, MsgSetAdministratorProposalRequest,
    MsgSetAdministratorProposalResponse, MsgSetDenomMetadataProposalRequest,
    MsgSetDenomMetadataProposalResponse, MsgSetDenomMetadataRequest, MsgSetDenomMetadataResponse,
    MsgSupplyDecreaseProposalRequest, MsgSupplyDecreaseProposalResponse,
    MsgSupplyIncreaseProposalRequest, MsgSupplyIncreaseProposalResponse, MsgTransferRequest,
    MsgTransferResponse, MsgUpdateForcedTransferRequest, MsgUpdateForcedTransferResponse,
    MsgUpdateParamsRequest, MsgUpdateParamsResponse, MsgUpdateRequiredAttributesRequest,
    MsgUpdateRequiredAttributesResponse, MsgUpdateSendDenyListRequest,
    MsgUpdateSendDenyListResponse, MsgWithdrawEscrowProposalRequest,
    MsgWithdrawEscrowProposalResponse, MsgWithdrawRequest, MsgWithdrawResponse, QueryAccessRequest,
    QueryAccessResponse, QueryAccountDataRequest, QueryAccountDataResponse, QueryAllMarkersRequest,
    QueryAllMarkersResponse, QueryDenomMetadataRequest, QueryDenomMetadataResponse,
    QueryEscrowRequest, QueryEscrowResponse, QueryHoldingRequest, QueryHoldingResponse,
    QueryMarkerRequest, QueryMarkerResponse, QueryNetAssetValuesRequest,
    QueryNetAssetValuesResponse, QueryParamsRequest, QueryParamsResponse, QuerySupplyRequest,
    QuerySupplyResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Marker<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Marker<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Marker<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub grant_allowance: MsgGrantAllowanceRequest["/provenance.marker.v1.MsgGrantAllowanceRequest"] => MsgGrantAllowanceResponse
    }

    fn_execute! {
        pub add_marker: MsgAddMarkerRequest["/provenance.marker.v1.MsgAddMarkerRequest"] => MsgAddMarkerResponse
    }

    fn_execute! {
        pub add_access: MsgAddAccessRequest["/provenance.marker.v1.MsgAddAccessRequest"] => MsgAddAccessResponse
    }

    fn_execute! {
        pub delete_access: MsgDeleteAccessRequest["/provenance.marker.v1.MsgDeleteAccessRequest"] => MsgDeleteAccessResponse
    }

    fn_execute! {
        pub finalize: MsgFinalizeRequest["/provenance.marker.v1.MsgFinalizeRequest"] => MsgFinalizeResponse
    }

    fn_execute! {
        pub activate: MsgActivateRequest["/provenance.marker.v1.MsgActivateRequest"] => MsgActivateResponse
    }

    fn_execute! {
        pub cancel: MsgCancelRequest["/provenance.marker.v1.MsgCancelRequest"] => MsgCancelResponse
    }

    fn_execute! {
        pub delete: MsgDeleteRequest["/provenance.marker.v1.MsgDeleteRequest"] => MsgDeleteResponse
    }

    fn_execute! {
        pub mint: MsgMintRequest["/provenance.marker.v1.MsgMintRequest"] => MsgMintResponse
    }

    fn_execute! {
        pub burn: MsgBurnRequest["/provenance.marker.v1.MsgBurnRequest"] => MsgBurnResponse
    }

    fn_execute! {
        pub withdraw: MsgWithdrawRequest["/provenance.marker.v1.MsgWithdrawRequest"] => MsgWithdrawResponse
    }

    fn_execute! {
        pub transfer: MsgTransferRequest["/provenance.marker.v1.MsgTransferRequest"] => MsgTransferResponse
    }

    fn_execute! {
        pub ibc_transfer: MsgIbcTransferRequest["/provenance.marker.v1.MsgIbcTransferRequest"] => MsgIbcTransferResponse
    }

    fn_execute! {
        pub set_denom_metadata: MsgSetDenomMetadataRequest["/provenance.marker.v1.MsgSetDenomMetadataRequest"] => MsgSetDenomMetadataResponse
    }

    fn_execute! {
        pub add_finalize_activate: MsgAddFinalizeActivateMarkerRequest["/provenance.marker.v1.MsgAddFinalizeActivateMarkerRequest"] => MsgAddFinalizeActivateMarkerResponse
    }

    fn_execute! {
        pub supply_increase_proposal: MsgSupplyIncreaseProposalRequest["/provenance.marker.v1.MsgSupplyIncreaseProposalRequest"] => MsgSupplyIncreaseProposalResponse
    }

    fn_execute! {
        pub supply_decrease_proposal: MsgSupplyDecreaseProposalRequest["/provenance.marker.v1.MsgSupplyDecreaseProposalRequest"] => MsgSupplyDecreaseProposalResponse
    }

    fn_execute! {
        pub update_required_attributes: MsgUpdateRequiredAttributesRequest["/provenance.marker.v1.MsgUpdateRequiredAttributesRequest"] => MsgUpdateRequiredAttributesResponse
    }

    fn_execute! {
        pub update_forced_transfer: MsgUpdateForcedTransferRequest["/provenance.marker.v1.MsgUpdateForcedTransferRequest"] => MsgUpdateForcedTransferResponse
    }

    fn_execute! {
        pub set_account_data: MsgSetAccountDataRequest["/provenance.marker.v1.MsgSetAccountDataRequest"] => MsgSetAccountDataResponse
    }

    fn_execute! {
        pub update_send_deny_list: MsgUpdateSendDenyListRequest["/provenance.marker.v1.MsgUpdateSendDenyListRequest"] => MsgUpdateSendDenyListResponse
    }

    fn_execute! {
        pub add_net_asset_values: MsgAddNetAssetValuesRequest["/provenance.marker.v1.MsgAddNetAssetValuesRequest"] => MsgAddNetAssetValuesResponse
    }

    fn_execute! {
        pub set_administrator_proposal: MsgSetAdministratorProposalRequest["/provenance.marker.v1.MsgSetAdministratorProposalRequest"] => MsgSetAdministratorProposalResponse
    }

    fn_execute! {
        pub remove_administrator_proposal: MsgRemoveAdministratorProposalRequest["/provenance.marker.v1.MsgRemoveAdministratorProposalRequest"] => MsgRemoveAdministratorProposalResponse
    }

    fn_execute! {
        pub change_status_proposal: MsgChangeStatusProposalRequest["/provenance.marker.v1.MsgChangeStatusProposalRequest"] => MsgChangeStatusProposalResponse
    }

    fn_execute! {
        pub withdraw_escrow_proposal: MsgWithdrawEscrowProposalRequest["/provenance.marker.v1.MsgWithdrawEscrowProposalRequest"] => MsgWithdrawEscrowProposalResponse
    }

    fn_execute! {
        pub set_denom_metadata_proposal: MsgSetDenomMetadataProposalRequest["/provenance.marker.v1.MsgSetDenomMetadataProposalRequest"] => MsgSetDenomMetadataProposalResponse
    }

    fn_execute! {
        pub update_params: MsgUpdateParamsRequest["/provenance.marker.v1.MsgUpdateParamsRequest"] => MsgUpdateParamsResponse
    }

    fn_query! {
        pub query_params ["/provenance.marker.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_all_markers ["/provenance.marker.v1.Query/AllMarkers"]: QueryAllMarkersRequest => QueryAllMarkersResponse
    }

    fn_query! {
        pub query_marker ["/provenance.marker.v1.Query/Marker"]: QueryMarkerRequest => QueryMarkerResponse
    }

    fn_query! {
        pub query_holding ["/provenance.marker.v1.Query/Holding"]: QueryHoldingRequest => QueryHoldingResponse
    }

    fn_query! {
        pub query_supply ["/provenance.marker.v1.Query/Supply"]: QuerySupplyRequest => QuerySupplyResponse
    }

    fn_query! {
        pub query_escrow ["/provenance.marker.v1.Query/Escrow"]: QueryEscrowRequest => QueryEscrowResponse
    }

    fn_query! {
        pub query_access ["/provenance.marker.v1.Query/Access"]: QueryAccessRequest => QueryAccessResponse
    }

    fn_query! {
        pub query_denom_metadata ["/provenance.marker.v1.Query/DenomMetadata"]: QueryDenomMetadataRequest => QueryDenomMetadataResponse
    }

    fn_query! {
        pub query_account_data ["/provenance.marker.v1.Query/AccountData"]: QueryAccountDataRequest => QueryAccountDataResponse
    }

    fn_query! {
        pub query_net_asset_values ["/provenance.marker.v1.Query/NetAssetValues"]: QueryNetAssetValuesRequest => QueryNetAssetValuesResponse
    }
}
