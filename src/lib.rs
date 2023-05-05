//! 这是一个翻译软件包。默认主要用于英文=>中文 的医学文档翻译
//!
//! 现阶段一共封装了[CNKI API](https://dict.cnki.net/)和[百度翻译API](https://fanyi-api.baidu.com/)，
//! CNKI为逆向的其网页版*不稳定*，百度使用的官方API，需要APIID*稳定*,使用量大需付费
//!
//! # Example
//!
//! ### CNKI 单句翻译
//! ```no_run
//! let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";
//!
//! let check_res = r#"患者男，55岁，因"家族性腺瘤性息肉病( FAP ) "背景下十二指肠异型增生性息肉就诊。患者既往有结肠次全切除回直肠吻合术史，1981年形成回肛袋，2000年(早了20多年)行造口还纳术。"#;
//! let mut cnki = Translate::Cnki.create();
//! //此值是反编译JS得到,默认值是此。若变化，自行修改时可传递
//! //cnki.set_secret_key("4e87183cfd3a45fe");
//! //修改为中 => 英. 默认为英 => 中
//! //cnki.set_from(Language::Zh);
//! cnki.set_words(words);
//! match cnki.trans() {
//!     Ok(tr_res) => {
//!         println!("{}\n{:?}", words, tr_res);
//!         assert_eq!(tr_res, check_res.to_owned())
//!     }
//!     Err(e) => println!("Err:{}", e),
//! }
//!
//! ```
//!
//! 只支持中英互译，无需调整参数，和网页使用效果相同。但是若网站调整加密算法或者接口，将失效
//!
//! 最大支持1200英文字符，600中文。翻译专业文档和术语相对最准确
//!
//! ### CNKI 文档翻译
//! ```no run
//!     let docs = "Left masticator space shows an altered signal intensity heterogeneous expansile soft tissue mass, showing iso-intense signal intensity on T1W, heterogeneously hyperintense to intermediate signal intensity on T2W lesion with low signal striations, seen arising from the posterior body and ramus of the mandible. The lesion shows diffusion restriction and heterogeneous enhancement on post-contrast images.
//!     The lesion extends from the anterior to the posterior border of the ramus of the mandible with thinning of the ramus of the mandible.  The lesion is infiltrating the left masseter muscle laterally and medial pterygoid muscle medially. The left parotid gland is seen compressed along the inferolateral margin of the lesion and is medially displacing the left parapharyngeal space towards the midline without infiltrating it.
//!     Ultrasound-guided core needle biopsy with needle tip within the lesion showing reverberation artifact.Histopathology reports suggest a poorly differentiated malignancy.
//!     Fluorescence in situ hybridization (FISH) is positive for EWSR1 (Ewing sarcoma breakpoint region 1) gene re-arrangement.
//!     The immunohistochemistry (IHC) panel shows the tumor is positive for CK (moderately), NKX 2.2, and CD99, which are features of Ewing sarcoma.";
//!     let mut cnki = Translate::Cnki.create();
//!     // cnki.set_secret_key("4e87183cfd3a45fe");
//!     println!("{:?}", cnki.docs(docs));
//! ```
//! 文档翻译,使用简单的标点【'!', '.', '?', ';', '？', '。', '；', '！'】切分。
//! 结果生成一个Vec<String>。
//!
//! 未检测max_len,如果分割之后，还有超过max_len的情况，将返回固定的" ---translation err--- "。
//!
//! 所有的错误情况，都在对应句子位置 返回 " ---translation err--- "。
//!
//! 若有更精确的需求，请自行实现，或提issue
//!
//! ### Baidu API
//! ```no_run
//! let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";
//!
//! let check_res = r#"一名55岁的男性在家族性腺瘤性息肉病（FAP）的背景下诊断为十二指肠息肉增生后进行术前评估。该患者曾于1981年接受结肠次全切除术，回肠直肠吻合术并形成回肠肛囊，随后于2000年（20多年前）造口逆转。"#;
//! let mut baidu = Translate::Baidu.create();
//!
//! baidu.set_appid("xxxxxxxxxxxx"); // 必须
//! baidu.set_secret_key("xxxxxxxxxxxx"); // 必须
//! baidu.set_from(Language::Auto); // default:Language::Auto
//! baidu.set_to(Language::Zh); // default:Language::Zh
//! baidu.set_domain(DomainType::Medicine); // default:DomainType::Medicine
//! baidu.set_words(words);
//!
//! match baidu.trans() {
//!     Ok(tr_res) => {
//!         println!("{:?}\n{:?}", words, tr_res);
//!         assert_eq!(tr_res, check_res.to_owned())
//!     }
//!     Err(e) => println!("Err:{}", e),
//! }
//!  // BAIDU_APPID & BAIDU_secret_key 请自行前往官网注册。 垂直领域，每月50万免费字符,超出后仅收取超出部分费用，49元/百万字符
//! ```
//!
//! 支持多种翻译。默认英文=>中文。
//!
//! #### Baidu文档类型
//! ```no_run
//! let docs = "Left masticator space shows an altered signal intensity heterogeneous expansile soft tissue mass, showing iso-intense signal intensity on T1W, heterogeneously hyperintense to intermediate signal intensity on T2W lesion with low signal striations, seen arising from the posterior body and ramus of the mandible. The lesion shows diffusion restriction and heterogeneous enhancement on post-contrast images.
//! The lesion extends from the anterior to the posterior border of the ramus of the mandible with thinning of the ramus of the mandible.  The lesion is infiltrating the left masseter muscle laterally and medial pterygoid muscle medially. The left parotid gland is seen compressed along the inferolateral margin of the lesion and is medially displacing the left parapharyngeal space towards the midline without infiltrating it.
//! Ultrasound-guided core needle biopsy with needle tip within the lesion showing reverberation artifact.Histopathology reports suggest a poorly differentiated malignancy.
//! Fluorescence in situ hybridization (FISH) is positive for EWSR1 (Ewing sarcoma breakpoint region 1) gene re-arrangement.
//! The immunohistochemistry (IHC) panel shows the tumor is positive for CK (moderately), NKX 2.2, and CD99, which are features of Ewing sarcoma.";
//! let mut baidu = Translate::Baidu.create();
//! baidu.set_appid("xxxxxxxxxxxx");
//! baidu.set_secret_key("xxxxxxxxxxxxxx");
//! baidu.set_from(Language::En);
//! baidu.set_to(Language::Zh);
//! baidu.set_domain(DomainType::Medicine);
//! println!("{:?}", baidu.docs(docs));
//! ```
//! 使用垂直行业接口,若使用其他行业，或通用领域，请自行更换接口API
macro_rules! map {
    ( $( $k:expr => $v:expr ),* ) => {
        {
            let mut temp_map = HashMap::new();
            $(
                temp_map.insert(stringify!($k).to_string(),stringify!($v).to_string());
            )*
            temp_map
        }
    };
}

pub mod baidu;
pub mod cnkis;
pub mod err;
extern crate ecb;
use err::TransError;
type Result<T> = std::result::Result<T, TransError>;
/// 翻译Trait,所有翻译接口都实现这个Trait
pub trait Translation<'a> {
    /// 设置句子
    fn set_words(&mut self, words: &'a str);
    /// 设置翻译源语言
    fn set_from(&mut self, language: Language);
    /// 设置翻译结果语言
    fn set_to(&mut self, language: Language);
    /// 设置appid
    fn set_appid(&mut self, appid: &'a str);
    /// 设置appid
    fn set_secret_key(&mut self, secret_key: &'a str);
    /// 设置翻译领域
    fn set_domain(&mut self, domain: DomainType);
    /// 设置单句最大长度
    fn set_max_length(&mut self, l: usize);
    /// 获取最大支持长度
    fn max_length(&mut self) -> usize;
    /// 获取传入的句子
    fn get_words(&self) -> &'a str;
    /// 获取源的语言类型
    fn from(&self) -> Language;
    /// 获取目标的语言类型
    fn to(&self) -> Language;
    /// 接口APP ID
    fn appid(&self) -> &'a str;
    /// 接口秘钥
    fn secret_key(&self) -> &'a str;
    /// 领域
    fn domain(&self) -> DomainType;
    /// 创建一个翻译器
    fn build(&self) -> Box<dyn Translation<'a> + 'a>;
    /// 开始调用翻译API 翻译
    fn trans(&mut self) -> Result<String>;
    /// 文档翻译,使用简单的标点【'!', '.', '?', ';', '？', '。', '；', '！'】切分
    /// 结果生成一个Vec<String>
    /// 未检测max_len,如果分割之后，还有超过max_len的情况，将返回固定的" ---translation err--- "
    /// 所有的错误情况，都在对应句子位置 返回 " ---translation err--- "
    /// 若有更精确的需求，请自行实现，或提issue
    fn docs(&mut self, docs: &'a str) -> Vec<String> {
        let mut res: Vec<String> = Vec::default();
        for x in docs.split_inclusive(&['!', '.', '?', ';', '？', '。', '；', '！'][..]) {
            self.set_words(x);
            match self.trans() {
                Ok(r) => res.push(r),
                Err(e) => res.push(" ---translation err--- ".to_owned()),
            }
        }
        res
    }
}

// enum Punctuation {}

/// 语言类型
#[derive(Debug, Clone, Copy)]
pub enum Language {
    Zh,   //中文
    En,   //英语
    Jp,   //日语
    Kor,  //韩语
    Th,   //泰语
    Vie,  //越南语
    Ru,   //俄语
    Auto, //自动
}
/// 将语言类型，翻译为字符串。百度api使用
impl From<Language> for &str {
    fn from(value: Language) -> Self {
        match value {
            Language::Zh => "zh",
            Language::En => "en",
            Language::Jp => "jp",
            Language::Kor => "kor",
            Language::Th => "th",
            Language::Vie => "vie",
            Language::Ru => "ru",
            Language::Auto => "auto",
        }
    }
}
/// 翻译领域模型
#[derive(Debug, Clone, Copy)]
pub enum DomainType {
    /// 电子科技领域
    Electronics,
    /// 金融财经领域
    Finance,
    /// 水利机械领域
    Mechanics,
    /// 生物医药领域
    Medicine,
    /// 网络文学领域
    Novel,
    /// 通用领域 || 无区分领域 传值 百度垂直传递此值 默认electronics
    General,
}
impl From<DomainType> for &str {
    fn from(value: DomainType) -> Self {
        match value {
            DomainType::Electronics => "electronics",
            DomainType::Finance => "finance",
            DomainType::Mechanics => "mechanics",
            DomainType::Medicine => "medicine",
            DomainType::Novel => "novel",
            DomainType::General => "electronics",
        }
    }
}

/// 入口Enum。将使用此enum切换特定的API
pub enum Translate {
    Cnki,
    Baidu,
    // Xunfei,
    // Youdao,
    // Deepl,
    // Ali,
    // Tencent,
}
impl Translate {
    pub fn create(&self) -> Box<dyn Translation + '_> {
        match self {
            Translate::Cnki => Box::new(cnkis::CNKI::default_new()),
            Translate::Baidu => Box::new(baidu::Baidu::new()),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works_CNKI() {
//         let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";

//         let check_res = r#"患者男，55岁，因"家族性腺瘤性息肉病( FAP ) "背景下十二指肠异型增生性息肉就诊。患者既往有结肠次全切除回直肠吻合术史，1981年形成回肛袋，2000年(早了20多年)行造口还纳术。"#;

//         let mut cnki = Translate::Cnki.create();
//         cnki.set_secret_key("4e87183cfd3a45fe");
//         cnki.set_words(words);
//         match cnki.trans() {
//             Ok(tr_res) => {
//                 println!("{}\n{:?}", words, tr_res);
//                 assert_eq!(tr_res, check_res.to_owned())
//             }
//             Err(e) => println!("Err:{}", e),
//         }
//     }

//     #[test]
//     fn it_works_BAIDU() {
//         let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";

//         let check_res = r#"一名55岁的男性在家族性腺瘤性息肉病（FAP）的背景下诊断为十二指肠息肉增生后进行术前评估。该患者曾于1981年接受结肠次全切除术，回肠直肠吻合术并形成回肠肛囊，随后于2000年（20多年前）造口逆转。"#;

//         let mut baidu = Translate::Baidu.create();

//         baidu.set_appid("xxxxxxxxxxxx");
//         baidu.set_secret_key("xxxxxxxxxxxx");
//         baidu.set_from(Language::Auto);
//         baidu.set_to(Language::Zh);
//         baidu.set_domain(DomainType::Medicine);
//         baidu.set_words(words);
//         // // 此处可使用环境变量一类的
//         match baidu.trans() {
//             Ok(tr_res) => {
//                 println!("{:?}\n{:?}", words, tr_res);
//                 assert_eq!(tr_res, check_res.to_owned())
//             }
//             Err(e) => println!("Err:{}", e),
//         }
//     }
//     #[test]
//     fn test_cnki_docs() {
//         let docs = "Left masticator space shows an altered signal intensity heterogeneous expansile soft tissue mass, showing iso-intense signal intensity on T1W, heterogeneously hyperintense to intermediate signal intensity on T2W lesion with low signal striations, seen arising from the posterior body and ramus of the mandible. The lesion shows diffusion restriction and heterogeneous enhancement on post-contrast images.
//         The lesion extends from the anterior to the posterior border of the ramus of the mandible with thinning of the ramus of the mandible.  The lesion is infiltrating the left masseter muscle laterally and medial pterygoid muscle medially. The left parotid gland is seen compressed along the inferolateral margin of the lesion and is medially displacing the left parapharyngeal space towards the midline without infiltrating it.
//         Ultrasound-guided core needle biopsy with needle tip within the lesion showing reverberation artifact.Histopathology reports suggest a poorly differentiated malignancy.
//         Fluorescence in situ hybridization (FISH) is positive for EWSR1 (Ewing sarcoma breakpoint region 1) gene re-arrangement.
//         The immunohistochemistry (IHC) panel shows the tumor is positive for CK (moderately), NKX 2.2, and CD99, which are features of Ewing sarcoma.";
//         let mut cnki = Translate::Cnki.create();
//         // cnki.set_secret_key("4e87183cfd3a45fe");
//         println!("{:?}", cnki.docs(docs));
//     }
//     #[test]
//     fn test_baidu_docs() {
//         let docs = "Left masticator space shows an altered signal intensity heterogeneous expansile soft tissue mass, showing iso-intense signal intensity on T1W, heterogeneously hyperintense to intermediate signal intensity on T2W lesion with low signal striations, seen arising from the posterior body and ramus of the mandible. The lesion shows diffusion restriction and heterogeneous enhancement on post-contrast images.
//         The lesion extends from the anterior to the posterior border of the ramus of the mandible with thinning of the ramus of the mandible.  The lesion is infiltrating the left masseter muscle laterally and medial pterygoid muscle medially. The left parotid gland is seen compressed along the inferolateral margin of the lesion and is medially displacing the left parapharyngeal space towards the midline without infiltrating it.
//         Ultrasound-guided core needle biopsy with needle tip within the lesion showing reverberation artifact.Histopathology reports suggest a poorly differentiated malignancy.
//         Fluorescence in situ hybridization (FISH) is positive for EWSR1 (Ewing sarcoma breakpoint region 1) gene re-arrangement.
//         The immunohistochemistry (IHC) panel shows the tumor is positive for CK (moderately), NKX 2.2, and CD99, which are features of Ewing sarcoma.";
//         let mut baidu = Translate::Baidu.create();
//         baidu.set_appid("xxxxxxxxxxxx");
//         baidu.set_secret_key("xxxxxxxxxxxxxx");
//         baidu.set_from(Language::En);
//         baidu.set_to(Language::Zh);
//         baidu.set_domain(DomainType::Medicine);
//         println!("{:?}", baidu.docs(docs));
//     }
// }
