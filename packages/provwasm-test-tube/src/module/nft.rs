use provwasm_std::types::cosmos::nft::v1beta1::{
    MsgSend, MsgSendResponse, QueryBalanceRequest, QueryBalanceResponse, QueryClassRequest,
    QueryClassResponse, QueryClassesRequest, QueryClassesResponse, QueryNftRequest, QueryNftResponse,
    QueryNftsRequest, QueryNftsResponse, QueryOwnerRequest, QueryOwnerResponse, QuerySupplyRequest,
    QuerySupplyResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Nft<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Nft<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Nft<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub send: MsgSend["/cosmos.nft.v1beta1.MsgSend"] => MsgSendResponse
    }

    fn_query! {
        pub query_balance ["/cosmos.nft.v1beta1.Query/Balance"]: QueryBalanceRequest => QueryBalanceResponse
    }

    fn_query! {
        pub query_class ["/cosmos.nft.v1beta1.Query/Class"]: QueryClassRequest => QueryClassResponse
    }

    fn_query! {
        pub query_classes ["/cosmos.nft.v1beta1.Query/Classes"]: QueryClassesRequest => QueryClassesResponse
    }

    fn_query! {
        pub query_nft ["/cosmos.nft.v1beta1.Query/NFT"]: QueryNftRequest => QueryNftResponse
    }

    fn_query! {
        pub query_nfts ["/cosmos.nft.v1beta1.Query/NFTs"]: QueryNftsRequest => QueryNftsResponse
    }

    fn_query! {
        pub query_owner ["/cosmos.nft.v1beta1.Query/Owner"]: QueryOwnerRequest => QueryOwnerResponse
    }

    fn_query! {
        pub query_supply ["/cosmos.nft.v1beta1.Query/Supply"]: QuerySupplyRequest => QuerySupplyResponse
    }
}

