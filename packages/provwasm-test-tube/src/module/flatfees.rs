use provwasm_std::types::provenance::flatfees::v1::{
    MsgUpdateConversionFactorRequest, MsgUpdateConversionFactorResponse, MsgUpdateMsgFeesRequest,
    MsgUpdateMsgFeesResponse, MsgUpdateParamsRequest, MsgUpdateParamsResponse,
    QueryAllMsgFeesRequest, QueryAllMsgFeesResponse, QueryMsgFeeRequest, QueryMsgFeeResponse,
    QueryParamsRequest, QueryParamsResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Flatfees<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Flatfees<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Flatfees<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub update_params: MsgUpdateParamsRequest["/provenance.flatfees.v1.MsgUpdateParamsRequest"] => MsgUpdateParamsResponse
    }

    fn_execute! {
        pub update_conversion_factor: MsgUpdateConversionFactorRequest["/provenance.flatfees.v1.MsgUpdateConversionFactorRequest"] => MsgUpdateConversionFactorResponse
    }

    fn_execute! {
        pub update_msg_fees: MsgUpdateMsgFeesRequest["/provenance.flatfees.v1.MsgUpdateMsgFeesRequest"] => MsgUpdateMsgFeesResponse
    }

    fn_query! {
        pub query_params ["/provenance.flatfees.v1.Query/Params"]: QueryParamsRequest => QueryParamsResponse
    }

    fn_query! {
        pub query_all_msg_fees ["/provenance.flatfees.v1.Query/AllMsgFees"]: QueryAllMsgFeesRequest => QueryAllMsgFeesResponse
    }

    fn_query! {
        pub query_msg_fee ["/provenance.flatfees.v1.Query/MsgFee"]: QueryMsgFeeRequest => QueryMsgFeeResponse
    }
}
