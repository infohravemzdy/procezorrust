use crate::service_types::article_code::ArticleCode;
use crate::service_types::contract_code::ContractCode;
use crate::service_types::month_code::MonthCode;
use crate::service_types::position_code::PositionCode;
use crate::service_types::variant_code::VariantCode;

pub(crate) trait ITermSymbol {
    fn is_null(&self) -> bool;
    fn get_month_code(&self) -> MonthCode;
    fn get_contract(&self) -> ContractCode;
    fn get_position(&self) -> PositionCode;
    fn get_variant(&self) -> VariantCode;
    fn get_article(&self) -> ArticleCode;
    fn get_article_descr(&self) -> String;
}