pub(crate) trait ITermConstants {
    const TERM_BEG_NOTLIMIT: i8;
    const TERM_BEG_FINISHED: i8;
    const TERM_END_NOTLIMIT: i8;
    const TERM_END_FINISHED: i8;
}

pub(crate) struct TermConstants {
}

impl ITermConstants for TermConstants {
    const TERM_BEG_NOTLIMIT: i8 = 0;
    const TERM_BEG_FINISHED: i8 = 32;
    const TERM_END_NOTLIMIT: i8 = 32;
    const TERM_END_FINISHED: i8 = 0;
}

