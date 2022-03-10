use std::collections::HashMap;
use std::sync::Arc;
use legalios::service::period::IPeriod;
use crate::service_types::article_code::ArticleCode;
use crate::service_types::concept_code::ConceptCode;
use crate::service_types::version_code::VersionCode;
use crate::service_types::article_define::{ArticleDefine, IArticleDefine};
use crate::registry_constants::article_consts::ArticleConst;
use crate::registry_constants::concept_consts::ConceptConst;
use crate::registry_factories::article_config::ArticleProviderConfig;
use crate::registry_providers::article_provider::{ArticleSpec, ArticleSpecProvider, ArcArticleSpec, BoxArticleSpecProvider, IArticleSpec, IArticleSpecProvider};
use crate::service_types::article_seqs::ArticleSeqs;
use crate::service_types::article_term::ArticleTerm;

type MapArticleCode = i32;

pub(crate) struct NotFoundArticleSpec {
    spec: ArticleSpec,
}

impl IArticleDefine for NotFoundArticleSpec {
    fn get_code(&self) -> ArticleCode {
        self.spec.get_code()
    }

    fn get_seqs(&self) -> ArticleSeqs {
        self.spec.get_seqs()
    }

    fn get_role(&self) -> ConceptCode {
        self.spec.get_role()
    }
    fn get_term(&self) -> ArticleTerm {
        self.spec.get_term()
    }

}

impl IArticleSpec for NotFoundArticleSpec {
    fn get_sums(&self) -> Vec<ArticleCode> {
        self.spec.get_sums()
    }

    fn get_defs(&self) -> ArticleDefine {
        self.spec.get_defs()
    }
}

impl NotFoundArticleSpec {
    fn get(_code: ArticleCode) -> NotFoundArticleSpec {
        let concept_code = ConceptConst::ConceptNotfound as i32;
        NotFoundArticleSpec {
            spec: ArticleSpec::new(
                _code,
                ArticleSeqs::zero(),
                ConceptCode::get(concept_code), vec![]),
        }
    }
    fn new() -> NotFoundArticleSpec {
        let article_code = ArticleConst::ArticleNotfound as i32;
        NotFoundArticleSpec::get(ArticleCode::get(article_code))
    }
}

pub(crate) struct NotFoundArticleProvider {
    spec: ArticleSpecProvider,
}

impl IArticleSpecProvider for NotFoundArticleProvider {
    fn get_code(&self) -> ArticleCode {
        self.spec.get_code()
    }

    fn get_spec(&self, _period: &dyn IPeriod, _version: &VersionCode) -> Arc<dyn IArticleSpec> {
        Arc::new(NotFoundArticleSpec::get(self.spec.code))
    }
}

impl NotFoundArticleProvider {
    fn new() -> NotFoundArticleProvider {
        let article_code = ArticleConst::ArticleNotfound as i32;
        NotFoundArticleProvider {
            spec: ArticleSpecProvider {
                code: ArticleCode::get(article_code),
            },
        }
    }
}

pub(crate) struct ProviderRecord {
    article: i32,
    sequens: i16,
    concept: i32,
    sums: Vec<i32>,
}

impl ProviderRecord {
    pub fn new(_article: i32, _sequens: i16, _concept: i32, _sums: Vec<i32>) -> ProviderRecord {
        ProviderRecord {
            article: _article,
            sequens: _sequens,
            concept: _concept,
            sums: _sums.to_vec(),
        }
    }
}

pub(crate) trait IArticleSpecFactory {
    fn get_spec(&self, code: &ArticleCode, period: &dyn IPeriod, version: &VersionCode) -> ArcArticleSpec;
    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<ArcArticleSpec>;
}
pub(crate) type BoxArticleSpecFactory = Box<dyn IArticleSpecFactory>;

pub(crate) struct ArticleSpecFactory {
    not_found_provider: BoxArticleSpecProvider,
    providers: HashMap<MapArticleCode, BoxArticleSpecProvider>,
}

impl IArticleSpecFactory for ArticleSpecFactory {
    fn get_spec(&self, code: &ArticleCode, period: &dyn IPeriod, version: &VersionCode) -> ArcArticleSpec {
        let opt_provider = self.get_provider(code, &self.not_found_provider);
        match opt_provider {
            Some(provider) => provider.get_spec(period, version),
            None => Arc::new(NotFoundArticleSpec::new()),
        }
    }

    fn get_spec_list(&self, period: &dyn IPeriod, version: &VersionCode) -> Vec<ArcArticleSpec> {
        let values = self.providers.iter()
            .map(|x| x.1.get_spec(period, version)).collect::<Vec<ArcArticleSpec>>();
        values
    }
}

type ArticleBuildFunc = fn() -> Vec<BoxArticleSpecProvider>;

impl ArticleSpecFactory {
    pub(crate) fn new(build_func: ArticleBuildFunc) -> ArticleSpecFactory {
        ArticleSpecFactory {
            not_found_provider: Box::new(NotFoundArticleProvider::new()),
            providers: build_func().into_iter().map(|x| (x.get_code().value, x)).collect()
        }
    }

    fn get_provider<'a>(&'a self, code: &ArticleCode, def_provider: &'a BoxArticleSpecProvider) -> Option<&'a BoxArticleSpecProvider> {
        let map_provider = self.providers.get(&code.value);
        let val_provider = match map_provider {
            Some(provider) => provider,
            None => def_provider,
        };
        Some(val_provider)
    }

    pub(crate) fn build_providers_from_records(records: Vec<ProviderRecord>) -> Vec<BoxArticleSpecProvider> {
        let providers: Vec<BoxArticleSpecProvider> = records.into_iter()
            .map(|x| Box::new(ArticleProviderConfig::new(x.article, x.sequens, x.concept, x.sums)) as BoxArticleSpecProvider).collect();

        providers
    }
}

