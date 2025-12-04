use provwasm_std::types::provenance::registry::v1::{
    MsgGrantRole, MsgGrantRoleResponse, MsgRegisterNft, MsgRegisterNftResponse,
    MsgRegistryBulkUpdate, MsgRegistryBulkUpdateResponse, MsgRevokeRole, MsgRevokeRoleResponse,
    MsgUnregisterNft, MsgUnregisterNftResponse, QueryGetRegistryRequest, QueryGetRegistryResponse,
    QueryHasRoleRequest, QueryHasRoleResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Registry<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Registry<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Registry<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub register_nft: MsgRegisterNft["/provenance.registry.v1.MsgRegisterNFT"] => MsgRegisterNftResponse
    }

    fn_execute! {
        pub grant_role: MsgGrantRole["/provenance.registry.v1.MsgGrantRole"] => MsgGrantRoleResponse
    }

    fn_execute! {
        pub revoke_role: MsgRevokeRole["/provenance.registry.v1.MsgRevokeRole"] => MsgRevokeRoleResponse
    }

    fn_execute! {
        pub unregister_nft: MsgUnregisterNft["/provenance.registry.v1.MsgUnregisterNFT"] => MsgUnregisterNftResponse
    }

    fn_execute! {
        pub registry_bulk_update: MsgRegistryBulkUpdate["/provenance.registry.v1.MsgRegistryBulkUpdate"] => MsgRegistryBulkUpdateResponse
    }

    fn_query! {
        pub query_get_registry ["/provenance.registry.v1.Query/GetRegistry"]: QueryGetRegistryRequest => QueryGetRegistryResponse
    }

    fn_query! {
        pub query_has_role ["/provenance.registry.v1.Query/HasRole"]: QueryHasRoleRequest => QueryHasRoleResponse
    }
}
