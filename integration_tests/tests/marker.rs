use cosmwasm_std::{coin, Coin, Uint128};
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, RunnerError};

use marker::msg::{ExecuteMsg, InitMsg, QueryMsg};
use marker::types::Marker;
use provwasm_test_tube::provwasm_std::types::provenance::marker::v1::{
    Access, AccessGrant, MarkerAccount, MarkerStatus, MarkerType, MsgAddMarkerRequest,
    QueryMarkerRequest,
};

#[test]
fn create_and_withdraw() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::new();
    let accs = app.init_accounts(&[coin(100_000_000_000_000, "nhash")], 1)?;
    let admin = &accs[0];

    let wasm = Wasm::new(&app);
    let wasm_byte_code =
        std::fs::read(format!("{}/wasm/marker.wasm", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                name: "marker-test.sc.pb".to_string(),
            },
            Some(&admin.address()),
            Some("marker test"),
            &[],
            admin,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Create {
            supply: Uint128::new(100),
            denom: "spy".into(),
            allow_forced_transfer: false,
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::GrantAccess {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Finalize {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Activate {
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::Withdraw {
            amount: Uint128::new(20),
            denom: "spy".into(),
        },
        &[],
        admin,
    )?;

    let marker = wasm.query::<QueryMsg, Marker>(
        &contract_addr,
        &QueryMsg::GetByDenom {
            denom: "spy".into(),
        },
    )?;

    assert_eq!(marker.marker_account.denom, "spy");

    Ok(())
}

#[test]
fn custom_module() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::new();
    let accs = app.init_accounts(&[coin(100_000_000_000_000, "nhash")], 1)?;
    let admin = &accs[0];

    let marker_module = provwasm_test_tube::marker::Marker::new(&app);
    marker_module.add_marker(
        MsgAddMarkerRequest {
            amount: Some(
                Coin {
                    amount: Uint128::new(100),
                    denom: "spy".to_string(),
                }
                .into(),
            ),
            manager: admin.address(),
            from_address: admin.address(),
            status: MarkerStatus::Proposed.into(),
            marker_type: MarkerType::Coin.into(),
            access_list: vec![AccessGrant {
                address: admin.address(),
                permissions: vec![
                    Access::Admin.into(),
                    Access::Burn.into(),
                    Access::Deposit.into(),
                    Access::Delete.into(),
                    Access::Mint.into(),
                    Access::Withdraw.into(),
                ],
            }],
            supply_fixed: false,
            allow_governance_control: false,
            allow_forced_transfer: false,
            required_attributes: vec![],
            usd_cents: 0,
            volume: 0,
            usd_mills: 0,
        },
        admin,
    )?;

    let marker_response = marker_module.query_marker(&QueryMarkerRequest {
        id: "spy".to_string(),
    })?;

    assert_eq!(
        MarkerAccount::try_from(marker_response.marker.unwrap())
            .unwrap()
            .denom,
        "spy"
    );

    Ok(())
}
