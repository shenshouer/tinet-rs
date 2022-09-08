use async_trait::async_trait;

use super::*;
use crate::{utils::timestamp, Client, Error, Result};
use log::{debug, error};

#[async_trait]
impl OutboundManager for Client {
    async fn cdr_ob_query(
        &self,
        params: ParamsCdrObQuery,
    ) -> Result<Response<RespCallDtailRecordOoutboundQuery>> {
        let url = self.api_url("/cdr/ob/query");
        debug!("{url}");

        let mut params = params;
        params.timestamp = timestamp().as_secs();
        params.validate_type = self.validate_type.clone();
        params.enterprise_id = Some(self.enterprise_id.clone());
        params.sign = format!(
            "{:x}",
            md5::compute(format!(
                "{}{}{}",
                params.enterprise_id.clone().unwrap(),
                params.timestamp,
                self.token
            ))
        );
        debug!("{}", &params.sign);
        let query = serde_json::to_value(params)?;
        let client = reqwest::Client::builder().build()?;

        let req_builder = client.get(&url);
        let req_builder = req_builder.query(&query);
        let mut req = req_builder.build()?;

        let method = req.method_mut();
        *method = reqwest::Method::GET;

        let resp = client.execute(req).await?;
        let status_code = resp.status();
        if !status_code.is_success() {
            let msg = resp.text().await?;
            let err_msg = format!("request {url} error: {msg}");
            error!("{err_msg}");
            return Err(Error::General(err_msg));
        }

        let data = resp
            .json::<Response<RespCallDtailRecordOoutboundQuery>>()
            .await?;
        Ok(data)

        // 测试验证
        // let data = resp.text().await?;
        // println!("{}", data);
        // let x = Response::default();
        // Ok(x)
    }
}
