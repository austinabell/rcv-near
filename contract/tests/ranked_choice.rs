use workspaces::prelude::*;

#[tokio::test]
async fn test_ranked_choice_winner() -> anyhow::Result<()> {
    let mut total_gas_burnt = 0;
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
    println!("Initialized contract");

    let call_result = alice
        .call(&worker, contract.id(), "vote")
        .args_json(serde_json::json!({
            "order": ["C".to_string(), "B".to_string(), "A".to_string()],
        }))?
        .transact()
        .await?;
    total_gas_burnt += call_result.total_gas_burnt;

    let bob = worker.dev_create_account().await?;
    let call_result = bob
        .call(&worker, contract.id(), "vote")
        .args_json(serde_json::json!({
            "order": ["C".to_string(), "A".to_string(), "B".to_string()],
        }))?
        .transact()
        .await?;
    total_gas_burnt += call_result.total_gas_burnt;

    println!("Total gas burnt: {} yoctoNear", total_gas_burnt);

    let winner = contract
        .call(&worker, "get_winner")
        .view()
        .await?
        .json::<String>()?;

    println!("Our winner is: {}", winner);
    assert_eq!(winner, "C");

    Ok(())
}
