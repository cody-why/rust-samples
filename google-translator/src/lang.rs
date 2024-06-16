use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy, Default)]
pub enum InputLang {
    #[default]
    Auto,
    Galego,   // 加利西亚语 - gl
    Guarani,  // 瓜拉尼语 - gn
    Gujarati, // 古吉拉特语 - gu
    Greek,    // 希腊语 - el
    // TODO
    Dutch,              // 荷兰语 - nl
    Nepali,             // 尼泊尔语 - ne
    Norwegian,          // 挪威 - no
    Danish,             // 丹麦语 - da
    Dogri,              // 多格里 - doi
    German,             // 德语 - de
    Dhivehi,            // 迪维希语 - dv
    Lao,                // 老挝语 - lo
    Latvian,            // 拉脱维亚语 - lv
    Latin,              // 拉丁 - la
    Russian,            // 俄语 - ru
    Luganda,            // 卢干达语 - lg
    Romanian,           // 罗马尼亚语 - ro
    Luxembourgish,      // 卢森堡语 - lb
    Lithuanian,         // 立陶宛语 - lt
    Lingala,            // 林加拉语 - ln
    Marathi,            // 马拉地语 - mr
    Maori,              // 毛利人 - mi
    Maithili,           // 米蒂尔语言 - mai
    Macedonian,         // 马其顿语 - mk
    Malagasy,           // 马尔加什 - mg
    Malayalam,          // 马拉雅拉姆语 - ml
    Malay,              // 马来语 - ms
    Meithei,            // 梅泰尔（曼尼普尔邦） - mni-Mtei
    Malti,              // 马耳他语 - mt
    Mongolian,          // 蒙文 - mn
    Hmong,              // 蒙 - hmn
    Burmese,            // 缅甸（缅甸语） - my
    Mizo,               // 沟沟 - lus
    Basque,             // 巴斯克 - eu
    Bambara,            // 班巴拉语 - bm
    Vietnamese,         // 越南语 - vi
    Belarusian,         // 白俄罗斯语 - be
    Bengali,            // 孟加拉 - bn
    Bosnian,            // 波斯尼亚语 - bs
    Bhojpuri,           // 博伊普尔 - bho
    NSotho,             // 北索托 - nso
    Bulgarian,          // 保加利亚语 - bg
    Samoan,             // 萨摩亚语 - sm
    Sanskrit,           // 梵文 - sa
    Serbian,            // 塞尔维亚 - sr
    Cebuano,            // 宿雾语 - ceb
    Sotho,              // 塞索托语 - st
    Somali,             // 索马里 - so
    Shona,              // 绍纳语 - sn
    Sundanese,          // 巽他语 - su
    Swahili,            // 斯瓦希里语 - sw
    Swedish,            // 瑞典 - sv
    ScottishGaelic,     // 花岗岩城 게일어 - gd
    Spanish,            // 西班牙语 - es
    Slovak,             // 斯洛伐克语 - sk
    Slovene,            // 斯洛文尼亚语 - sl
    Sindhi,             // 信德语 - sd
    Sinhala,            // 僧伽罗语 - si
    Arabic,             // 阿拉伯 - ar
    Armenian,           // 亚美尼亚语 - hy
    Assamese,           // 阿萨姆语 - as
    Aymara,             // 艾马拉 - ay
    Icelandic,          // 冰岛的 - is
    HaitianCreole,      // 海地克里奥尔语 - ht
    Irish,              // 爱尔兰人 - ga
    Azerbaijani,        // 阿塞拜疆语 - az
    Afrikaans,          // 南非荷兰语 - af
    Albanian,           // 阿尔巴尼亚人 - sq
    Amharic,            // 阿姆哈拉语 - am
    Estonian,           // 爱沙尼亚语 - et
    Esperanto,          // 世界语 - eo
    Ewe,                // 母羊 - ee
    English,            // 英语 - en
    Oromo,              // 奥罗莫 - om
    Odia,               // 奥里亚语 - or
    Yoruba,             // 约鲁巴语 - yo
    Urdu,               // 乌尔都语 - ur
    Uzbek,              // 乌兹别克语 - uz
    Ukrainian,          // 乌克兰 - uk
    Welsh,              // 威尔士语 - cy
    Uyghur,             // 维吾尔 - ug
    Igbo,               // 伊博语 - ig
    Yiddish,            // 意第绪语 - yi
    Italian,            // 意大利语 - it
    Indonesian,         // 印度尼西亚 - id
    Ilocano,            // 伊洛卡诺 - ilo
    Japanese,           // 日本人 - ja
    Javanese,           // 爪哇语 - jw
    Georgian,           // 格鲁吉亚语 - ka
    Zulu,               // 祖鲁语 - zu
    SimplifiedChinese,  // 简体中文） - zh-CN
    TraditionalChinese, // 繁体中文） - zh-TW
    Chewa,              // 切瓦 - ny
    Czech,              // 捷克语 - cs
    Tsonga,             // 特松加 - ts
    Kazakh,             // 哈萨克语 - kk
    Catalan,            // 加泰罗尼亚语 - ca
    Kannada,            // 卡纳达语 kn
    Quechuan,           // 盖丘亚语 - qu
    Corsican,           // 科西嘉岛 - co
    Xhosa,              // 科萨语 - xh
    Konkani,            // 孔卡尼 - gom
    Sorani,             // 库尔德（索拉尼） - ckb
    Kurmanji,           // 库尔特语（库尔曼吉语） - ku
    Croatian,           // 克罗地亚语 - hr
    Krio,               // 克里奥 - kri
    Khmer,              // 高棉语 - km
    Kinyarwanda,        // 基尼亚卢旺达语 - rw
    Kyrgyz,             // 吉尔吉斯语 - ky
    Tamil,              // 泰米尔语 - ta
    Tajik,              // 塔吉克 - tg
    Tatar,              // 鞑靼语 - tt
    Thai,               // 泰国 - th
    Turkish,            // 土耳其 - tr
    Telugu,             // 泰卢固语 - te
    Turkmen,            // 土库曼人 - tk
    Akan,               // 推特 - ak
    Tigrinya,           // 提格里尼亚语 - ti
    Pashto,             // 普什图语 - ps
    Punjabi,            // 旁遮普语 - pa
    Persian,            // 波斯语 - fa
    Portuguese,         // 葡萄牙语 - pt
    Polish,             // 抛光 - pl
    French,             // 法语 - fr
    Frisian,            // 弗里斯兰语 - fy
    Finnish,            // 芬兰 - fi
    Filipino,           // 菲律宾人 - tl
    Hawaiian,           // 夏威夷语 - haw
    Hausa,              // 豪萨语 - ha
    Korean,             // 韩国人 - ko
    Hungarian,          // 匈牙利 - hu
    Hebrew,             // 希伯来语 - iw
    Hindi,              // 印地语 - hi
}

unsafe impl Send for InputLang {}

impl From<String> for InputLang {
    fn from(val: String) -> Self {
        val.as_str().into()
    }
}

impl From<&String> for InputLang {
    fn from(val: &String) -> Self {
        val.as_str().into()
    }
}

impl From<&str> for InputLang {
    /// 输入语言代码、语言名称和使用区域（只有一个时）并返回 InputLang。
    fn from(val: &str) -> InputLang {
        let data = val.to_lowercase();
        match data.as_str() {
            "auto" => InputLang::Auto,

            // 加利西亚语
            "gl" | "glg" => InputLang::Galego,
            "galego" => InputLang::Galego,
            "galicia" => InputLang::Galego,
            "galician" => InputLang::Galego,
            "gallego" => InputLang::Galego,

            // 瓜拉尼语
            "gn" | "grn" | "nhd" | "gui" | "gun" | "gug" | "gnw" => InputLang::Guarani,
            "guaraní" | "guarani" => InputLang::Guarani,
            "avañe'ẽ" => InputLang::Guarani,

            // 古吉拉特语
            "gu" | "guj" => InputLang::Gujarati,
            "gujarati" => InputLang::Gujarati,
            "gujarat" => InputLang::Gujarati,
            "ગુજરાતી" => InputLang::Gujarati,
            "ગુજરાત" => InputLang::Gujarati,
            "gujarātī" => InputLang::Gujarati,

            // 希腊语
            "el" | "gre" | "ell" | "grc" | "cpg" | "gmy" | "pnt" | "tsd" | "yej" => {
                InputLang::Greek
            }
            "eλληνικά" => InputLang::Greek,
            "elliniká" => InputLang::Greek,
            "Ἑλληνική" => InputLang::Greek,
            "eλληνική" => InputLang::Greek,
            "ελληνικά" => InputLang::Greek,
            "hellēnikḗ" => InputLang::Greek,
            "greece" => InputLang::Greek,
            "hellenic" => InputLang::Greek,

            "nl" => InputLang::Dutch,
            "ne" => InputLang::Nepali,
            "no" => InputLang::Norwegian,
            "da" => InputLang::Danish,
            "doi" => InputLang::Dogri,
            "de" => InputLang::German,
            "dv" => InputLang::Dhivehi,
            "lo" => InputLang::Lao,
            "lv" => InputLang::Latvian,
            "la" => InputLang::Latin,
            "ru" => InputLang::Russian,
            "lg" => InputLang::Luganda,
            "ro" => InputLang::Romanian,
            "lb" => InputLang::Luxembourgish,
            "lt" => InputLang::Lithuanian,
            "ln" => InputLang::Lingala,
            "mr" => InputLang::Marathi,
            "mi" => InputLang::Maori,
            "mai" => InputLang::Maithili,
            "mk" => InputLang::Macedonian,
            "mg" => InputLang::Malagasy,
            "ml" => InputLang::Malayalam,
            "ms" => InputLang::Malay,
            "mni-mtei" => InputLang::Meithei,
            "mt" => InputLang::Malti,
            "mn" => InputLang::Mongolian,
            "hmn" => InputLang::Hmong,
            "my" => InputLang::Burmese,
            "lus" => InputLang::Mizo,
            "eu" => InputLang::Basque,
            "bm" => InputLang::Bambara,
            "vi" => InputLang::Vietnamese,
            "be" => InputLang::Belarusian,
            "bn" => InputLang::Bengali,
            "bs" => InputLang::Bosnian,
            "bho" => InputLang::Bhojpuri,
            "nso" => InputLang::NSotho,
            "bg" => InputLang::Bulgarian,
            "sm" => InputLang::Samoan,
            "sa" => InputLang::Sanskrit,
            "sr" => InputLang::Serbian,
            "ceb" => InputLang::Cebuano,
            "st" => InputLang::Sotho,
            "so" => InputLang::Somali,
            "sn" => InputLang::Shona,
            "su" => InputLang::Sundanese,
            "sw" => InputLang::Swahili,
            "sv" => InputLang::Swedish,
            "gd" => InputLang::ScottishGaelic,
            "es" => InputLang::Spanish,
            "sk" => InputLang::Slovak,
            "sl" => InputLang::Slovene,
            "sd" => InputLang::Sindhi,
            "si" => InputLang::Sinhala,
            "ar" => InputLang::Arabic,
            "hy" => InputLang::Armenian,
            "as" => InputLang::Assamese,
            "ay" => InputLang::Aymara,
            "is" => InputLang::Icelandic,
            "ht" => InputLang::HaitianCreole,
            "ga" => InputLang::Irish,
            "az" => InputLang::Azerbaijani,
            "af" => InputLang::Afrikaans,
            "sq" => InputLang::Albanian,
            "am" => InputLang::Amharic,
            "et" => InputLang::Estonian,
            "eo" => InputLang::Esperanto,
            "ee" => InputLang::Ewe,
            "en" => InputLang::English,
            "om" => InputLang::Oromo,
            "or" => InputLang::Odia,
            "yo" => InputLang::Yoruba,
            "ur" => InputLang::Urdu,
            "uz" => InputLang::Uzbek,
            "uk" => InputLang::Ukrainian,
            "cy" => InputLang::Welsh,
            "ug" => InputLang::Uyghur,
            "ig" => InputLang::Igbo,
            "yi" => InputLang::Yiddish,
            "it" => InputLang::Italian,
            "id" => InputLang::Indonesian,
            "ilo" => InputLang::Ilocano,
            "ja" => InputLang::Japanese,
            "jw" => InputLang::Javanese,
            "ka" => InputLang::Georgian,
            "zu" => InputLang::Zulu,
            "zh-cn" => InputLang::SimplifiedChinese,
            "zh-tw" => InputLang::TraditionalChinese,
            "ny" => InputLang::Chewa,
            "cs" => InputLang::Czech,
            "ts" => InputLang::Tsonga,
            "kk" => InputLang::Kazakh,
            "ca" => InputLang::Catalan,
            "kn" => InputLang::Kannada,
            "qu" => InputLang::Quechuan,
            "co" => InputLang::Corsican,
            "xh" => InputLang::Xhosa,
            "gom" => InputLang::Konkani,
            "ckb" => InputLang::Sorani,
            "ku" => InputLang::Kurmanji,
            "hr" => InputLang::Croatian,
            "kri" => InputLang::Krio,
            "km" => InputLang::Khmer,
            "rw" => InputLang::Kinyarwanda,
            "ky" => InputLang::Kyrgyz,
            "ta" => InputLang::Tamil,
            "tg" => InputLang::Tajik,
            "tt" => InputLang::Tatar,
            "th" => InputLang::Thai,
            "tr" => InputLang::Turkish,
            "te" => InputLang::Telugu,
            "tk" => InputLang::Turkmen,
            "ak" => InputLang::Akan,
            "ti" => InputLang::Tigrinya,
            "ps" => InputLang::Pashto,
            "pa" => InputLang::Punjabi,
            "fa" => InputLang::Persian,
            "pt" => InputLang::Portuguese,
            "pl" => InputLang::Polish,
            "fr" => InputLang::French,
            "fy" => InputLang::Frisian,
            "fi" => InputLang::Finnish,
            "tl" => InputLang::Filipino,
            "haw" => InputLang::Hawaiian,
            "ha" => InputLang::Hausa,
            "ko" => InputLang::Korean,
            "hu" => InputLang::Hungarian,
            "iw" => InputLang::Hebrew,
            "hi" => InputLang::Hindi,

            _ => unreachable!("Invalid language code: {}", data)
        }
    }
}
impl Display for InputLang {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let lang = match self {
            InputLang::Auto => "auto",
            InputLang::Galego => "gl",
            InputLang::Guarani => "gn",
            InputLang::Gujarati => "gu",
            InputLang::Greek => "el",
            InputLang::Dutch => "nl",
            InputLang::Nepali => "ne",
            InputLang::Norwegian => "no",
            InputLang::Danish => "da",
            InputLang::Dogri => "doi",
            InputLang::German => "de",
            InputLang::Dhivehi => "dv",
            InputLang::Lao => "lo",
            InputLang::Latvian => "lv",
            InputLang::Latin => "la",
            InputLang::Russian => "ru",
            InputLang::Luganda => "lg",
            InputLang::Romanian => "ro",
            InputLang::Luxembourgish => "lb",
            InputLang::Lithuanian => "lt",
            InputLang::Lingala => "ln",
            InputLang::Marathi => "mr",
            InputLang::Maori => "mi",
            InputLang::Maithili => "mai",
            InputLang::Macedonian => "mk",
            InputLang::Malagasy => "mg",
            InputLang::Malayalam => "ml",
            InputLang::Malay => "ms",
            InputLang::Meithei => "mni-Mtei",
            InputLang::Malti => "mt",
            InputLang::Mongolian => "mn",
            InputLang::Hmong => "hmn",
            InputLang::Burmese => "my",
            InputLang::Mizo => "lus",
            InputLang::Basque => "eu",
            InputLang::Bambara => "bm",
            InputLang::Vietnamese => "vi",
            InputLang::Belarusian => "be",
            InputLang::Bengali => "bn",
            InputLang::Bosnian => "bs",
            InputLang::Bhojpuri => "bho",
            InputLang::NSotho => "nso",
            InputLang::Bulgarian => "bg",
            InputLang::Samoan => "sm",
            InputLang::Sanskrit => "sa",
            InputLang::Serbian => "sr",
            InputLang::Cebuano => "ceb",
            InputLang::Sotho => "st",
            InputLang::Somali => "so",
            InputLang::Shona => "sn",
            InputLang::Sundanese => "su",
            InputLang::Swahili => "sw",
            InputLang::Swedish => "sv",
            InputLang::ScottishGaelic => "gd",
            InputLang::Spanish => "es",
            InputLang::Slovak => "sk",
            InputLang::Slovene => "sl",
            InputLang::Sindhi => "sd",
            InputLang::Sinhala => "si",
            InputLang::Arabic => "ar",
            InputLang::Armenian => "hy",
            InputLang::Assamese => "as",
            InputLang::Aymara => "ay",
            InputLang::Icelandic => "is",
            InputLang::HaitianCreole => "ht",
            InputLang::Irish => "ga",
            InputLang::Azerbaijani => "az",
            InputLang::Afrikaans => "af",
            InputLang::Albanian => "sq",
            InputLang::Amharic => "am",
            InputLang::Estonian => "et",
            InputLang::Esperanto => "eo",
            InputLang::Ewe => "ee",
            InputLang::English => "en",
            InputLang::Oromo => "om",
            InputLang::Odia => "or",
            InputLang::Yoruba => "yo",
            InputLang::Urdu => "ur",
            InputLang::Uzbek => "uz",
            InputLang::Ukrainian => "uk",
            InputLang::Welsh => "cy",
            InputLang::Uyghur => "ug",
            InputLang::Igbo => "ig",
            InputLang::Yiddish => "yi",
            InputLang::Italian => "it",
            InputLang::Indonesian => "id",
            InputLang::Ilocano => "ilo",
            InputLang::Japanese => "ja",
            InputLang::Javanese => "jw",
            InputLang::Georgian => "ka",
            InputLang::Zulu => "zu",
            InputLang::SimplifiedChinese => "zh-CN",
            InputLang::TraditionalChinese => "zh-TW",
            InputLang::Chewa => "ny",
            InputLang::Czech => "cs",
            InputLang::Tsonga => "ts",
            InputLang::Kazakh => "kk",
            InputLang::Catalan => "ca",
            InputLang::Kannada => "kn",
            InputLang::Quechuan => "qu",
            InputLang::Corsican => "co",
            InputLang::Xhosa => "xh",
            InputLang::Konkani => "gom",
            InputLang::Sorani => "ckb",
            InputLang::Kurmanji => "ku",
            InputLang::Croatian => "hr",
            InputLang::Krio => "kri",
            InputLang::Khmer => "km",
            InputLang::Kinyarwanda => "rw",
            InputLang::Kyrgyz => "ky",
            InputLang::Tamil => "ta",
            InputLang::Tajik => "tg",
            InputLang::Tatar => "tt",
            InputLang::Thai => "th",
            InputLang::Turkish => "tr",
            InputLang::Telugu => "te",
            InputLang::Turkmen => "tk",
            InputLang::Akan => "ak",
            InputLang::Tigrinya => "ti",
            InputLang::Pashto => "ps",
            InputLang::Punjabi => "pa",
            InputLang::Persian => "fa",
            InputLang::Portuguese => "pt",
            InputLang::Polish => "pl",
            InputLang::French => "fr",
            InputLang::Frisian => "fy",
            InputLang::Finnish => "fi",
            InputLang::Filipino => "tl",
            InputLang::Hawaiian => "haw",
            InputLang::Hausa => "ha",
            InputLang::Korean => "ko",
            InputLang::Hungarian => "hu",
            InputLang::Hebrew => "iw",
            InputLang::Hindi => "hi",
        };
        write!(f, "{}", lang)
    }
}
