use phf::{phf_map, Map};

#[cfg(test)]
mod tests {

    #[test]
    fn test_from_code() {
        // let l: crate::LanguageCode = ZH;
        let l = crate::from_code("zh");
        print!("from_code result {:?}", l)
    }

    #[test]
    fn test_from_code_2t() {
        // let l: crate::LanguageCode = ZH;
        let l = crate::from_code_2t("zhi");
        print!("from_code result {:?}", l)
    }

    #[test]
    fn test_from_code_2b() {
        // let l: crate::LanguageCode = ZH;
        let l = crate::from_code_2b("zhi");
        print!("from_code result {:?}", l)
    }

    #[test]
    fn test_from_code_3() {
        // let l: crate::LanguageCode = ZH;
        let l = crate::from_code_3("zhi");
        print!("from_code result {:?}", l)
    }
}

#[derive(Debug, Copy, Clone)]

pub struct LanguageCode<'a> {
    ///ISO Language Name
    pub name: &'static str,
    ///639-1
    pub code: &'static str,
    ///639-2/T
    pub code_2t: &'static str,
    ///639-2/B
    pub code_2b: &'static str,
    //639-3 Macrolanguage
    pub code_3: &'static str,

    pub individual_languages: &'a [IndividualLanguages],
}

#[derive(Debug, Copy, Clone)]
pub struct IndividualLanguages {
    ///Name
    pub name: &'static str,
    ///Code
    pub code: &'static str,
}

pub fn from_code(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    CODE_MAP.get(up.as_str()).cloned()
}
pub fn from_code_2t(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    CODE_2T_MAP.get(up.as_str()).cloned()
}
pub fn from_code_2b(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    CODE_2B_MAP.get(up.as_str()).cloned()
}
pub fn from_code_3(code: &str) -> Option<LanguageCode> {
    let up = code.to_lowercase();
    CODE_3_MAP.get(up.as_str()).cloned()
}

pub const AB: LanguageCode = LanguageCode {
    name: "Abkhazian",
    code: "ab",
    code_2t: "abk",
    code_2b: "abk",
    code_3: "abk",
    individual_languages: &[],
};

pub const AA: LanguageCode = LanguageCode {
    name: "Afar",
    code: "aa",
    code_2t: "aar",
    code_2b: "aar",
    code_3: "aar",
    individual_languages: &[],
};

pub const AF: LanguageCode = LanguageCode {
    name: "Afrikaans",
    code: "af",
    code_2t: "afr",
    code_2b: "afr",
    code_3: "afr",
    individual_languages: &[],
};

pub const AK: LanguageCode = LanguageCode {
    name: "Akan",
    code: "ak",
    code_2t: "aka",
    code_2b: "aka",
    code_3: "aka",
    individual_languages: &[
        IndividualLanguages {
            name: "fat",
            code: "Fanti",
        },
        IndividualLanguages {
            name: "twi",
            code: "Twi",
        },
    ],
};

pub const SQ: LanguageCode = LanguageCode {
    name: "Albanian",
    code: "sq",
    code_2t: "sqi",
    code_2b: "alb",
    code_3: "sqi",
    individual_languages: &[
        IndividualLanguages {
            name: "aae",
            code: "Arbëreshë Albanian",
        },
        IndividualLanguages {
            name: "aat",
            code: "Arvanitika Albanian",
        },
        IndividualLanguages {
            name: "aln",
            code: "Gheg Albanian",
        },
        IndividualLanguages {
            name: "als",
            code: "Tosk Albanian",
        },
    ],
};

pub const AM: LanguageCode = LanguageCode {
    name: "Amharic",
    code: "am",
    code_2t: "amh",
    code_2b: "amh",
    code_3: "amh",
    individual_languages: &[],
};

pub const AR: LanguageCode = LanguageCode {
    name: "Arabic",
    code: "ar",
    code_2t: "ara",
    code_2b: "ara",
    code_3: "ara",
    individual_languages: &[],
};

pub const AN: LanguageCode = LanguageCode {
    name: "Aragonese",
    code: "an",
    code_2t: "arg",
    code_2b: "arg",
    code_3: "arg",
    individual_languages: &[],
};

pub const HY: LanguageCode = LanguageCode {
    name: "Armenian",
    code: "hy",
    code_2t: "hye",
    code_2b: "arm",
    code_3: "hye",
    individual_languages: &[],
};

pub const AS: LanguageCode = LanguageCode {
    name: "Assamese",
    code: "as",
    code_2t: "asm",
    code_2b: "asm",
    code_3: "asm",
    individual_languages: &[],
};

pub const AV: LanguageCode = LanguageCode {
    name: "Avaric",
    code: "av",
    code_2t: "ava",
    code_2b: "ava",
    code_3: "ava",
    individual_languages: &[],
};

pub const AE: LanguageCode = LanguageCode {
    name: "Avestan",
    code: "ae",
    code_2t: "ave",
    code_2b: "ave",
    code_3: "ave",
    individual_languages: &[],
};

pub const AY: LanguageCode = LanguageCode {
    name: "Aymara",
    code: "ay",
    code_2t: "aym",
    code_2b: "aym",
    code_3: "aym",
    individual_languages: &[],
};

pub const AZ: LanguageCode = LanguageCode {
    name: "Azerbaijani",
    code: "az",
    code_2t: "aze",
    code_2b: "aze",
    code_3: "aze",
    individual_languages: &[],
};

pub const BM: LanguageCode = LanguageCode {
    name: "Bambara",
    code: "bm",
    code_2t: "bam",
    code_2b: "bam",
    code_3: "bam",
    individual_languages: &[],
};

pub const BA: LanguageCode = LanguageCode {
    name: "Bashkir",
    code: "ba",
    code_2t: "bak",
    code_2b: "bak",
    code_3: "bak",
    individual_languages: &[],
};

pub const EU: LanguageCode = LanguageCode {
    name: "Basque",
    code: "eu",
    code_2t: "eus",
    code_2b: "baq",
    code_3: "eus",
    individual_languages: &[],
};

pub const BE: LanguageCode = LanguageCode {
    name: "Belarusian",
    code: "be",
    code_2t: "bel",
    code_2b: "bel",
    code_3: "bel",
    individual_languages: &[],
};

pub const BN: LanguageCode = LanguageCode {
    name: "Bengali",
    code: "bn",
    code_2t: "ben",
    code_2b: "ben",
    code_3: "ben",
    individual_languages: &[],
};

pub const BI: LanguageCode = LanguageCode {
    name: "Bislama",
    code: "bi",
    code_2t: "bis",
    code_2b: "bis",
    code_3: "bis",
    individual_languages: &[],
};

pub const BS: LanguageCode = LanguageCode {
    name: "Bosnian",
    code: "bs",
    code_2t: "bos",
    code_2b: "bos",
    code_3: "bos",
    individual_languages: &[],
};

pub const BR: LanguageCode = LanguageCode {
    name: "Breton",
    code: "br",
    code_2t: "bre",
    code_2b: "bre",
    code_3: "bre",
    individual_languages: &[],
};

pub const BG: LanguageCode = LanguageCode {
    name: "Bulgarian",
    code: "bg",
    code_2t: "bul",
    code_2b: "bul",
    code_3: "bul",
    individual_languages: &[],
};

pub const MY: LanguageCode = LanguageCode {
    name: "Burmese",
    code: "my",
    code_2t: "mya",
    code_2b: "bur",
    code_3: "mya",
    individual_languages: &[],
};

pub const CA: LanguageCode = LanguageCode {
    name: "Catalan, Valencian",
    code: "ca",
    code_2t: "cat",
    code_2b: "cat",
    code_3: "cat",
    individual_languages: &[],
};

pub const CH: LanguageCode = LanguageCode {
    name: "Chamorro",
    code: "ch",
    code_2t: "cha",
    code_2b: "cha",
    code_3: "cha",
    individual_languages: &[],
};

pub const CE: LanguageCode = LanguageCode {
    name: "Chechen",
    code: "ce",
    code_2t: "che",
    code_2b: "che",
    code_3: "che",
    individual_languages: &[],
};

pub const NY: LanguageCode = LanguageCode {
    name: "Chichewa, Chewa, Nyanja",
    code: "ny",
    code_2t: "nya",
    code_2b: "nya",
    code_3: "nya",
    individual_languages: &[],
};

pub const ZH: LanguageCode = LanguageCode {
    name: "Chinese",
    code: "zh",
    code_2t: "zho",
    code_2b: "chi",
    code_3: "zho",
    individual_languages: &[
        IndividualLanguages {
            name: "cdo",
            code: "Min Dong Chinese",
        },
        IndividualLanguages {
            name: "cjy",
            code: "Jinyu Chinese",
        },
        IndividualLanguages {
            name: "cnm",
            code: "Mandarin Chinese",
        },
        IndividualLanguages {
            name: "cnp",
            code: "Northern Ping Chinese",
        },
        IndividualLanguages {
            name: "cpx",
            code: "Pu-Xian Chinese",
        },
        IndividualLanguages {
            name: "csp",
            code: "Southern Ping Chinese",
        },
        IndividualLanguages {
            name: "czh",
            code: "Huizhou Chinese",
        },
        IndividualLanguages {
            name: "czo",
            code: "Min Zhong Chinese",
        },
        IndividualLanguages {
            name: "gan",
            code: "Gan Chinese",
        },
        IndividualLanguages {
            name: "hak",
            code: "Hakka Chinese",
        },
        IndividualLanguages {
            name: "han",
            code: "Xiang Chinese",
        },
        IndividualLanguages {
            name: "lzh",
            code: "Literary Chinese",
        },
        IndividualLanguages {
            name: "mnp",
            code: "Min Bei Chinese",
        },
        IndividualLanguages {
            name: "nan",
            code: "Min Nan Chinese",
        },
        IndividualLanguages {
            name: "wuu",
            code: "Wu Chinese",
        },
        IndividualLanguages {
            name: "yue",
            code: "Yue Chinese",
        },
    ],
};

pub const CU: LanguageCode = LanguageCode {
    name: "Old&nbsp;Church&nbsp;Slavonic",
    code: "cu",
    code_2t: "chu",
    code_2b: "chu",
    code_3: "chu",
    individual_languages: &[],
};

pub const CV: LanguageCode = LanguageCode {
    name: "Chuvash",
    code: "cv",
    code_2t: "chv",
    code_2b: "chv",
    code_3: "chv",
    individual_languages: &[],
};

pub const KW: LanguageCode = LanguageCode {
    name: "Cornish",
    code: "kw",
    code_2t: "cor",
    code_2b: "cor",
    code_3: "cor",
    individual_languages: &[],
};

pub const CO: LanguageCode = LanguageCode {
    name: "Corsican",
    code: "co",
    code_2t: "cos",
    code_2b: "cos",
    code_3: "cos",
    individual_languages: &[],
};

pub const CR: LanguageCode = LanguageCode {
    name: "Cree",
    code: "cr",
    code_2t: "cre",
    code_2b: "cre",
    code_3: "cre",
    individual_languages: &[],
};

pub const HR: LanguageCode = LanguageCode {
    name: "Croatian",
    code: "hr",
    code_2t: "hrv",
    code_2b: "hrv",
    code_3: "hrv",
    individual_languages: &[],
};

pub const CS: LanguageCode = LanguageCode {
    name: "Czech",
    code: "cs",
    code_2t: "ces",
    code_2b: "cze",
    code_3: "ces",
    individual_languages: &[],
};

pub const DA: LanguageCode = LanguageCode {
    name: "Danish",
    code: "da",
    code_2t: "dan",
    code_2b: "dan",
    code_3: "dan",
    individual_languages: &[],
};

pub const DV: LanguageCode = LanguageCode {
    name: "Divehi, Dhivehi, Maldivian",
    code: "dv",
    code_2t: "div",
    code_2b: "div",
    code_3: "div",
    individual_languages: &[],
};

pub const NL: LanguageCode = LanguageCode {
    name: "Dutch, Flemish",
    code: "nl",
    code_2t: "nld",
    code_2b: "dut",
    code_3: "nld",
    individual_languages: &[],
};

pub const DZ: LanguageCode = LanguageCode {
    name: "Dzongkha",
    code: "dz",
    code_2t: "dzo",
    code_2b: "dzo",
    code_3: "dzo",
    individual_languages: &[],
};

pub const EN: LanguageCode = LanguageCode {
    name: "English",
    code: "en",
    code_2t: "eng",
    code_2b: "eng",
    code_3: "eng",
    individual_languages: &[],
};

pub const EO: LanguageCode = LanguageCode {
    name: "Esperanto",
    code: "eo",
    code_2t: "epo",
    code_2b: "epo",
    code_3: "epo",
    individual_languages: &[],
};

pub const ET: LanguageCode = LanguageCode {
    name: "Estonian",
    code: "et",
    code_2t: "est",
    code_2b: "est",
    code_3: "est",
    individual_languages: &[],
};

pub const EE: LanguageCode = LanguageCode {
    name: "Ewe",
    code: "ee",
    code_2t: "ewe",
    code_2b: "ewe",
    code_3: "ewe",
    individual_languages: &[],
};

pub const FO: LanguageCode = LanguageCode {
    name: "Faroese",
    code: "fo",
    code_2t: "fao",
    code_2b: "fao",
    code_3: "fao",
    individual_languages: &[],
};

pub const FJ: LanguageCode = LanguageCode {
    name: "Fijian",
    code: "fj",
    code_2t: "fij",
    code_2b: "fij",
    code_3: "fij",
    individual_languages: &[],
};

pub const FI: LanguageCode = LanguageCode {
    name: "Finnish",
    code: "fi",
    code_2t: "fin",
    code_2b: "fin",
    code_3: "fin",
    individual_languages: &[],
};

pub const FR: LanguageCode = LanguageCode {
    name: "French",
    code: "fr",
    code_2t: "fra",
    code_2b: "fre",
    code_3: "fra",
    individual_languages: &[],
};

pub const FY: LanguageCode = LanguageCode {
    name: "Western Frisian",
    code: "fy",
    code_2t: "fry",
    code_2b: "fry",
    code_3: "fry",
    individual_languages: &[],
};

pub const FF: LanguageCode = LanguageCode {
    name: "Fulah",
    code: "ff",
    code_2t: "ful",
    code_2b: "ful",
    code_3: "ful",
    individual_languages: &[],
};

pub const GD: LanguageCode = LanguageCode {
    name: "Gaelic, Scottish Gaelic",
    code: "gd",
    code_2t: "gla",
    code_2b: "gla",
    code_3: "gla",
    individual_languages: &[],
};

pub const GL: LanguageCode = LanguageCode {
    name: "Galician",
    code: "gl",
    code_2t: "glg",
    code_2b: "glg",
    code_3: "glg",
    individual_languages: &[],
};

pub const LG: LanguageCode = LanguageCode {
    name: "Ganda",
    code: "lg",
    code_2t: "lug",
    code_2b: "lug",
    code_3: "lug",
    individual_languages: &[],
};

pub const KA: LanguageCode = LanguageCode {
    name: "Georgian",
    code: "ka",
    code_2t: "kat",
    code_2b: "geo",
    code_3: "kat",
    individual_languages: &[],
};

pub const DE: LanguageCode = LanguageCode {
    name: "German",
    code: "de",
    code_2t: "deu",
    code_2b: "ger",
    code_3: "deu",
    individual_languages: &[],
};

pub const EL: LanguageCode = LanguageCode {
    name: "Greek, Modern (1453–)",
    code: "el",
    code_2t: "ell",
    code_2b: "gre",
    code_3: "ell",
    individual_languages: &[],
};

pub const KL: LanguageCode = LanguageCode {
    name: "Kalaallisut, Greenlandic",
    code: "kl",
    code_2t: "kal",
    code_2b: "kal",
    code_3: "kal",
    individual_languages: &[],
};

pub const GN: LanguageCode = LanguageCode {
    name: "Guarani",
    code: "gn",
    code_2t: "grn",
    code_2b: "grn",
    code_3: "grn",
    individual_languages: &[],
};

pub const GU: LanguageCode = LanguageCode {
    name: "Gujarati",
    code: "gu",
    code_2t: "guj",
    code_2b: "guj",
    code_3: "guj",
    individual_languages: &[],
};

pub const HT: LanguageCode = LanguageCode {
    name: "Haitian, Haitian Creole",
    code: "ht",
    code_2t: "hat",
    code_2b: "hat",
    code_3: "hat",
    individual_languages: &[],
};

pub const HA: LanguageCode = LanguageCode {
    name: "Hausa",
    code: "ha",
    code_2t: "hau",
    code_2b: "hau",
    code_3: "hau",
    individual_languages: &[],
};

pub const HE: LanguageCode = LanguageCode {
    name: "Hebrew",
    code: "he",
    code_2t: "heb",
    code_2b: "heb",
    code_3: "heb",
    individual_languages: &[],
};

pub const HZ: LanguageCode = LanguageCode {
    name: "Herero",
    code: "hz",
    code_2t: "her",
    code_2b: "her",
    code_3: "her",
    individual_languages: &[],
};

pub const HI: LanguageCode = LanguageCode {
    name: "Hindi",
    code: "hi",
    code_2t: "hin",
    code_2b: "hin",
    code_3: "hin",
    individual_languages: &[],
};

pub const HO: LanguageCode = LanguageCode {
    name: "Hiri Motu",
    code: "ho",
    code_2t: "hmo",
    code_2b: "hmo",
    code_3: "hmo",
    individual_languages: &[],
};

pub const HU: LanguageCode = LanguageCode {
    name: "Hungarian",
    code: "hu",
    code_2t: "hun",
    code_2b: "hun",
    code_3: "hun",
    individual_languages: &[],
};

pub const IS: LanguageCode = LanguageCode {
    name: "Icelandic",
    code: "is",
    code_2t: "isl",
    code_2b: "ice",
    code_3: "isl",
    individual_languages: &[],
};

pub const IO: LanguageCode = LanguageCode {
    name: "Ido",
    code: "io",
    code_2t: "ido",
    code_2b: "ido",
    code_3: "ido",
    individual_languages: &[],
};

pub const IG: LanguageCode = LanguageCode {
    name: "Igbo",
    code: "ig",
    code_2t: "ibo",
    code_2b: "ibo",
    code_3: "ibo",
    individual_languages: &[],
};

pub const ID: LanguageCode = LanguageCode {
    name: "Indonesian",
    code: "id",
    code_2t: "ind",
    code_2b: "ind",
    code_3: "ind",
    individual_languages: &[],
};

pub const IA: LanguageCode = LanguageCode {
    name: "Interlingua (International Auxiliary Language Association)",
    code: "ia",
    code_2t: "ina",
    code_2b: "ina",
    code_3: "ina",
    individual_languages: &[],
};

pub const IE: LanguageCode = LanguageCode {
    name: "Interlingue, Occidental",
    code: "ie",
    code_2t: "ile",
    code_2b: "ile",
    code_3: "ile",
    individual_languages: &[],
};

pub const IU: LanguageCode = LanguageCode {
    name: "Inuktitut",
    code: "iu",
    code_2t: "iku",
    code_2b: "iku",
    code_3: "iku",
    individual_languages: &[],
};

pub const IK: LanguageCode = LanguageCode {
    name: "Inupiaq",
    code: "ik",
    code_2t: "ipk",
    code_2b: "ipk",
    code_3: "ipk",
    individual_languages: &[],
};

pub const GA: LanguageCode = LanguageCode {
    name: "Irish",
    code: "ga",
    code_2t: "gle",
    code_2b: "gle",
    code_3: "gle",
    individual_languages: &[],
};

pub const IT: LanguageCode = LanguageCode {
    name: "Italian",
    code: "it",
    code_2t: "ita",
    code_2b: "ita",
    code_3: "ita",
    individual_languages: &[],
};

pub const JA: LanguageCode = LanguageCode {
    name: "Japanese",
    code: "ja",
    code_2t: "jpn",
    code_2b: "jpn",
    code_3: "jpn",
    individual_languages: &[],
};

pub const JV: LanguageCode = LanguageCode {
    name: "Javanese",
    code: "jv",
    code_2t: "jav",
    code_2b: "jav",
    code_3: "jav",
    individual_languages: &[],
};

pub const KN: LanguageCode = LanguageCode {
    name: "Kannada",
    code: "kn",
    code_2t: "kan",
    code_2b: "kan",
    code_3: "kan",
    individual_languages: &[],
};

pub const KR: LanguageCode = LanguageCode {
    name: "Kanuri",
    code: "kr",
    code_2t: "kau",
    code_2b: "kau",
    code_3: "kau",
    individual_languages: &[],
};

pub const KS: LanguageCode = LanguageCode {
    name: "Kashmiri",
    code: "ks",
    code_2t: "kas",
    code_2b: "kas",
    code_3: "kas",
    individual_languages: &[],
};

pub const KK: LanguageCode = LanguageCode {
    name: "Kazakh",
    code: "kk",
    code_2t: "kaz",
    code_2b: "kaz",
    code_3: "kaz",
    individual_languages: &[],
};

pub const KM: LanguageCode = LanguageCode {
    name: "Central Khmer",
    code: "km",
    code_2t: "khm",
    code_2b: "khm",
    code_3: "khm",
    individual_languages: &[],
};

pub const KI: LanguageCode = LanguageCode {
    name: "Kikuyu, Gikuyu",
    code: "ki",
    code_2t: "kik",
    code_2b: "kik",
    code_3: "kik",
    individual_languages: &[],
};

pub const RW: LanguageCode = LanguageCode {
    name: "Kinyarwanda",
    code: "rw",
    code_2t: "kin",
    code_2b: "kin",
    code_3: "kin",
    individual_languages: &[],
};

pub const KY: LanguageCode = LanguageCode {
    name: "Kirghiz, Kyrgyz",
    code: "ky",
    code_2t: "kir",
    code_2b: "kir",
    code_3: "kir",
    individual_languages: &[],
};

pub const KV: LanguageCode = LanguageCode {
    name: "Komi",
    code: "kv",
    code_2t: "kom",
    code_2b: "kom",
    code_3: "kom",
    individual_languages: &[],
};

pub const KG: LanguageCode = LanguageCode {
    name: "Kongo",
    code: "kg",
    code_2t: "kon",
    code_2b: "kon",
    code_3: "kon",
    individual_languages: &[],
};

pub const KO: LanguageCode = LanguageCode {
    name: "Korean",
    code: "ko",
    code_2t: "kor",
    code_2b: "kor",
    code_3: "kor",
    individual_languages: &[],
};

pub const KJ: LanguageCode = LanguageCode {
    name: "Kuanyama, Kwanyama",
    code: "kj",
    code_2t: "kua",
    code_2b: "kua",
    code_3: "kua",
    individual_languages: &[],
};

pub const KU: LanguageCode = LanguageCode {
    name: "Kurdish",
    code: "ku",
    code_2t: "kur",
    code_2b: "kur",
    code_3: "kur",
    individual_languages: &[],
};

pub const LO: LanguageCode = LanguageCode {
    name: "Lao",
    code: "lo",
    code_2t: "lao",
    code_2b: "lao",
    code_3: "lao",
    individual_languages: &[],
};

pub const LA: LanguageCode = LanguageCode {
    name: "Latin",
    code: "la",
    code_2t: "lat",
    code_2b: "lat",
    code_3: "lat",
    individual_languages: &[],
};

pub const LV: LanguageCode = LanguageCode {
    name: "Latvian",
    code: "lv",
    code_2t: "lav",
    code_2b: "lav",
    code_3: "lav",
    individual_languages: &[],
};

pub const LI: LanguageCode = LanguageCode {
    name: "Limburgan, Limburger, Limburgish",
    code: "li",
    code_2t: "lim",
    code_2b: "lim",
    code_3: "lim",
    individual_languages: &[],
};

pub const LN: LanguageCode = LanguageCode {
    name: "Lingala",
    code: "ln",
    code_2t: "lin",
    code_2b: "lin",
    code_3: "lin",
    individual_languages: &[],
};

pub const LT: LanguageCode = LanguageCode {
    name: "Lithuanian",
    code: "lt",
    code_2t: "lit",
    code_2b: "lit",
    code_3: "lit",
    individual_languages: &[],
};

pub const LU: LanguageCode = LanguageCode {
    name: "Luba-Katanga",
    code: "lu",
    code_2t: "lub",
    code_2b: "lub",
    code_3: "lub",
    individual_languages: &[],
};

pub const LB: LanguageCode = LanguageCode {
    name: "Luxembourgish, Letzeburgesch",
    code: "lb",
    code_2t: "ltz",
    code_2b: "ltz",
    code_3: "ltz",
    individual_languages: &[],
};

pub const MK: LanguageCode = LanguageCode {
    name: "Macedonian",
    code: "mk",
    code_2t: "mkd",
    code_2b: "mac",
    code_3: "mkd",
    individual_languages: &[],
};

pub const MG: LanguageCode = LanguageCode {
    name: "Malagasy",
    code: "mg",
    code_2t: "mlg",
    code_2b: "mlg",
    code_3: "mlg",
    individual_languages: &[],
};

pub const MS: LanguageCode = LanguageCode {
    name: "Malay",
    code: "ms",
    code_2t: "msa",
    code_2b: "may",
    code_3: "msa",
    individual_languages: &[],
};

pub const ML: LanguageCode = LanguageCode {
    name: "Malayalam",
    code: "ml",
    code_2t: "mal",
    code_2b: "mal",
    code_3: "mal",
    individual_languages: &[],
};

pub const MT: LanguageCode = LanguageCode {
    name: "Maltese",
    code: "mt",
    code_2t: "mlt",
    code_2b: "mlt",
    code_3: "mlt",
    individual_languages: &[],
};

pub const GV: LanguageCode = LanguageCode {
    name: "Manx",
    code: "gv",
    code_2t: "glv",
    code_2b: "glv",
    code_3: "glv",
    individual_languages: &[],
};

pub const MI: LanguageCode = LanguageCode {
    name: "Maori",
    code: "mi",
    code_2t: "mri",
    code_2b: "mao",
    code_3: "mri",
    individual_languages: &[],
};

pub const MR: LanguageCode = LanguageCode {
    name: "Marathi",
    code: "mr",
    code_2t: "mar",
    code_2b: "mar",
    code_3: "mar",
    individual_languages: &[],
};

pub const MH: LanguageCode = LanguageCode {
    name: "Marshallese",
    code: "mh",
    code_2t: "mah",
    code_2b: "mah",
    code_3: "mah",
    individual_languages: &[],
};

pub const MN: LanguageCode = LanguageCode {
    name: "Mongolian",
    code: "mn",
    code_2t: "mon",
    code_2b: "mon",
    code_3: "mon",
    individual_languages: &[],
};

pub const NA: LanguageCode = LanguageCode {
    name: "Nauru",
    code: "na",
    code_2t: "nau",
    code_2b: "nau",
    code_3: "nau",
    individual_languages: &[],
};

pub const NV: LanguageCode = LanguageCode {
    name: "Navajo, Navaho",
    code: "nv",
    code_2t: "nav",
    code_2b: "nav",
    code_3: "nav",
    individual_languages: &[],
};

pub const ND: LanguageCode = LanguageCode {
    name: "North Ndebele",
    code: "nd",
    code_2t: "nde",
    code_2b: "nde",
    code_3: "nde",
    individual_languages: &[],
};

pub const NR: LanguageCode = LanguageCode {
    name: "South Ndebele",
    code: "nr",
    code_2t: "nbl",
    code_2b: "nbl",
    code_3: "nbl",
    individual_languages: &[],
};

pub const NG: LanguageCode = LanguageCode {
    name: "Ndonga",
    code: "ng",
    code_2t: "ndo",
    code_2b: "ndo",
    code_3: "ndo",
    individual_languages: &[],
};

pub const NE: LanguageCode = LanguageCode {
    name: "Nepali",
    code: "ne",
    code_2t: "nep",
    code_2b: "nep",
    code_3: "nep",
    individual_languages: &[],
};

pub const NO: LanguageCode = LanguageCode {
    name: "Norwegian",
    code: "no",
    code_2t: "nor",
    code_2b: "nor",
    code_3: "nor",
    individual_languages: &[],
};

pub const NB: LanguageCode = LanguageCode {
    name: "Norwegian Bokmål",
    code: "nb",
    code_2t: "nob",
    code_2b: "nob",
    code_3: "nob",
    individual_languages: &[],
};

pub const NN: LanguageCode = LanguageCode {
    name: "Norwegian Nynorsk",
    code: "nn",
    code_2t: "nno",
    code_2b: "nno",
    code_3: "nno",
    individual_languages: &[],
};

pub const II: LanguageCode = LanguageCode {
    name: "Sichuan Yi, Nuosu",
    code: "ii",
    code_2t: "iii",
    code_2b: "iii",
    code_3: "iii",
    individual_languages: &[],
};

pub const OC: LanguageCode = LanguageCode {
    name: "Occitan",
    code: "oc",
    code_2t: "oci",
    code_2b: "oci",
    code_3: "oci",
    individual_languages: &[],
};

pub const OJ: LanguageCode = LanguageCode {
    name: "Ojibwa",
    code: "oj",
    code_2t: "oji",
    code_2b: "oji",
    code_3: "oji",
    individual_languages: &[],
};

pub const OR: LanguageCode = LanguageCode {
    name: "Oriya",
    code: "or",
    code_2t: "ori",
    code_2b: "ori",
    code_3: "ori",
    individual_languages: &[],
};

pub const OM: LanguageCode = LanguageCode {
    name: "Oromo",
    code: "om",
    code_2t: "orm",
    code_2b: "orm",
    code_3: "orm",
    individual_languages: &[],
};

pub const OS: LanguageCode = LanguageCode {
    name: "Ossetian, Ossetic",
    code: "os",
    code_2t: "oss",
    code_2b: "oss",
    code_3: "oss",
    individual_languages: &[],
};

pub const PI: LanguageCode = LanguageCode {
    name: "Pali",
    code: "pi",
    code_2t: "pli",
    code_2b: "pli",
    code_3: "pli",
    individual_languages: &[],
};

pub const PS: LanguageCode = LanguageCode {
    name: "Pashto, Pushto",
    code: "ps",
    code_2t: "pus",
    code_2b: "pus",
    code_3: "pus",
    individual_languages: &[],
};

pub const FA: LanguageCode = LanguageCode {
    name: "Persian",
    code: "fa",
    code_2t: "fas",
    code_2b: "per",
    code_3: "fas",
    individual_languages: &[],
};

pub const PL: LanguageCode = LanguageCode {
    name: "Polish",
    code: "pl",
    code_2t: "pol",
    code_2b: "pol",
    code_3: "pol",
    individual_languages: &[],
};

pub const PT: LanguageCode = LanguageCode {
    name: "Portuguese",
    code: "pt",
    code_2t: "por",
    code_2b: "por",
    code_3: "por",
    individual_languages: &[],
};

pub const PA: LanguageCode = LanguageCode {
    name: "Punjabi, Panjabi",
    code: "pa",
    code_2t: "pan",
    code_2b: "pan",
    code_3: "pan",
    individual_languages: &[],
};

pub const QU: LanguageCode = LanguageCode {
    name: "Quechua",
    code: "qu",
    code_2t: "que",
    code_2b: "que",
    code_3: "que",
    individual_languages: &[],
};

pub const RO: LanguageCode = LanguageCode {
    name: "Moldavian, Moldovan",
    code: "ro",
    code_2t: "ron",
    code_2b: "rum",
    code_3: "ron",
    individual_languages: &[],
};

pub const RM: LanguageCode = LanguageCode {
    name: "Romansh",
    code: "rm",
    code_2t: "roh",
    code_2b: "roh",
    code_3: "roh",
    individual_languages: &[],
};

pub const RN: LanguageCode = LanguageCode {
    name: "Rundi",
    code: "rn",
    code_2t: "run",
    code_2b: "run",
    code_3: "run",
    individual_languages: &[],
};

pub const RU: LanguageCode = LanguageCode {
    name: "Russian",
    code: "ru",
    code_2t: "rus",
    code_2b: "rus",
    code_3: "rus",
    individual_languages: &[],
};

pub const SE: LanguageCode = LanguageCode {
    name: "Northern Sami",
    code: "se",
    code_2t: "sme",
    code_2b: "sme",
    code_3: "sme",
    individual_languages: &[],
};

pub const SM: LanguageCode = LanguageCode {
    name: "Samoan",
    code: "sm",
    code_2t: "smo",
    code_2b: "smo",
    code_3: "smo",
    individual_languages: &[],
};

pub const SG: LanguageCode = LanguageCode {
    name: "Sango",
    code: "sg",
    code_2t: "sag",
    code_2b: "sag",
    code_3: "sag",
    individual_languages: &[],
};

pub const SA: LanguageCode = LanguageCode {
    name: "Sanskrit",
    code: "sa",
    code_2t: "san",
    code_2b: "san",
    code_3: "san",
    individual_languages: &[],
};

pub const SC: LanguageCode = LanguageCode {
    name: "Sardinian",
    code: "sc",
    code_2t: "srd",
    code_2b: "srd",
    code_3: "srd",
    individual_languages: &[],
};

pub const SR: LanguageCode = LanguageCode {
    name: "Serbian",
    code: "sr",
    code_2t: "srp",
    code_2b: "srp",
    code_3: "srp",
    individual_languages: &[],
};

pub const SN: LanguageCode = LanguageCode {
    name: "Shona",
    code: "sn",
    code_2t: "sna",
    code_2b: "sna",
    code_3: "sna",
    individual_languages: &[],
};

pub const SD: LanguageCode = LanguageCode {
    name: "Sindhi",
    code: "sd",
    code_2t: "snd",
    code_2b: "snd",
    code_3: "snd",
    individual_languages: &[],
};

pub const SI: LanguageCode = LanguageCode {
    name: "Sinhala, Sinhalese",
    code: "si",
    code_2t: "sin",
    code_2b: "sin",
    code_3: "sin",
    individual_languages: &[],
};

pub const SK: LanguageCode = LanguageCode {
    name: "Slovak",
    code: "sk",
    code_2t: "slk",
    code_2b: "slo",
    code_3: "slk",
    individual_languages: &[],
};

pub const SL: LanguageCode = LanguageCode {
    name: "Slovenian",
    code: "sl",
    code_2t: "slv",
    code_2b: "slv",
    code_3: "slv",
    individual_languages: &[],
};

pub const SO: LanguageCode = LanguageCode {
    name: "Somali",
    code: "so",
    code_2t: "som",
    code_2b: "som",
    code_3: "som",
    individual_languages: &[],
};

pub const ST: LanguageCode = LanguageCode {
    name: "Southern Sotho",
    code: "st",
    code_2t: "sot",
    code_2b: "sot",
    code_3: "sot",
    individual_languages: &[],
};

pub const ES: LanguageCode = LanguageCode {
    name: "Spanish, Castilian",
    code: "es",
    code_2t: "spa",
    code_2b: "spa",
    code_3: "spa",
    individual_languages: &[],
};

pub const SU: LanguageCode = LanguageCode {
    name: "Sundanese",
    code: "su",
    code_2t: "sun",
    code_2b: "sun",
    code_3: "sun",
    individual_languages: &[],
};

pub const SW: LanguageCode = LanguageCode {
    name: "Swahili",
    code: "sw",
    code_2t: "swa",
    code_2b: "swa",
    code_3: "swa",
    individual_languages: &[],
};

pub const SS: LanguageCode = LanguageCode {
    name: "Swati",
    code: "ss",
    code_2t: "ssw",
    code_2b: "ssw",
    code_3: "ssw",
    individual_languages: &[],
};

pub const SV: LanguageCode = LanguageCode {
    name: "Swedish",
    code: "sv",
    code_2t: "swe",
    code_2b: "swe",
    code_3: "swe",
    individual_languages: &[],
};

pub const TL: LanguageCode = LanguageCode {
    name: "Tagalog",
    code: "tl",
    code_2t: "tgl",
    code_2b: "tgl",
    code_3: "tgl",
    individual_languages: &[],
};

pub const TY: LanguageCode = LanguageCode {
    name: "Tahitian",
    code: "ty",
    code_2t: "tah",
    code_2b: "tah",
    code_3: "tah",
    individual_languages: &[],
};

pub const TG: LanguageCode = LanguageCode {
    name: "Tajik",
    code: "tg",
    code_2t: "tgk",
    code_2b: "tgk",
    code_3: "tgk",
    individual_languages: &[],
};

pub const TA: LanguageCode = LanguageCode {
    name: "Tamil",
    code: "ta",
    code_2t: "tam",
    code_2b: "tam",
    code_3: "tam",
    individual_languages: &[],
};

pub const TT: LanguageCode = LanguageCode {
    name: "Tatar",
    code: "tt",
    code_2t: "tat",
    code_2b: "tat",
    code_3: "tat",
    individual_languages: &[],
};

pub const TE: LanguageCode = LanguageCode {
    name: "Telugu",
    code: "te",
    code_2t: "tel",
    code_2b: "tel",
    code_3: "tel",
    individual_languages: &[],
};

pub const TH: LanguageCode = LanguageCode {
    name: "Thai",
    code: "th",
    code_2t: "tha",
    code_2b: "tha",
    code_3: "tha",
    individual_languages: &[],
};

pub const BO: LanguageCode = LanguageCode {
    name: "Tibetan",
    code: "bo",
    code_2t: "bod",
    code_2b: "tib",
    code_3: "bod",
    individual_languages: &[],
};

pub const TI: LanguageCode = LanguageCode {
    name: "Tigrinya",
    code: "ti",
    code_2t: "tir",
    code_2b: "tir",
    code_3: "tir",
    individual_languages: &[],
};

pub const TO: LanguageCode = LanguageCode {
    name: "Tonga (Tonga Islands)",
    code: "to",
    code_2t: "ton",
    code_2b: "ton",
    code_3: "ton",
    individual_languages: &[],
};

pub const TS: LanguageCode = LanguageCode {
    name: "Tsonga",
    code: "ts",
    code_2t: "tso",
    code_2b: "tso",
    code_3: "tso",
    individual_languages: &[],
};

pub const TN: LanguageCode = LanguageCode {
    name: "Tswana",
    code: "tn",
    code_2t: "tsn",
    code_2b: "tsn",
    code_3: "tsn",
    individual_languages: &[],
};

pub const TR: LanguageCode = LanguageCode {
    name: "Turkish",
    code: "tr",
    code_2t: "tur",
    code_2b: "tur",
    code_3: "tur",
    individual_languages: &[],
};

pub const TK: LanguageCode = LanguageCode {
    name: "Turkmen",
    code: "tk",
    code_2t: "tuk",
    code_2b: "tuk",
    code_3: "tuk",
    individual_languages: &[],
};

pub const TW: LanguageCode = LanguageCode {
    name: "Twi",
    code: "tw",
    code_2t: "twi",
    code_2b: "twi",
    code_3: "twi",
    individual_languages: &[],
};

pub const UG: LanguageCode = LanguageCode {
    name: "Uighur, Uyghur",
    code: "ug",
    code_2t: "uig",
    code_2b: "uig",
    code_3: "uig",
    individual_languages: &[],
};

pub const UK: LanguageCode = LanguageCode {
    name: "Ukrainian",
    code: "uk",
    code_2t: "ukr",
    code_2b: "ukr",
    code_3: "ukr",
    individual_languages: &[],
};

pub const UR: LanguageCode = LanguageCode {
    name: "Urdu",
    code: "ur",
    code_2t: "urd",
    code_2b: "urd",
    code_3: "urd",
    individual_languages: &[],
};

pub const UZ: LanguageCode = LanguageCode {
    name: "Uzbek",
    code: "uz",
    code_2t: "uzb",
    code_2b: "uzb",
    code_3: "uzb",
    individual_languages: &[],
};

pub const VE: LanguageCode = LanguageCode {
    name: "Venda",
    code: "ve",
    code_2t: "ven",
    code_2b: "ven",
    code_3: "ven",
    individual_languages: &[],
};

pub const VI: LanguageCode = LanguageCode {
    name: "Vietnamese",
    code: "vi",
    code_2t: "vie",
    code_2b: "vie",
    code_3: "vie",
    individual_languages: &[],
};

pub const VO: LanguageCode = LanguageCode {
    name: "Volapük",
    code: "vo",
    code_2t: "vol",
    code_2b: "vol",
    code_3: "vol",
    individual_languages: &[],
};

pub const WA: LanguageCode = LanguageCode {
    name: "Walloon",
    code: "wa",
    code_2t: "wln",
    code_2b: "wln",
    code_3: "wln",
    individual_languages: &[],
};

pub const CY: LanguageCode = LanguageCode {
    name: "Welsh",
    code: "cy",
    code_2t: "cym",
    code_2b: "wel",
    code_3: "cym",
    individual_languages: &[],
};

pub const WO: LanguageCode = LanguageCode {
    name: "Wolof",
    code: "wo",
    code_2t: "wol",
    code_2b: "wol",
    code_3: "wol",
    individual_languages: &[],
};

pub const XH: LanguageCode = LanguageCode {
    name: "Xhosa",
    code: "xh",
    code_2t: "xho",
    code_2b: "xho",
    code_3: "xho",
    individual_languages: &[],
};

pub const YI: LanguageCode = LanguageCode {
    name: "Yiddish",
    code: "yi",
    code_2t: "yid",
    code_2b: "yid",
    code_3: "yid",
    individual_languages: &[],
};

pub const YO: LanguageCode = LanguageCode {
    name: "Yoruba",
    code: "yo",
    code_2t: "yor",
    code_2b: "yor",
    code_3: "yor",
    individual_languages: &[],
};

pub const ZA: LanguageCode = LanguageCode {
    name: "Zhuang, Chuang",
    code: "za",
    code_2t: "zha",
    code_2b: "zha",
    code_3: "zha",
    individual_languages: &[],
};

pub const ZU: LanguageCode = LanguageCode {
    name: "Zulu",
    code: "zu",
    code_2t: "zul",
    code_2b: "zul",
    code_3: "zul",
    individual_languages: &[],
};

pub const CODE_MAP: Map<&str, LanguageCode> = phf_map! {
"ab" => AB,
"aa" => AA,
"af" => AF,
"ak" => AK,
"sq" => SQ,
"am" => AM,
"ar" => AR,
"an" => AN,
"hy" => HY,
"as" => AS,
"av" => AV,
"ae" => AE,
"ay" => AY,
"az" => AZ,
"bm" => BM,
"ba" => BA,
"eu" => EU,
"be" => BE,
"bn" => BN,
"bi" => BI,
"bs" => BS,
"br" => BR,
"bg" => BG,
"my" => MY,
"ca" => CA,
"ch" => CH,
"ce" => CE,
"ny" => NY,
"zh" => ZH,
"cu" => CU,
"cv" => CV,
"kw" => KW,
"co" => CO,
"cr" => CR,
"hr" => HR,
"cs" => CS,
"da" => DA,
"dv" => DV,
"nl" => NL,
"dz" => DZ,
"en" => EN,
"eo" => EO,
"et" => ET,
"ee" => EE,
"fo" => FO,
"fj" => FJ,
"fi" => FI,
"fr" => FR,
"fy" => FY,
"ff" => FF,
"gd" => GD,
"gl" => GL,
"lg" => LG,
"ka" => KA,
"de" => DE,
"el" => EL,
"kl" => KL,
"gn" => GN,
"gu" => GU,
"ht" => HT,
"ha" => HA,
"he" => HE,
"hz" => HZ,
"hi" => HI,
"ho" => HO,
"hu" => HU,
"is" => IS,
"io" => IO,
"ig" => IG,
"id" => ID,
"ia" => IA,
"ie" => IE,
"iu" => IU,
"ik" => IK,
"ga" => GA,
"it" => IT,
"ja" => JA,
"jv" => JV,
"kn" => KN,
"kr" => KR,
"ks" => KS,
"kk" => KK,
"km" => KM,
"ki" => KI,
"rw" => RW,
"ky" => KY,
"kv" => KV,
"kg" => KG,
"ko" => KO,
"kj" => KJ,
"ku" => KU,
"lo" => LO,
"la" => LA,
"lv" => LV,
"li" => LI,
"ln" => LN,
"lt" => LT,
"lu" => LU,
"lb" => LB,
"mk" => MK,
"mg" => MG,
"ms" => MS,
"ml" => ML,
"mt" => MT,
"gv" => GV,
"mi" => MI,
"mr" => MR,
"mh" => MH,
"mn" => MN,
"na" => NA,
"nv" => NV,
"nd" => ND,
"nr" => NR,
"ng" => NG,
"ne" => NE,
"no" => NO,
"nb" => NB,
"nn" => NN,
"ii" => II,
"oc" => OC,
"oj" => OJ,
"or" => OR,
"om" => OM,
"os" => OS,
"pi" => PI,
"ps" => PS,
"fa" => FA,
"pl" => PL,
"pt" => PT,
"pa" => PA,
"qu" => QU,
"ro" => RO,
"rm" => RM,
"rn" => RN,
"ru" => RU,
"se" => SE,
"sm" => SM,
"sg" => SG,
"sa" => SA,
"sc" => SC,
"sr" => SR,
"sn" => SN,
"sd" => SD,
"si" => SI,
"sk" => SK,
"sl" => SL,
"so" => SO,
"st" => ST,
"es" => ES,
"su" => SU,
"sw" => SW,
"ss" => SS,
"sv" => SV,
"tl" => TL,
"ty" => TY,
"tg" => TG,
"ta" => TA,
"tt" => TT,
"te" => TE,
"th" => TH,
"bo" => BO,
"ti" => TI,
"to" => TO,
"ts" => TS,
"tn" => TN,
"tr" => TR,
"tk" => TK,
"tw" => TW,
"ug" => UG,
"uk" => UK,
"ur" => UR,
"uz" => UZ,
"ve" => VE,
"vi" => VI,
"vo" => VO,
"wa" => WA,
"cy" => CY,
"wo" => WO,
"xh" => XH,
"yi" => YI,
"yo" => YO,
"za" => ZA,
"zu" => ZU,

};

pub const CODE_2T_MAP: Map<&str, LanguageCode> = phf_map! {
"abk" => AB,
"aar" => AA,
"afr" => AF,
"aka" => AK,
"sqi" => SQ,
"amh" => AM,
"ara" => AR,
"arg" => AN,
"hye" => HY,
"asm" => AS,
"ava" => AV,
"ave" => AE,
"aym" => AY,
"aze" => AZ,
"bam" => BM,
"bak" => BA,
"eus" => EU,
"bel" => BE,
"ben" => BN,
"bis" => BI,
"bos" => BS,
"bre" => BR,
"bul" => BG,
"mya" => MY,
"cat" => CA,
"cha" => CH,
"che" => CE,
"nya" => NY,
"zho" => ZH,
"chu" => CU,
"chv" => CV,
"cor" => KW,
"cos" => CO,
"cre" => CR,
"hrv" => HR,
"ces" => CS,
"dan" => DA,
"div" => DV,
"nld" => NL,
"dzo" => DZ,
"eng" => EN,
"epo" => EO,
"est" => ET,
"ewe" => EE,
"fao" => FO,
"fij" => FJ,
"fin" => FI,
"fra" => FR,
"fry" => FY,
"ful" => FF,
"gla" => GD,
"glg" => GL,
"lug" => LG,
"kat" => KA,
"deu" => DE,
"ell" => EL,
"kal" => KL,
"grn" => GN,
"guj" => GU,
"hat" => HT,
"hau" => HA,
"heb" => HE,
"her" => HZ,
"hin" => HI,
"hmo" => HO,
"hun" => HU,
"isl" => IS,
"ido" => IO,
"ibo" => IG,
"ind" => ID,
"ina" => IA,
"ile" => IE,
"iku" => IU,
"ipk" => IK,
"gle" => GA,
"ita" => IT,
"jpn" => JA,
"jav" => JV,
"kan" => KN,
"kau" => KR,
"kas" => KS,
"kaz" => KK,
"khm" => KM,
"kik" => KI,
"kin" => RW,
"kir" => KY,
"kom" => KV,
"kon" => KG,
"kor" => KO,
"kua" => KJ,
"kur" => KU,
"lao" => LO,
"lat" => LA,
"lav" => LV,
"lim" => LI,
"lin" => LN,
"lit" => LT,
"lub" => LU,
"ltz" => LB,
"mkd" => MK,
"mlg" => MG,
"msa" => MS,
"mal" => ML,
"mlt" => MT,
"glv" => GV,
"mri" => MI,
"mar" => MR,
"mah" => MH,
"mon" => MN,
"nau" => NA,
"nav" => NV,
"nde" => ND,
"nbl" => NR,
"ndo" => NG,
"nep" => NE,
"nor" => NO,
"nob" => NB,
"nno" => NN,
"iii" => II,
"oci" => OC,
"oji" => OJ,
"ori" => OR,
"orm" => OM,
"oss" => OS,
"pli" => PI,
"pus" => PS,
"fas" => FA,
"pol" => PL,
"por" => PT,
"pan" => PA,
"que" => QU,
"ron" => RO,
"roh" => RM,
"run" => RN,
"rus" => RU,
"sme" => SE,
"smo" => SM,
"sag" => SG,
"san" => SA,
"srd" => SC,
"srp" => SR,
"sna" => SN,
"snd" => SD,
"sin" => SI,
"slk" => SK,
"slv" => SL,
"som" => SO,
"sot" => ST,
"spa" => ES,
"sun" => SU,
"swa" => SW,
"ssw" => SS,
"swe" => SV,
"tgl" => TL,
"tah" => TY,
"tgk" => TG,
"tam" => TA,
"tat" => TT,
"tel" => TE,
"tha" => TH,
"bod" => BO,
"tir" => TI,
"ton" => TO,
"tso" => TS,
"tsn" => TN,
"tur" => TR,
"tuk" => TK,
"twi" => TW,
"uig" => UG,
"ukr" => UK,
"urd" => UR,
"uzb" => UZ,
"ven" => VE,
"vie" => VI,
"vol" => VO,
"wln" => WA,
"cym" => CY,
"wol" => WO,
"xho" => XH,
"yid" => YI,
"yor" => YO,
"zha" => ZA,
"zul" => ZU,

};

pub const CODE_2B_MAP: Map<&str, LanguageCode> = phf_map! {
"abk" => AB,
"aar" => AA,
"afr" => AF,
"aka" => AK,
"alb" => SQ,
"amh" => AM,
"ara" => AR,
"arg" => AN,
"arm" => HY,
"asm" => AS,
"ava" => AV,
"ave" => AE,
"aym" => AY,
"aze" => AZ,
"bam" => BM,
"bak" => BA,
"baq" => EU,
"bel" => BE,
"ben" => BN,
"bis" => BI,
"bos" => BS,
"bre" => BR,
"bul" => BG,
"bur" => MY,
"cat" => CA,
"cha" => CH,
"che" => CE,
"nya" => NY,
"chi" => ZH,
"chu" => CU,
"chv" => CV,
"cor" => KW,
"cos" => CO,
"cre" => CR,
"hrv" => HR,
"cze" => CS,
"dan" => DA,
"div" => DV,
"dut" => NL,
"dzo" => DZ,
"eng" => EN,
"epo" => EO,
"est" => ET,
"ewe" => EE,
"fao" => FO,
"fij" => FJ,
"fin" => FI,
"fre" => FR,
"fry" => FY,
"ful" => FF,
"gla" => GD,
"glg" => GL,
"lug" => LG,
"geo" => KA,
"ger" => DE,
"gre" => EL,
"kal" => KL,
"grn" => GN,
"guj" => GU,
"hat" => HT,
"hau" => HA,
"heb" => HE,
"her" => HZ,
"hin" => HI,
"hmo" => HO,
"hun" => HU,
"ice" => IS,
"ido" => IO,
"ibo" => IG,
"ind" => ID,
"ina" => IA,
"ile" => IE,
"iku" => IU,
"ipk" => IK,
"gle" => GA,
"ita" => IT,
"jpn" => JA,
"jav" => JV,
"kan" => KN,
"kau" => KR,
"kas" => KS,
"kaz" => KK,
"khm" => KM,
"kik" => KI,
"kin" => RW,
"kir" => KY,
"kom" => KV,
"kon" => KG,
"kor" => KO,
"kua" => KJ,
"kur" => KU,
"lao" => LO,
"lat" => LA,
"lav" => LV,
"lim" => LI,
"lin" => LN,
"lit" => LT,
"lub" => LU,
"ltz" => LB,
"mac" => MK,
"mlg" => MG,
"may" => MS,
"mal" => ML,
"mlt" => MT,
"glv" => GV,
"mao" => MI,
"mar" => MR,
"mah" => MH,
"mon" => MN,
"nau" => NA,
"nav" => NV,
"nde" => ND,
"nbl" => NR,
"ndo" => NG,
"nep" => NE,
"nor" => NO,
"nob" => NB,
"nno" => NN,
"iii" => II,
"oci" => OC,
"oji" => OJ,
"ori" => OR,
"orm" => OM,
"oss" => OS,
"pli" => PI,
"pus" => PS,
"per" => FA,
"pol" => PL,
"por" => PT,
"pan" => PA,
"que" => QU,
"rum" => RO,
"roh" => RM,
"run" => RN,
"rus" => RU,
"sme" => SE,
"smo" => SM,
"sag" => SG,
"san" => SA,
"srd" => SC,
"srp" => SR,
"sna" => SN,
"snd" => SD,
"sin" => SI,
"slo" => SK,
"slv" => SL,
"som" => SO,
"sot" => ST,
"spa" => ES,
"sun" => SU,
"swa" => SW,
"ssw" => SS,
"swe" => SV,
"tgl" => TL,
"tah" => TY,
"tgk" => TG,
"tam" => TA,
"tat" => TT,
"tel" => TE,
"tha" => TH,
"tib" => BO,
"tir" => TI,
"ton" => TO,
"tso" => TS,
"tsn" => TN,
"tur" => TR,
"tuk" => TK,
"twi" => TW,
"uig" => UG,
"ukr" => UK,
"urd" => UR,
"uzb" => UZ,
"ven" => VE,
"vie" => VI,
"vol" => VO,
"wln" => WA,
"wel" => CY,
"wol" => WO,
"xho" => XH,
"yid" => YI,
"yor" => YO,
"zha" => ZA,
"zul" => ZU,

};

pub const CODE_3_MAP: Map<&str, LanguageCode> = phf_map! {
"abk" => AB,
"aar" => AA,
"afr" => AF,
"aka" => AK,
"sqi" => SQ,
"amh" => AM,
"ara" => AR,
"arg" => AN,
"hye" => HY,
"asm" => AS,
"ava" => AV,
"ave" => AE,
"aym" => AY,
"aze" => AZ,
"bam" => BM,
"bak" => BA,
"eus" => EU,
"bel" => BE,
"ben" => BN,
"bis" => BI,
"bos" => BS,
"bre" => BR,
"bul" => BG,
"mya" => MY,
"cat" => CA,
"cha" => CH,
"che" => CE,
"nya" => NY,
"zho" => ZH,
"chu" => CU,
"chv" => CV,
"cor" => KW,
"cos" => CO,
"cre" => CR,
"hrv" => HR,
"ces" => CS,
"dan" => DA,
"div" => DV,
"nld" => NL,
"dzo" => DZ,
"eng" => EN,
"epo" => EO,
"est" => ET,
"ewe" => EE,
"fao" => FO,
"fij" => FJ,
"fin" => FI,
"fra" => FR,
"fry" => FY,
"ful" => FF,
"gla" => GD,
"glg" => GL,
"lug" => LG,
"kat" => KA,
"deu" => DE,
"ell" => EL,
"kal" => KL,
"grn" => GN,
"guj" => GU,
"hat" => HT,
"hau" => HA,
"heb" => HE,
"her" => HZ,
"hin" => HI,
"hmo" => HO,
"hun" => HU,
"isl" => IS,
"ido" => IO,
"ibo" => IG,
"ind" => ID,
"ina" => IA,
"ile" => IE,
"iku" => IU,
"ipk" => IK,
"gle" => GA,
"ita" => IT,
"jpn" => JA,
"jav" => JV,
"kan" => KN,
"kau" => KR,
"kas" => KS,
"kaz" => KK,
"khm" => KM,
"kik" => KI,
"kin" => RW,
"kir" => KY,
"kom" => KV,
"kon" => KG,
"kor" => KO,
"kua" => KJ,
"kur" => KU,
"lao" => LO,
"lat" => LA,
"lav" => LV,
"lim" => LI,
"lin" => LN,
"lit" => LT,
"lub" => LU,
"ltz" => LB,
"mkd" => MK,
"mlg" => MG,
"msa" => MS,
"mal" => ML,
"mlt" => MT,
"glv" => GV,
"mri" => MI,
"mar" => MR,
"mah" => MH,
"mon" => MN,
"nau" => NA,
"nav" => NV,
"nde" => ND,
"nbl" => NR,
"ndo" => NG,
"nep" => NE,
"nor" => NO,
"nob" => NB,
"nno" => NN,
"iii" => II,
"oci" => OC,
"oji" => OJ,
"ori" => OR,
"orm" => OM,
"oss" => OS,
"pli" => PI,
"pus" => PS,
"fas" => FA,
"pol" => PL,
"por" => PT,
"pan" => PA,
"que" => QU,
"ron" => RO,
"roh" => RM,
"run" => RN,
"rus" => RU,
"sme" => SE,
"smo" => SM,
"sag" => SG,
"san" => SA,
"srd" => SC,
"srp" => SR,
"sna" => SN,
"snd" => SD,
"sin" => SI,
"slk" => SK,
"slv" => SL,
"som" => SO,
"sot" => ST,
"spa" => ES,
"sun" => SU,
"swa" => SW,
"ssw" => SS,
"swe" => SV,
"tgl" => TL,
"tah" => TY,
"tgk" => TG,
"tam" => TA,
"tat" => TT,
"tel" => TE,
"tha" => TH,
"bod" => BO,
"tir" => TI,
"ton" => TO,
"tso" => TS,
"tsn" => TN,
"tur" => TR,
"tuk" => TK,
"twi" => TW,
"uig" => UG,
"ukr" => UK,
"urd" => UR,
"uzb" => UZ,
"ven" => VE,
"vie" => VI,
"vol" => VO,
"wln" => WA,
"cym" => CY,
"wol" => WO,
"xho" => XH,
"yid" => YI,
"yor" => YO,
"zha" => ZA,
"zul" => ZU,

};

pub const ALL_CODE: &[&str] = &[
    "ab", "aa", "af", "ak", "sq", "am", "ar", "an", "hy", "as", "av", "ae", "ay", "az", "bm", "ba",
    "eu", "be", "bn", "bi", "bs", "br", "bg", "my", "ca", "ch", "ce", "ny", "zh", "cu", "cv", "kw",
    "co", "cr", "hr", "cs", "da", "dv", "nl", "dz", "en", "eo", "et", "ee", "fo", "fj", "fi", "fr",
    "fy", "ff", "gd", "gl", "lg", "ka", "de", "el", "kl", "gn", "gu", "ht", "ha", "he", "hz", "hi",
    "ho", "hu", "is", "io", "ig", "id", "ia", "ie", "iu", "ik", "ga", "it", "ja", "jv", "kn", "kr",
    "ks", "kk", "km", "ki", "rw", "ky", "kv", "kg", "ko", "kj", "ku", "lo", "la", "lv", "li", "ln",
    "lt", "lu", "lb", "mk", "mg", "ms", "ml", "mt", "gv", "mi", "mr", "mh", "mn", "na", "nv", "nd",
    "nr", "ng", "ne", "no", "nb", "nn", "ii", "oc", "oj", "or", "om", "os", "pi", "ps", "fa", "pl",
    "pt", "pa", "qu", "ro", "rm", "rn", "ru", "se", "sm", "sg", "sa", "sc", "sr", "sn", "sd", "si",
    "sk", "sl", "so", "st", "es", "su", "sw", "ss", "sv", "tl", "ty", "tg", "ta", "tt", "te", "th",
    "bo", "ti", "to", "ts", "tn", "tr", "tk", "tw", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "wa",
    "cy", "wo", "xh", "yi", "yo", "za", "zu",
];

pub const ALL_CODE_2T: &[&str] = &[
    "abk", "aar", "afr", "aka", "sqi", "amh", "ara", "arg", "hye", "asm", "ava", "ave", "aym",
    "aze", "bam", "bak", "eus", "bel", "ben", "bis", "bos", "bre", "bul", "mya", "cat", "cha",
    "che", "nya", "zho", "chu", "chv", "cor", "cos", "cre", "hrv", "ces", "dan", "div", "nld",
    "dzo", "eng", "epo", "est", "ewe", "fao", "fij", "fin", "fra", "fry", "ful", "gla", "glg",
    "lug", "kat", "deu", "ell", "kal", "grn", "guj", "hat", "hau", "heb", "her", "hin", "hmo",
    "hun", "isl", "ido", "ibo", "ind", "ina", "ile", "iku", "ipk", "gle", "ita", "jpn", "jav",
    "kan", "kau", "kas", "kaz", "khm", "kik", "kin", "kir", "kom", "kon", "kor", "kua", "kur",
    "lao", "lat", "lav", "lim", "lin", "lit", "lub", "ltz", "mkd", "mlg", "msa", "mal", "mlt",
    "glv", "mri", "mar", "mah", "mon", "nau", "nav", "nde", "nbl", "ndo", "nep", "nor", "nob",
    "nno", "iii", "oci", "oji", "ori", "orm", "oss", "pli", "pus", "fas", "pol", "por", "pan",
    "que", "ron", "roh", "run", "rus", "sme", "smo", "sag", "san", "srd", "srp", "sna", "snd",
    "sin", "slk", "slv", "som", "sot", "spa", "sun", "swa", "ssw", "swe", "tgl", "tah", "tgk",
    "tam", "tat", "tel", "tha", "bod", "tir", "ton", "tso", "tsn", "tur", "tuk", "twi", "uig",
    "ukr", "urd", "uzb", "ven", "vie", "vol", "wln", "cym", "wol", "xho", "yid", "yor", "zha",
    "zul",
];

pub const ALL_CODE_2B: &[&str] = &[
    "abk", "aar", "afr", "aka", "alb", "amh", "ara", "arg", "arm", "asm", "ava", "ave", "aym",
    "aze", "bam", "bak", "baq", "bel", "ben", "bis", "bos", "bre", "bul", "bur", "cat", "cha",
    "che", "nya", "chi", "chu", "chv", "cor", "cos", "cre", "hrv", "cze", "dan", "div", "dut",
    "dzo", "eng", "epo", "est", "ewe", "fao", "fij", "fin", "fre", "fry", "ful", "gla", "glg",
    "lug", "geo", "ger", "gre", "kal", "grn", "guj", "hat", "hau", "heb", "her", "hin", "hmo",
    "hun", "ice", "ido", "ibo", "ind", "ina", "ile", "iku", "ipk", "gle", "ita", "jpn", "jav",
    "kan", "kau", "kas", "kaz", "khm", "kik", "kin", "kir", "kom", "kon", "kor", "kua", "kur",
    "lao", "lat", "lav", "lim", "lin", "lit", "lub", "ltz", "mac", "mlg", "may", "mal", "mlt",
    "glv", "mao", "mar", "mah", "mon", "nau", "nav", "nde", "nbl", "ndo", "nep", "nor", "nob",
    "nno", "iii", "oci", "oji", "ori", "orm", "oss", "pli", "pus", "per", "pol", "por", "pan",
    "que", "rum", "roh", "run", "rus", "sme", "smo", "sag", "san", "srd", "srp", "sna", "snd",
    "sin", "slo", "slv", "som", "sot", "spa", "sun", "swa", "ssw", "swe", "tgl", "tah", "tgk",
    "tam", "tat", "tel", "tha", "tib", "tir", "ton", "tso", "tsn", "tur", "tuk", "twi", "uig",
    "ukr", "urd", "uzb", "ven", "vie", "vol", "wln", "wel", "wol", "xho", "yid", "yor", "zha",
    "zul",
];

pub const ALL_CODE_3: &[&str] = &[
    "abk", "aar", "afr", "aka", "sqi", "amh", "ara", "arg", "hye", "asm", "ava", "ave", "aym",
    "aze", "bam", "bak", "eus", "bel", "ben", "bis", "bos", "bre", "bul", "mya", "cat", "cha",
    "che", "nya", "zho", "chu", "chv", "cor", "cos", "cre", "hrv", "ces", "dan", "div", "nld",
    "dzo", "eng", "epo", "est", "ewe", "fao", "fij", "fin", "fra", "fry", "ful", "gla", "glg",
    "lug", "kat", "deu", "ell", "kal", "grn", "guj", "hat", "hau", "heb", "her", "hin", "hmo",
    "hun", "isl", "ido", "ibo", "ind", "ina", "ile", "iku", "ipk", "gle", "ita", "jpn", "jav",
    "kan", "kau", "kas", "kaz", "khm", "kik", "kin", "kir", "kom", "kon", "kor", "kua", "kur",
    "lao", "lat", "lav", "lim", "lin", "lit", "lub", "ltz", "mkd", "mlg", "msa", "mal", "mlt",
    "glv", "mri", "mar", "mah", "mon", "nau", "nav", "nde", "nbl", "ndo", "nep", "nor", "nob",
    "nno", "iii", "oci", "oji", "ori", "orm", "oss", "pli", "pus", "fas", "pol", "por", "pan",
    "que", "ron", "roh", "run", "rus", "sme", "smo", "sag", "san", "srd", "srp", "sna", "snd",
    "sin", "slk", "slv", "som", "sot", "spa", "sun", "swa", "ssw", "swe", "tgl", "tah", "tgk",
    "tam", "tat", "tel", "tha", "bod", "tir", "ton", "tso", "tsn", "tur", "tuk", "twi", "uig",
    "ukr", "urd", "uzb", "ven", "vie", "vol", "wln", "cym", "wol", "xho", "yid", "yor", "zha",
    "zul",
];
