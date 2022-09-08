mod client;
pub use client::{Client, ParamsChecker, ValidateType};

/// 错误定义
mod err;
pub use err::{Error, Result};

/// CDR 通话记录管理
mod call_detail_record;
pub use call_detail_record::*;

mod utils;
pub use utils::*;
