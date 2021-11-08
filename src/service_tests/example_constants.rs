#[derive(Debug)]
pub(crate) enum ExampleArticleConst {
    ArticleTimeshtWorking = 80001,
    ArticlePaymentSalary = 80002,
    ArticlePaymentBonus = 80003,
    ArticlePaymentBarter = 80004,
    ArticleAllowceHoffice = 80005,
    ArticleHealthInsbase = 80006,
    ArticleSocialInsbase = 80007,
    ArticleHealthInspaym = 80008,
    ArticleSocialInspaym = 80009,
    ArticleTaxingAdvbase = 80010,
    ArticleTaxingAdvpaym = 80011,
    ArticleIncomeGross = 80012,
    ArticleIncomeNetto = 80013,
}

#[allow(dead_code)]
impl ExampleArticleConst {
    pub(crate) fn name_of_article(item: i32) -> String {
        let article: ExampleArticleConst = unsafe { std::mem::transmute(item as i32) };
        format!("{:?}", article)
    }
}

#[derive(Debug)]
pub(crate) enum ExampleConceptConst {
    ConceptTimeshtWorking = 80001,
    ConceptAmountBasis = 80002,
    ConceptAmountFixed = 80003,
    ConceptHealthInsbase = 80006,
    ConceptSocialInsbase = 80007,
    ConceptHealthInspaym = 80008,
    ConceptSocialInspaym = 80009,
    ConceptTaxingAdvbase = 80010,
    ConceptTaxingAdvpaym = 80011,
    ConceptIncomeGross = 80012,
    ConceptIncomeNetto = 80013,
}

#[allow(dead_code)]
impl ExampleConceptConst {
    pub(crate) fn name_of_concept(item: i32) -> String {
        let article: ExampleConceptConst = unsafe { std::mem::transmute(item as i32) };
        format!("{:?}", article)
    }
}

