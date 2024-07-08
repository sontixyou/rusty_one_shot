#[tokio::main]
async fn main() {
    // awaitは、非同期関数の結果を待つためのキーワードです。async関数やブロックから呼び出され、非同期で処理が完了するまで実行を停止します。これにより、他のタスクが並行して実行されることが可能になります。
    let result = some_async_function().await;
    println!("Result: {:?}", result);
}

async fn some_async_function() -> u32 {
    42
}
