use redis::{AsyncCommands,Client};
use crate::errors::AppError;


pub async fn get_cached_url(
    client:&Client,
    code:String,
    ) -> Result<Option<String>,AppError>{

    let mut conn = client.get_multiplexed_async_connection().await?;

    let url:Option<String> = conn.get(code).await?;

    Ok(url)
}

pub async fn set_cached_url(
    client:&Client,
    code:String,
    url:String,
) -> Result<(),AppError>{

    let ttl = std::env::var("REDIS_EXP")
        .expect("REDIS_EXP must be set")
        .parse::<u64>()
        .expect("Parse Error");

    let mut conn = client.get_multiplexed_async_connection().await?;

    Ok(conn.set_ex(code,url,ttl).await?)

}

pub async fn add_click(
    code:String,
    client:&Client,
) -> Result<(),AppError>{
    let mut conn = client.get_multiplexed_async_connection().await?;
    
    let payload = serde_json::to_string(&serde_json::json!({
        "code": code,
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))?;

    conn.lpush::<_, _, ()>("analytics:clicks", payload).await?;
    
    Ok(())

}