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
    url:String,
    code:String,
) -> Result<(),AppError>{

    let mut conn = client.get_multiplexed_async_connection().await?;

    Ok(conn.set_ex(code,url,3600).await?)

}