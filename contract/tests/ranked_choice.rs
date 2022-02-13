use workspaces::prelude::*;

#[tokio::test]
async fn test_ranked_choice_winner() -> anyhow::Result<()> {
    let mut total_gas_burnt = 0;
    let worker = workspaces::sandbox();

    // Create new accounts called Alice and Bob that we can use later
    // to test our voting contract.
    let alice = worker.dev_create_account().await?;
    let bob = worker.dev_create_account().await?;

    // Deploy our contract and initialize it with some candidates.
    let contract = worker
        .dev_deploy(include_bytes!("../../out/main.wasm"))
        .await?;

    contract
        .call(&worker, "new")
        .args_json(serde_json::json!({
            "candidates": ["A".to_string(), "B".to_string(), "C".to_string()],
        }))?
        .transact()
        .await?;
    println!("Initialized contract");

    // Alice votes for camdidates "C", "B", "A" in that order.
    let call_result = alice
        .call(&worker, contract.id(), "vote")
        .args_json(serde_json::json!({
            "order": ["C".to_string(), "B".to_string(), "A".to_string()],
        }))?
        .transact()
        .await?;
    total_gas_burnt += call_result.total_gas_burnt;

    // Bob votes for camdidates "C", "A", "B" in that order.
    let call_result = bob
        .call(&worker, contract.id(), "vote")
        .args_json(serde_json::json!({
            "order": ["C".to_string(), "A".to_string(), "B".to_string()],
        }))?
        .transact()
        .await?;
    total_gas_burnt += call_result.total_gas_burnt;
    println!("Total gas burnt: {} yoctoNear", total_gas_burnt);

    // Our winner should be "C" since both Alice and Bob chose "C" as their
    // first choice.
    let winner = contract
        .call(&worker, "get_winner")
        .view()
        .await?
        .json::<String>()?;

    println!("Our winner is: {}", winner);
    assert_eq!(winner, "C");

    Ok(())
}

#[tokio::test]
async fn test_ranked_choice_handle_error() -> anyhow::Result<()> {
    let worker = workspaces::sandbox();
    let alice = worker.dev_create_account().await?;

    let contract = worker
        .dev_deploy(include_bytes!("../../out/main.wasm"))
        .await?;

    contract
        .call(&worker, "new")
        .args_json(serde_json::json!({
            "candidates": ["A".to_string(), "B".to_string(), "C".to_string()],
        }))?
        .transact()
        .await?;

    // Alice votes for camdidates "C", "B", "D" in that order. But "D" is not
    // a candidate. Our call should return us an error!
    let result = alice
        .call(&worker, contract.id(), "vote")
        .args_json(serde_json::json!({
            "order": ["C".to_string(), "B".to_string(), "D".to_string()],
        }))?
        .transact()
        .await?;
    assert!(result.is_failure());
    println!("{:?}", result.status);

    Ok(())
}
