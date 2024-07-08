use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 非同期関数の完了を待ちつつ、Result型のエラーハンドリングも行います。エラーが発生した場合には関数を終了し、エラーを呼び出し元に返します。
    let result = some_async_function().await?;
    println!("Result: {}", &result);
    Ok(())
}

async fn some_async_function() -> Result<u32, Box<dyn Error>> {
    Ok(42)
}
