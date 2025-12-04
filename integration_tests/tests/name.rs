use cosmwasm_std::coin;
use name::msg::{ExecuteMsg, InitMsg, LookupResponse, QueryMsg};
use provwasm_test_tube::wasm::Wasm;
use provwasm_test_tube::{Account, Module, ProvwasmTestApp, ProvwasmTestAppOptions, RunnerError};

#[test]
fn bind_unbind_prefix() -> Result<(), RunnerError> {
    let app = ProvwasmTestApp::new_with_options(ProvwasmTestAppOptions {
        chain_id: "testchain".to_string(),
        address_prefix: "tp".to_string(),
        fee_denom: "nhash".to_string(),
        load_msg_fees: false,
    });
    let accs = app.init_accounts(&[coin(100_000_000_000_000, "nhash")], 1)?;
    let admin = &accs[0];

    let wasm = Wasm::new(&app);
    let wasm_byte_code =
        std::fs::read(format!("{}/wasm/name.wasm", env!("CARGO_MANIFEST_DIR"))).unwrap();
    let store_res = wasm.store_code(&wasm_byte_code, None, admin);
    let code_id = store_res?.data.code_id;
    assert_eq!(code_id, 1);

    // let init_admins = vec![admin.address()];
    let contract_addr = wasm
        .instantiate(
            code_id,
            &InitMsg {
                name: "name-test.sc.pb".to_string(),
            },
            Some(&admin.address()),
            Some("name test"),
            &[],
            admin,
        )?
        .data
        .address;

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::BindPrefix {
            prefix: "nm".to_string(),
        },
        &[],
        admin,
    )?;
    let resolve_response = wasm.query::<QueryMsg, String>(
        &contract_addr,
        &QueryMsg::Resolve {
            name: "nm.name-test.sc.pb".to_string(),
        },
    )?;
    assert_eq!(resolve_response, contract_addr);

    let lookup_response = wasm.query::<QueryMsg, LookupResponse>(
        &contract_addr,
        &QueryMsg::Lookup {
            address: contract_addr.clone(),
        },
    )?;
    assert_eq!(lookup_response.name.len(), 2);
    assert!(lookup_response
        .name
        .contains(&"name-test.sc.pb".to_string()));
    assert!(lookup_response
        .name
        .contains(&"nm.name-test.sc.pb".to_string()));

    wasm.execute::<ExecuteMsg>(
        &contract_addr,
        &ExecuteMsg::UnbindPrefix {
            prefix: "nm".to_string(),
        },
        &[],
        admin,
    )?;
    let resolve_response = wasm.query::<QueryMsg, String>(
        &contract_addr,
        &QueryMsg::Resolve {
            name: "nm.name-test.sc.pb".to_string(),
        },
    );
    assert!(resolve_response.is_err());

    let lookup_response = wasm.query::<QueryMsg, LookupResponse>(
        &contract_addr,
        &QueryMsg::Lookup {
            address: contract_addr.clone(),
        },
    )?;
    assert_eq!(lookup_response.name.len(), 1);
    assert!(lookup_response
        .name
        .contains(&"name-test.sc.pb".to_string()));

    Ok(())
}
