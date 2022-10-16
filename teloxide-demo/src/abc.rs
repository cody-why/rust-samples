/*
 * @Author: plucky
 * @Date: 2022-10-13 15:24:43
 * @LastEditTime: 2022-10-13 15:24:47
 * @Description: 
 */

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    prepare_logging()?;

    info!("Initializing the Cardquest backend...");

    prepare_fs().await?;

    let cfg = if let Ok(cfg) = prepare_config().await {
        cfg
    } else {
        return Ok(());
    };

    let db = prepare_db(&cfg).await?;

    let db_clone = db.clone();
    let tg_key = cfg.telegram.api_key.clone();

    let telegram_handle: JoinHandle<anyhow::Result<()>> =
        tokio::task::spawn(async move { start_telegram(tg_key, db_clone).await });
    let api_handle = tokio::task::spawn(async move { start_api(&cfg, db).await });

    let (tg, api) = join!(telegram_handle, api_handle);

    tg??;
    api??;

    Ok(())
}
