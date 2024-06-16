use anyhow::Result;
// use sqlx -- to readyset

#[tokio::main]
async fn main() -> Result<()> {
    let time = std::time::Instant::now();
    // insert_data().await?;
    execute_query().await?;
    println!("Time elapsed: {}ms", time.elapsed().as_millis());
    Ok(())
}

async fn get_pool() -> Result<sqlx::postgres::PgPool> {
    let url = include_str!("../.env").trim_start_matches("DATABASE_URL=");
    let pool = sqlx::postgres::PgPool::connect(url).await?;
    Ok(pool)
}

async fn execute_query() -> Result<i64> {
    let pool = get_pool().await?;
    let sql = "SELECT count(*) FROM title_ratings JOIN title_basics ON title_ratings.tconst = title_basics.tconst WHERE title_basics.startyear = 1990 AND title_ratings.averagerating > 0;";
    let rows: (i64,) = sqlx::query_as(sql).fetch_one(&pool).await?;
    let result = rows.0;

    println!("Result: {}", result);
    Ok(result)
}

async fn insert_data() -> Result<u64> {
    let pool = get_pool().await?;
    let sql = "INSERT INTO title_ratings (tconst, averagerating) VALUES ($1, $2)";
    let tconst = "tt0000001";
    let averagerating = 8.5;
    let row_id: u64 = sqlx::query(sql)
       .bind(tconst)
       .bind(averagerating)
       .execute(&pool)
       .await?
       .rows_affected();

    println!("Row ID: {}", row_id);
    Ok(row_id)
}