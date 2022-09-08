//! 外呼模块
use async_trait::async_trait;

use crate::Result;

/// 外呼管理
#[async_trait]
pub trait OutboundManager {
    /// 获取座席外呼通话记录接口
    /// Call Detail Record Outbound query
    /// https://wiki.cticloud.cn/?page=/html/wiki/API/通话记录/外呼/获取座席外呼通话记录接口.html
    /// https://api-6.cticloud.cn/interface/v10/cdr/ob/query
    async fn cdr_ob_query(
        &self,
        params: ParamsCdrObQuery,
    ) -> Result<Response<RespCallDtailRecordOoutboundQuery>>;
}

/// 接口实现
mod outbound;

mod dto;
pub use dto::*;

mod model;
pub use model::*;
