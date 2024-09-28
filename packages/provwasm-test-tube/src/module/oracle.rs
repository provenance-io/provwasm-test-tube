use provwasm_std::types::provenance::oracle::v1::{
    MsgSendQueryOracleRequest, MsgSendQueryOracleResponse, MsgUpdateOracleRequest,
    MsgUpdateOracleResponse, QueryOracleAddressRequest, QueryOracleAddressResponse,
    QueryOracleRequest, QueryOracleResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Oracle<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Oracle<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Oracle<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub update_oracle: MsgUpdateOracleRequest["/provenance.name.v1.MsgUpdateOracleRequest"] => MsgUpdateOracleResponse
    }

    fn_execute! {
        pub send_query_oracle: MsgSendQueryOracleRequest["/provenance.name.v1.MsgSendQueryOracleRequest"] => MsgSendQueryOracleResponse
    }

    fn_query! {
        pub query_oracle_address ["/provenance.name.v1.Query/OracleAddress"]: QueryOracleAddressRequest => QueryOracleAddressResponse
    }

    fn_query! {
        pub query_oracle ["/provenance.name.v1.Query/Oracle"]: QueryOracleRequest => QueryOracleResponse
    }
}
