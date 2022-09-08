use crate::{ParamsChecker, ValidateType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ParamsCdrObQuery {
    /// 验证类型 	取值1，2； 取值说明：
    /// validateType=1时使用部门编号(departmentId)进行验证；
    /// validateType=2时使用呼叫中心编号(enterpriseId)进行验证；
    #[serde(rename = "validateType")]
    pub validate_type: ValidateType,
    /// 部门编号 	说明：validateType=1时，此参数为必选参数，例如：departmentId=BM0000001；
    #[serde(rename = "departmentId", skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 呼叫中心编号 	说明：validateType=2时，此参数为必选参数，例如：enterpriseId=7000101；
    #[serde(rename = "enterpriseId", skip_serializing_if = "Option::is_none")]
    pub enterprise_id: Option<String>,
    /// 当前时间戳 	说明：取当前linux时间戳，精确到s，时间戳有效期为30分钟
    #[serde(rename = "timestamp")]
    pub timestamp: u64,
    /// 验证值 	说明：
    /// validateType=1时，sign=MD5({departmentId}+{timestamp}+{部门token值})；
    /// validateType=2时，sign=MD5({enterpriseId}+{timestamp}+{部门token值})；
    /// 格式要求：32位全小写
    pub sign: String,
    // 电话唯一标识 	如果uniqueId值不为空，则以下其它参数无效，即仅查询电话唯一标识为uniqueId的记录信息
    #[serde(rename = "uniqueId", skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    ///请求唯一id 	查询请求唯一id对应的记录信息，默认查询当前月，如requestUniqueId不属于当前月，查询时请传递查询参数 查询开始时间(startTime)
    #[serde(rename = "requestUniqueId", skip_serializing_if = "Option::is_none")]
    pub request_unique_id: Option<String>,
    /// 呼叫类型 	为空表示全部，可选值为：4:预览外呼 6:主叫外呼 9:内部呼叫
    #[serde(rename = "callType", skip_serializing_if = "Option::is_none")]
    pub call_type: Option<i32>,
    /// 呼叫结果 	参数说明：30 座席未接听; 31 座席接听,未呼叫客户; 32 座席接听,客户未接听; 33 双方接听; 50 主叫外呼接听; 51 主叫外呼,客户未接听; 52 主叫外呼,双方接听。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 客户电话归属省份 	为空表示全部，如"四川"，此参数需要URLEncode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub province: Option<String>,
    /// 客户电话归属城市 	为空表示全部，如"成都"，此参数需要URLEncode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 查询时间开始结束范围生效类型
    /// 通话记录startTime和endTime时间范围生效类型，
    /// 当startTime和endTime不为空时生效；
    /// 取值：1.呼叫开始时间 2.呼叫结束时间；
    /// 默认为1
    #[serde(rename = "timeRangeType", skip_serializing_if = "Option::is_none")]
    pub time_range_type: Option<i32>,
    /// 查询开始时间 	时间戳格式,精确到s。startTime与endTime不允许跨月，默认值取当前月份第一天
    #[serde(rename = "startTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 查询结束时间 	时间戳格式,精确到s。startTime与endTime不允许跨月，默认值取当前月份最后一天
    #[serde(rename = "endTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// 从第几条开始取 	大于等于0，默认0
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// 需要取出的数据条数 	大于0，最大不能超过1000，默认10
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u64>,
    /// 用户自定义参数指定查询key 	传递该参数时必须传递userFieldValue，仅查询一个字段，不支持模糊匹配，构造的自定义字段查询条件为:{"userFieldkey":"userFieldvalue"}，此参数需要URLEncode
    #[serde(rename = "userFieldkey", skip_serializing_if = "Option::is_none")]
    pub user_fieldkey: Option<String>,
    /// 用户自定义参数指定查询value 	传递该参数时必须传递userFieldKey，仅查询一个字段，不支持模糊匹配，构造的自定义字段查询条件为:{"userFieldkey":"userFieldvalue"}，此参数需要URLEncode
    #[serde(rename = "userFieldValue", skip_serializing_if = "Option::is_none")]
    pub user_field_value: Option<String>,
    /// 客户号码 	客户号码
    #[serde(rename = "customerNumber", skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<String>,
    /// 座席工号 	座席工号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cno: Option<String>,
    /// 座席所属外呼组 单个外呼组号2-20位，支持一个或多个座席工号查询，多个座席工号以英文逗号隔开，一次最多支持10个。不支持模糊匹配。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gno: Option<String>,
    /// 座席号码 	座席号码
    #[serde(rename = "agentNumber", skip_serializing_if = "Option::is_none")]
    pub agent_number: Option<String>,
    /// 座席姓名 	座席姓名
    #[serde(rename = "agentName", skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// 外显号码 	外显号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clid: Option<String>,
    /// 取值说明：0: 按照记录产生的时序顺序排序，1：按照记录产生的时序逆序排序，2: 按照结束时间的时序顺序排序，3：按照结束时间的时序逆序排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i32>,
    /// 是否返回总条数 	取值说明：0: 不返回，1：返回，不传默认为1
    #[serde(rename = "returnCount", skip_serializing_if = "Option::is_none")]
    pub return_count: Option<i32>,
}

impl ParamsChecker for ParamsCdrObQuery {
    fn check(&self) -> crate::Result<()> {
        use crate::Error;
        use ValidateType::*;
        match self.validate_type {
            Department => {
                if self.department_id.is_none() {
                    return Err(Error::General("部门编号(departmentId)未设置".to_string()));
                }
            }
            Enterprise => {
                if self.enterprise_id.is_none() {
                    return Err(Error::General(
                        "呼叫中心编号(enterpriseId)未设置".to_string(),
                    ));
                }
            }
        };
        Ok(())
    }
}
