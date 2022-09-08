use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Response<T> {
    #[serde(deserialize_with = "str_to_i64")]
    pub result: i64,
    pub description: String,
    #[serde(
        rename = "totalCount",
        deserialize_with = "str_to_opt_u64",
        skip_serializing_if = "Option::is_none"
    )]
    pub total_count: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,
    // pub data: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RespCallDtailRecordOoutboundQuery {
    #[serde(deserialize_with = "str_to_u64")]
    pub id: u64,
    #[serde(rename = "callType")]
    pub call_type: String,
    pub status: String,
    #[serde(rename = "statusCode", deserialize_with = "str_to_u64")]
    pub status_code: u64,
    pub gno: String,
    #[serde(rename = "endReason")]
    pub end_reason: String,
    #[serde(rename = "recordFile")]
    pub record_file: Vec<RecordFile>,
    #[serde(rename = "uniqueId")]
    pub unique_id: String,
    #[serde(rename = "requestUniqueId")]
    pub request_unique_id: String,
    #[serde(rename = "customerNumber")]
    pub customer_number: String,
    #[serde(rename = "customerProvince")]
    pub customer_province: String,
    #[serde(rename = "customerCity")]
    pub customer_city: String,
    #[serde(rename = "agentName")]
    pub agent_name: String,
    pub cno: String,
    #[serde(rename = "agentNumber")]
    pub agent_number: String,
    #[serde(rename = "startTime", deserialize_with = "str_to_u64")]
    pub start_time: u64,
    #[serde(rename = "endTime", deserialize_with = "str_to_u64")]
    pub end_time: u64,
    #[serde(rename = "calleeRingingTime", deserialize_with = "str_to_u64")]
    pub callee_ringing_time: u64,
    #[serde(rename = "bridgeTime", deserialize_with = "str_to_u64")]
    pub bridge_time: u64,
    #[serde(rename = "waitDuration", deserialize_with = "str_to_u64")]
    pub wait_duration: u64,
    #[serde(rename = "vadIn", deserialize_with = "str_to_u64")]
    pub vad_in: u64,
    #[serde(rename = "vadOut", deserialize_with = "str_to_u64")]
    pub vad_out: u64,
    #[serde(rename = "bridgeDuration", deserialize_with = "str_to_u64")]
    pub bridge_duration: u64,
    #[serde(rename = "totalDuration", deserialize_with = "str_to_u64")]
    pub total_duration: u64,
    #[serde(rename = "sipCause")]
    pub sip_cause: String,
    #[serde(rename = "userField")]
    pub user_field: UserField,
    pub clid: String,
    #[serde(rename = "agentClid")]
    pub agent_clid: String,
    #[serde(rename = "xNumber")]
    pub x_number: String,
    #[serde(rename = "answerTime", deserialize_with = "str_to_u64")]
    pub answer_time: u64,
    #[serde(rename = "mainRingingTime", deserialize_with = "str_to_u64")]
    pub main_ringing_time: u64,
    #[serde(rename = "hangupReason")]
    pub hangup_reason: String,
    #[serde(rename = "taskId")]
    pub task_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct RecordFile {
    pub file: String,
    pub cdr_wav_transfer_format: String,
    pub index: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub monitor_type: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct UserField {
    pub uid: String,
    pub phone: String,
    #[serde(rename = "rmsTag")]
    pub rms_tag: String,
    #[serde(rename = "sourcechannel")]
    pub source_channel: Option<String>,
    pub cdr_x1_number: String,
}

fn str_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.parse().map_err(D::Error::custom)?)
}

fn str_to_opt_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Deserialize::deserialize(deserializer)?;
    if let Some(s) = s {
        return Ok(Some(s.parse().map_err(D::Error::custom)?));
    }
    Ok(None)
}

fn str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    Ok(s.parse().map_err(D::Error::custom)?)
}
