use provwasm_std::types::provenance::asset::v1::{
    MsgBurnAsset, MsgBurnAssetResponse, MsgCreateAsset, MsgCreateAssetClass,
    MsgCreateAssetClassResponse, MsgCreateAssetResponse, MsgCreatePool, MsgCreatePoolResponse,
    MsgCreateSecuritization, MsgCreateSecuritizationResponse, MsgCreateTokenization,
    MsgCreateTokenizationResponse, QueryAssetClassRequest, QueryAssetClassResponse,
    QueryAssetClassesRequest, QueryAssetClassesResponse, QueryAssetRequest, QueryAssetResponse,
    QueryAssetsRequest, QueryAssetsResponse,
};
use test_tube_prov::{fn_execute, fn_query, Module, Runner};

pub struct Asset<'a, R: Runner<'a>> {
    runner: &'a R,
}

impl<'a, R: Runner<'a>> Module<'a, R> for Asset<'a, R> {
    fn new(runner: &'a R) -> Self {
        Self { runner }
    }
}

impl<'a, R> Asset<'a, R>
where
    R: Runner<'a>,
{
    fn_execute! {
        pub burn_asset: MsgBurnAsset["/provenance.asset.v1.MsgBurnAsset"] => MsgBurnAssetResponse
    }

    fn_execute! {
        pub create_asset: MsgCreateAsset["/provenance.asset.v1.MsgCreateAsset"] => MsgCreateAssetResponse
    }

    fn_execute! {
        pub create_asset_class: MsgCreateAssetClass["/provenance.asset.v1.MsgCreateAssetClass"] => MsgCreateAssetClassResponse
    }

    fn_execute! {
        pub create_pool: MsgCreatePool["/provenance.asset.v1.MsgCreatePool"] => MsgCreatePoolResponse
    }

    fn_execute! {
        pub create_tokenization: MsgCreateTokenization["/provenance.asset.v1.MsgCreateTokenization"] => MsgCreateTokenizationResponse
    }

    fn_execute! {
        pub create_securitization: MsgCreateSecuritization["/provenance.asset.v1.MsgCreateSecuritization"] => MsgCreateSecuritizationResponse
    }

    fn_query! {
        pub query_asset ["/provenance.asset.v1.Query/Asset"]: QueryAssetRequest => QueryAssetResponse
    }

    fn_query! {
        pub query_assets ["/provenance.asset.v1.Query/Assets"]: QueryAssetsRequest => QueryAssetsResponse
    }

    fn_query! {
        pub query_asset_class ["/provenance.asset.v1.Query/AssetClass"]: QueryAssetClassRequest => QueryAssetClassResponse
    }

    fn_query! {
        pub query_asset_classes ["/provenance.asset.v1.Query/AssetClasses"]: QueryAssetClassesRequest => QueryAssetClassesResponse
    }
}
