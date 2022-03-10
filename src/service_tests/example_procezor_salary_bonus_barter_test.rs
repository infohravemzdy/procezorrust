#[cfg(test)]
mod service_example_tests {
    use std::sync::Arc;
    use legalios::service::period::{IPeriod, Period};
    use legalios::service::bundle_props::{BundleProps};
    use crate::service_types::article_code::ArticleCode;
    use crate::service_types::concept_code::ConceptCode;
    use crate::service_types::contract_code::ContractCode;
    use crate::service_types::month_code::MonthCode;
    use crate::service_types::variant_code::VariantCode;
    use crate::service_types::version_code::VersionCode;
    use crate::service_types::position_code::PositionCode;
    use crate::service_types::term_symbol::ITermSymbol;
    use crate::service_types::term_target::{ArcTermTargetList, ITermTarget};
    use crate::service::service_procezor::{IServiceProcezor};
    use crate::service_tests::example_constants::ExampleArticleConst;
    use crate::service_tests::example_constants::ExampleConceptConst;
    use crate::service_tests::example_concepts::ExampleTermTarget;
    use crate::service_tests::example_service::ExampleService;

    fn get_targets_with_salary_bonus_barter(period: &dyn IPeriod) -> ArcTermTargetList {
        let contract_code = 0i16;
        let position_code = 0i16;

        let mont_code = MonthCode::get(period.get_code());
        let contract = ContractCode::get(contract_code);
        let position = PositionCode::get(position_code);
        let variant1 = VariantCode::get(1);
        vec![
            Arc::new(ExampleTermTarget::zero_value(&mont_code, &contract, &position, &variant1,
                                                   &ArticleCode::get(ExampleArticleConst::ArticleTimeshtWorking as i32),
                                                   &ConceptCode::get(ExampleConceptConst::ConceptTimeshtWorking as i32))),
            Arc::new(ExampleTermTarget::zero_value(&mont_code, &contract, &position, &variant1,
                                                   &ArticleCode::get(ExampleArticleConst::ArticlePaymentSalary as i32),
                                                   &ConceptCode::get(ExampleConceptConst::ConceptAmountBasis as i32))),
            Arc::new(ExampleTermTarget::zero_value(&mont_code, &contract, &position, &variant1,
                                                   &ArticleCode::get(ExampleArticleConst::ArticlePaymentBonus as i32),
                                                   &ConceptCode::get(ExampleConceptConst::ConceptAmountFixed as i32))),
            Arc::new(ExampleTermTarget::zero_value(&mont_code, &contract, &position, &variant1,
                                                   &ArticleCode::get(ExampleArticleConst::ArticlePaymentBarter as i32),
                                                   &ConceptCode::get(ExampleConceptConst::ConceptAmountFixed as i32))),
        ]
    }
    #[test]
    fn example_salary_bonus_barter() {
        let example_version = 100i32;
        let test_version = VersionCode::get(example_version);
        let test_period = Period::get_with_year_month(2021, 1);

        let test_calc_arts = vec!(ArticleCode::get(ExampleArticleConst::ArticleIncomeNetto as i32));
        let mut test_service = ExampleService::new(example_version, &test_calc_arts);
        let test_ruleset = BundleProps::empty(&test_period);

        let factory_article_code = ArticleCode::get(ExampleArticleConst::ArticleTimeshtWorking as i32);
        let factory_concept_code= ConceptCode::get(ExampleConceptConst::ConceptTimeshtWorking as i32);

        let factory_article = test_service.get_article_spec(&factory_article_code, &test_period, &test_version);
        let factory_concept = test_service.get_concept_spec(&factory_concept_code, &test_period, &test_version);

        let init_service = test_service.init_with_period(&test_period);
        let rest_targets = get_targets_with_salary_bonus_barter(&test_period);
        let test_results = test_service.get_results(&test_period, &test_ruleset, &rest_targets);

        for (index, result) in test_results.iter().enumerate() {
            if result.is_ok() {
                let result_value = result.as_ref().ok().unwrap();
                let article_symbol = result_value.get_article_descr();
                let concept_symbol = result_value.get_concept_descr();
                println!("Index: {}, ART: {}, CON: {}", index, article_symbol, concept_symbol);
            }
            if result.is_err() {
                let error_value = result.as_ref().err().unwrap();
                let article_symbol = error_value.get_article_descr();
                let concept_symbol = error_value.get_concept_descr();
                println!("Index: {}, ART: {}, CON: {}, Error: {}", index, article_symbol, concept_symbol, error_value.get_message());
            }
        }

        let test_articles = [80001, 80003, 80004, 80002, 80006, 80007, 80010, 80012, 80008, 80009, 80011, 80013];
        let rest_articles: Vec<i32> = test_results.iter()
            .filter(|x| x.is_ok())
            .map(|x| x.as_ref().ok().unwrap().get_article().get_value())
            .collect();
        let errs_articles: Vec<String> = test_results.iter()
            .filter(|x| x.is_err())
            .map(|x| x.as_ref().err().unwrap().get_message())
            .collect();
        let test_articles_len = test_articles.len();
        let rest_articles_len = rest_articles.len();
        let errs_articles_len = errs_articles.len();

        let mut articles_diff = rest_articles.len() != test_articles.len();
        if rest_articles_len == test_articles_len {
            articles_diff = rest_articles.iter().zip(test_articles).filter(|&(a, b)| *a == b).count()!=rest_articles.len();
        }

        assert_eq!(test_version.get_value(), 100,
                   "Error getting period from service expected {} got {}", 100, test_version.get_value());
        assert_eq!(test_period.get_code(), 202101,
                   "Error getting article from service expected {} got {}", 202101, test_period.get_code());
        assert_eq!(factory_article.get_code().get_value(), ExampleArticleConst::ArticleTimeshtWorking as i32,
                   "Error getting article from service expected {} got {}", ExampleArticleConst::ArticleTimeshtWorking as i32, factory_article.get_code().get_value());
        assert_eq!(factory_concept.get_code().get_value(), ExampleConceptConst::ConceptTimeshtWorking as i32,
                   "Error getting concept from service expected {} got {}", ExampleConceptConst::ConceptTimeshtWorking as i32, factory_concept.get_code().get_value());
        assert_eq!(init_service, true,
                   "Error initializing service expected {} got {}", true, init_service);
        assert_eq!(rest_articles_len, test_articles_len,
                   "Error getting result from service result len don't match, expected {} got {}", test_articles_len, rest_articles_len);
        assert_eq!(errs_articles_len, 0,
                   "Error getting result from service errors len don't match, expected {} got {}", 0, errs_articles_len);
        assert_eq!(articles_diff, false,
                   "Error getting result from service result len don't match, expected {} got {}", false, articles_diff);
        assert_eq!(articles_diff, false,
                   "Error getting result from service result article code don't match,\n expected {:?};\n got: {:?}\n", test_articles, rest_articles);
    }
}