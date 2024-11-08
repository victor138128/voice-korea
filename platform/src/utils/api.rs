use dioxus_logger::tracing;

pub async fn get<T>(url: &str) -> T
where
    T: Default + serde::de::DeserializeOwned,
{
    match reqwest::get(format!(
        "{}/api{}",
        option_env!("BASE_URL").unwrap_or("https://voice-korea.dev.biyard.co/"),
        url
    ))
    .await
    {
        Ok(res) => match res.json::<T>().await {
            Ok(v) => v,
            Err(e) => {
                tracing::error!("Failed to parse {}: {e:?}", url);
                T::default()
            }
        },
        Err(e) => {
            tracing::error!("Failed to get {}: {e:?}", url);
            T::default()
        }
    }
}
