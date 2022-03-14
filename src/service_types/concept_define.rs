use crate::service_types::concept_code::ConceptCode;

pub trait IConceptDefine {
    fn get_code(&self) -> ConceptCode;
}