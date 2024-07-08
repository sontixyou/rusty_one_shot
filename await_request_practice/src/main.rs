use reqwest::Error;

#[tokio::main]
async fn main() -> Result<(), Option<Error>> {
    let response = make_request("https://xjsonplaceholder.typicode.com/posts/1").await?;
    println!("Response: {:?}", response);
    Ok(())
}

async fn make_request(url: &str) -> Result<String, Option<Error>> {
    // HTTP GETリクエストを実行
    let response = reqwest::get(url).await?;

    // レスポンスのステータスコードをチェックし、エラーハンドリング
    if response.status().is_success() {
        // レスポンスボディを文字列として取得
        let body = response.text().await?;
        Ok(body)
    } else {
        // ステータスコードが成功でない場合はエラーを返す
        return Err(response.error_for_status_ref().err());
    }
}
