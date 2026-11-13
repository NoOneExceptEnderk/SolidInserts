/// Match SolidCraft's language tags. Direct callers that omit the language keep the
/// original Italian messages; an unsupported app language uses English.
#[derive(Clone, Copy)]
pub(crate) enum Language {
    Italian,
    English,
    German,
    Spanish,
    French,
    Portuguese,
}

impl Language {
    pub(crate) fn from_tag(tag: Option<&str>) -> Self {
        match tag {
            None | Some("it") => Self::Italian,
            Some("de") => Self::German,
            Some("es") => Self::Spanish,
            Some("fr") => Self::French,
            Some("pt-BR") => Self::Portuguese,
            _ => Self::English,
        }
    }
}

// Rust checks every format string and its arguments at compile time.
macro_rules! local_format {
    ($lang:expr, [$it:literal, $en:literal] $(, $arg:expr)* $(,)?) => {
        if matches!($lang, $crate::locale::Language::Italian) {
            format!($it $(, $arg)*)
        } else {
            format!($en $(, $arg)*)
        }
    };
    ($lang:expr, [$it:literal, $en:literal, $de:literal, $es:literal, $fr:literal, $pt:literal] $(, $arg:expr)* $(,)?) => {
        match $lang {
            $crate::locale::Language::Italian => format!($it $(, $arg)*),
            $crate::locale::Language::English => format!($en $(, $arg)*),
            $crate::locale::Language::German => format!($de $(, $arg)*),
            $crate::locale::Language::Spanish => format!($es $(, $arg)*),
            $crate::locale::Language::French => format!($fr $(, $arg)*),
            $crate::locale::Language::Portuguese => format!($pt $(, $arg)*),
        }
    };
}

pub(crate) use local_format;
