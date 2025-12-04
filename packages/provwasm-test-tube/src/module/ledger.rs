use provwasm_std::types::provenance::ledger::v1::{
    MsgAddLedgerClassBucketTypeRequest, MsgAddLedgerClassBucketTypeResponse,
    MsgAddLedgerClassEntryTypeRequest, MsgAddLedgerClassEntryTypeResponse,
    MsgAddLedgerClassStatusTypeRequest, MsgAddLedgerClassStatusTypeResponse, MsgAppendRequest,
    MsgAppendResponse, MsgBulkCreateRequest, MsgBulkCreateResponse, MsgCreateLedgerClassRequest,
    MsgCreateLedgerClassResponse, MsgCreateLedgerRequest, MsgCreateLedgerResponse,
    MsgDestroyRequest, MsgDestroyResponse, MsgTransferFundsWithSettlementRequest,
    MsgTransferFundsWithSettlementResponse, MsgUpdateBalancesRequest, MsgUpdateBalancesResponse,
    MsgUpdateInterestRateRequest, MsgUpdateInterestRateResponse, MsgUpdateMaturityDateRequest,
    MsgUpdateMaturityDateResponse, MsgUpdatePaymentRequest, MsgUpdatePaymentResponse,
    MsgUpdateStatusRequest, MsgUpdateStatusResponse, QueryLedgerBalancesAsOfRequest,
    QueryLedgerBalancesAsOfResponse, QueryLedgerClassBucketTypesRequest,
    QueryLedgerClassBucketTypesResponse, QueryLedgerClassEntryTypesRequest,
    QueryLedgerClassEntryTypesResponse, QueryLedgerClassRequest, QueryLedgerClassResponse,
    QueryLedgerClassStatusTypesRequest, QueryLedgerClassStatusTypesResponse,
    QueryLedgerEntriesRequest, QueryLedgerEntriesResponse, QueryLedgerEntryRequest,
    QueryLedgerEntryResponse, QueryLedgerRequest, QueryLedgerResponse,
    QueryLedgerSettlementsByCorrelationIdRequest, QueryLedgerSettlementsByCorrelationIdResponse,
    QueryLedgerSettlementsRequest, QueryLedgerSettlementsResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Ledger<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Ledger<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Ledger<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub create_ledger: MsgCreateLedgerRequest["/provenance.ledger.v1.MsgCreateLedgerRequest"] => MsgCreateLedgerResponse
    }

    fn_execute! {
        pub update_status: MsgUpdateStatusRequest["/provenance.ledger.v1.MsgUpdateStatusRequest"] => MsgUpdateStatusResponse
    }

    fn_execute! {
        pub update_interest_rate: MsgUpdateInterestRateRequest["/provenance.ledger.v1.MsgUpdateInterestRateRequest"] => MsgUpdateInterestRateResponse
    }

    fn_execute! {
        pub update_payment: MsgUpdatePaymentRequest["/provenance.ledger.v1.MsgUpdatePaymentRequest"] => MsgUpdatePaymentResponse
    }

    fn_execute! {
        pub update_maturity_date: MsgUpdateMaturityDateRequest["/provenance.ledger.v1.MsgUpdateMaturityDateRequest"] => MsgUpdateMaturityDateResponse
    }

    fn_execute! {
        pub append: MsgAppendRequest["/provenance.ledger.v1.MsgAppendRequest"] => MsgAppendResponse
    }

    fn_execute! {
        pub update_balances: MsgUpdateBalancesRequest["/provenance.ledger.v1.MsgUpdateBalancesRequest"] => MsgUpdateBalancesResponse
    }

    fn_execute! {
        pub transfer_funds_with_settlement: MsgTransferFundsWithSettlementRequest["/provenance.ledger.v1.MsgTransferFundsWithSettlementRequest"] => MsgTransferFundsWithSettlementResponse
    }

    fn_execute! {
        pub destroy: MsgDestroyRequest["/provenance.ledger.v1.MsgDestroyRequest"] => MsgDestroyResponse
    }

    fn_execute! {
        pub create_ledger_class: MsgCreateLedgerClassRequest["/provenance.ledger.v1.MsgCreateLedgerClassRequest"] => MsgCreateLedgerClassResponse
    }

    fn_execute! {
        pub add_ledger_class_status_type: MsgAddLedgerClassStatusTypeRequest["/provenance.ledger.v1.MsgAddLedgerClassStatusTypeRequest"] => MsgAddLedgerClassStatusTypeResponse
    }

    fn_execute! {
        pub add_ledger_class_entry_type: MsgAddLedgerClassEntryTypeRequest["/provenance.ledger.v1.MsgAddLedgerClassEntryTypeRequest"] => MsgAddLedgerClassEntryTypeResponse
    }

    fn_execute! {
        pub add_ledger_class_bucket_type: MsgAddLedgerClassBucketTypeRequest["/provenance.ledger.v1.MsgAddLedgerClassBucketTypeRequest"] => MsgAddLedgerClassBucketTypeResponse
    }

    fn_execute! {
        pub bulk_create: MsgBulkCreateRequest["/provenance.ledger.v1.MsgBulkCreateRequest"] => MsgBulkCreateResponse
    }

    fn_query! {
        pub query_ledger_class ["/provenance.ledger.v1.Query/LedgerClass"]: QueryLedgerClassRequest => QueryLedgerClassResponse
    }

    fn_query! {
        pub query_ledger_class_entry_types ["/provenance.ledger.v1.Query/LedgerClassEntryTypes"]: QueryLedgerClassEntryTypesRequest => QueryLedgerClassEntryTypesResponse
    }

    fn_query! {
        pub query_ledger_class_status_types ["/provenance.ledger.v1.Query/LedgerClassStatusTypes"]: QueryLedgerClassStatusTypesRequest => QueryLedgerClassStatusTypesResponse
    }

    fn_query! {
        pub query_ledger_class_bucket_types ["/provenance.ledger.v1.Query/LedgerClassBucketTypes"]: QueryLedgerClassBucketTypesRequest => QueryLedgerClassBucketTypesResponse
    }

    fn_query! {
        pub query_ledger ["/provenance.ledger.v1.Query/Ledger"]: QueryLedgerRequest => QueryLedgerResponse
    }

    fn_query! {
        pub query_ledger_entries ["/provenance.ledger.v1.Query/LedgerEntries"]: QueryLedgerEntriesRequest => QueryLedgerEntriesResponse
    }

    fn_query! {
        pub query_ledger_entry ["/provenance.ledger.v1.Query/LedgerEntry"]: QueryLedgerEntryRequest => QueryLedgerEntryResponse
    }

    fn_query! {
        pub query_ledger_balances_as_of ["/provenance.ledger.v1.Query/LedgerBalancesAsOf"]: QueryLedgerBalancesAsOfRequest => QueryLedgerBalancesAsOfResponse
    }

    fn_query! {
        pub query_ledger_settlements ["/provenance.ledger.v1.Query/LedgerSettlements"]: QueryLedgerSettlementsRequest => QueryLedgerSettlementsResponse
    }

    fn_query! {
        pub query_ledger_settlements_by_correlation_id ["/provenance.ledger.v1.Query/LedgerSettlementsByCorrelationID"]: QueryLedgerSettlementsByCorrelationIdRequest => QueryLedgerSettlementsByCorrelationIdResponse
    }
}
