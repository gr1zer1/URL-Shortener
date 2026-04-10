use crate::{models::LinkModel};
use sqlx::{PgPool, postgres::PgDatabaseError};
use crate::errors::AppError;

pub async fn get_links(pool: &PgPool) -> Result<Vec<LinkModel>,AppError>{

    let data = sqlx::query_as!(LinkModel, "SELECT * FROM links")
        .fetch_all(pool)
        .await?;

    return Ok(data);


}

pub async fn create_link(pool: &PgPool,url:String) -> Result<LinkModel,AppError>{

    let code = nanoid::nanoid!(7);

    loop{
        let data = sqlx::query_as!(LinkModel,"INSERT INTO links ( code, url) VALUES ($1, $2) RETURNING id, code, url, created_at",code,url)
            .fetch_one(pool)
            .await;

        match data {
            Ok(link) => return Ok(link),
            Err(sqlx::Error::Database(db_err))
                if db_err
                    .downcast_ref::<PgDatabaseError>()
                    .code()
                    .starts_with("23505") =>
            {
                // unique violation — пробуем новый код
                continue;
            }
            Err(e) => return Err(AppError::DatabaseError(e)),
        }
    }

    
}

pub async fn get_link_by_code(
        pool:&PgPool,
        code:String
    ) -> Result<LinkModel,AppError>{


    let query = sqlx::query_as!(
        LinkModel,
        "SELECT id,code,url,created_at FROM links WHERE code = $1",
        code,
    )
    .fetch_one(pool)
    .await?;

    Ok(query)

    

}