use chrono::{Local, TimeZone};
use dotenv::dotenv;
use std::env;
use tinet::{Client, OutboundManager, ParamsCdrObQuery, ValidateType};
use tracing::debug;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let enterprise_id = env::var("ENTERPRISE_ID")?;
    let token = env::var("TOKEN")?;
    debug!("ENTERPRISE_ID: {enterprise_id}, TOKEN: {token}");

    // let today = Local::today();
    // let start_date = Local.ymd(2022, 1, 1);
    let start_time = Local.ymd(2022, 1, 1).and_hms(0, 0, 0);
    let end_time = Local.ymd(2022, 1, 31).and_hms(23, 59, 59);

    let params = ParamsCdrObQuery {
        start_time: Some(start_time.timestamp()),
        end_time: Some(end_time.timestamp()),
        ..Default::default()
    };

    let client = Client::new(enterprise_id, ValidateType::Enterprise, token);
    let x = client.cdr_ob_query(params).await?;
    debug!("{:?}", x.total_count);

    Ok(())
}
