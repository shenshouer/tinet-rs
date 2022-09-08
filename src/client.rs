/// 基础API url
const BASE_URL: &str = "https://api-6.cticloud.cn/interface/v10";

use crate::Result;
use serde_repr::*;

/// 参数检查接口
pub trait ParamsChecker {
    fn check(&self) -> Result<()>;
}

pub struct Client {
    pub enterprise_id: String,
    pub validate_type: ValidateType,
    pub token: String,
}

impl Client {
    pub fn new(enterprise_id: String, validate_type: ValidateType, token: String) -> Self {
        Client {
            enterprise_id,
            validate_type,
            token,
        }
    }

    pub(crate) fn api_url(&self, path: &str) -> String {
        format!("{BASE_URL}/{path}")
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Default, Clone)]
#[serde(untagged)]
#[repr(u8)]
pub enum ValidateType {
    Department = 1,
    #[default]
    Enterprise = 2,
}
