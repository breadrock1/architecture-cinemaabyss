pub async fn send_get_request<T>(url: &str) -> anyhow::Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let response = reqwest::Client::new()
        .get(url)
        .send()
        .await?
        .error_for_status()?;

    let data = response.json::<T>().await?;
    Ok(data)
}

pub async fn send_post_request<T, U>(url: &str, body: T) -> anyhow::Result<U>
where
    T: serde::Serialize,
    U: serde::de::DeserializeOwned,
{
    let response = reqwest::Client::new()
        .post(url)
        .json(&body)
        .send()
        .await?
        .error_for_status()?;

    let data = response.json::<U>().await?;
    Ok(data)
}

pub async fn send_delete_request(url: &str) -> anyhow::Result<()> {
    let _response = reqwest::Client::new()
        .delete(url)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
