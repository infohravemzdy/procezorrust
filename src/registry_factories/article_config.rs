use legalios::service::period::IPeriod;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_providers::article_provider::{ArticleSpec, ArticleSpecProvider, IArticleSpec, IArticleSpecProvider};

#[derive(Debug, Clone)]
pub(crate) struct ArticleSpecConfig {
    spec: ArticleSpec,
}

impl IArticleDefine for ArticleSpecConfig {
    fn get_code(&self) -> ArticleCode {
        self.spec.get_code()
    }
    fn get_role(&self) -> ConceptCode {
        self.spec.get_role()
    }
}

impl IArticleSpec for ArticleSpecConfig {
    fn get_sums(&self) -> Vec<ArticleCode> {
        self.spec.get_sums()
    }

    fn get_defs(&self) -> ArticleDefine {
        self.spec.get_defs()
    }
}

#[allow(dead_code)]
impl ArticleSpecConfig {
    fn new(_code: i32, _role: i32, _sums: Vec<i32>) -> ArticleSpecConfig {
        ArticleSpecConfig {
            spec: ArticleSpec::new(ArticleCode::get(_code), ConceptCode::get(_role),
                                   ArticleSpec::const_to_sums_array(_sums)),
        }
    }
    fn specs_to_number_sums(_codes: Vec<ArticleCode>) -> Vec<i32> {
        return _codes.into_iter().map(|x| x.value).collect()
    }
}

pub(crate) struct ArticleProviderConfig {
    spec: ArticleSpecProvider,
    article_spec: ArticleSpecConfig,
}

#[allow(dead_code)]
impl ArticleProviderConfig {
    pub(crate) fn new(article: i32, concept: i32, sums: Vec<i32>) -> ArticleProviderConfig {
        ArticleProviderConfig {
            spec: ArticleSpecProvider::new(ArticleCode::get(article)),
            article_spec: ArticleSpecConfig::new(article, concept, sums.to_vec()),
        }
    }
    fn get_spec_config(article: &ArticleCode, concept: &ConceptCode, sums: Vec<ArticleCode>) -> ArticleProviderConfig {
        ArticleProviderConfig::new(article.get_value(), concept.get_value(), ArticleSpecConfig::specs_to_number_sums(sums))
    }
    fn get_const_config(article: i32, concept: i32, sums: Vec<i32>) -> ArticleProviderConfig {
        ArticleProviderConfig::new(article, concept, sums)
    }
}

impl IArticleSpecProvider for ArticleProviderConfig {
    fn get_code(&self) -> ArticleCode {
        self.spec.get_code()
    }

    fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> Box<dyn IArticleSpec> {
        Box::new(self.article_spec.clone())
    }
}

