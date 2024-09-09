use provwasm_std::types::provenance::attribute::v1::{
    MsgAddAttributeRequest, MsgAddAttributeResponse, MsgDeleteAttributeRequest,
    MsgDeleteAttributeResponse, MsgDeleteDistinctAttributeRequest, MsgSetAccountDataRequest,
    MsgSetAccountDataResponse, MsgUpdateAttributeExpirationRequest,
    MsgUpdateAttributeExpirationResponse, MsgUpdateAttributeRequest, MsgUpdateAttributeResponse,
    MsgUpdateParamsRequest, MsgUpdateParamsResponse, QueryAttributeRequest, QueryAttributeResponse,
    QueryAttributesRequest, QueryAttributesResponse, QueryParamsRequest, QueryParamsResponse,
    QueryScanRequest, QueryScanResponse,
};

use test_tube_prov::module::Module;
use test_tube_prov::{fn_execute, fn_query, Runner};

pub struct Attribute<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Attribute<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Attribute<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub add_attribute: MsgAddAttributeRequest["/provenance.attribute.v1.MsgAddAttributeRequest"] => MsgAddAttributeResponse
    }

    fn_execute! {
        pub update_attribute_request: MsgUpdateAttributeRequest["/provenance.attribute.v1.MsgAddAttributeRequest"] => MsgUpdateAttributeResponse
    }

    fn_execute! {
        pub update_expiration: MsgUpdateAttributeExpirationRequest["/provenance.attribute.v1.MsgUpdateAttributeExpirationRequest"] => MsgUpdateAttributeExpirationResponse
    }

    fn_execute! {
        pub delete_attribute: MsgDeleteAttributeRequest["/provenance.attribute.v1.MsgDeleteAttributeRequest"] => MsgDeleteAttributeResponse
    }

    fn_execute! {
        pub delete_distinct_attribute: MsgDeleteDistinctAttributeRequest["/provenance.attribute.v1.MsgDeleteDistinctAttributeRequest"] => MsgAddAttributeResponse
    }

    fn_execute! {
        pub set_account_data: MsgSetAccountDataRequest["/provenance.attribute.v1.MsgSetAccountDataRequest"] => MsgSetAccountDataResponse
    }

    fn_execute! {
        pub update_params: MsgUpdateParamsRequest["/provenance.attribute.v1.MsgUpdateParamsRequest"] => MsgUpdateParamsResponse
    }

    fn_query! {
        pub query_attribute ["/provenance.attribute.v1.Query/Attribute"]: QueryAttributeRequest => QueryAttributeResponse
    }

    fn_query! {
        pub query_attributes ["/provenance.attribute.v1.Query/Attributes"]: QueryAttributesRequest => QueryAttributesResponse
    }

    fn_query! {
        pub query_params ["/provenance.attribute.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_scan ["/provenance.attribute.v1.Query/Scan"]: QueryScanRequest => QueryScanResponse
    }

    fn_query! {
        pub query_ ["/provenance.attribute.v1.Query/AttributeAccounts"]: QueryScanRequest => QueryScanResponse
    }

    fn_query! {
        pub query_scan ["/provenance.attribute.v1.Query/AccountData"]: QueryScanRequest => QueryScanResponse
    }
}
