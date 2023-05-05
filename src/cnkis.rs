use std::collections::HashMap;

use crate::{
    err::{ErrInfoBuildle, TransError},
    DomainType, Language, Translation,
};
use aes::cipher::{block_padding::Pkcs7, BlockEncryptMut, KeyInit};
use base64::{
    alphabet,
    engine::{self, general_purpose},
    Engine as _,
};
use reqwest::header::{REFERER, USER_AGENT};
use serde::Deserialize;
use serde_json::Value;

type Result<T> = std::result::Result<T, TransError>;
type Aes128EcbEnc = ecb::Encryptor<aes::Aes128>;
const TOKEN_URL: &str = "https://dict.cnki.net/fyzs-front-api/getToken";
const TRANSLATION_API: &str = "https://dict.cnki.net/fyzs-front-api/translate/literaltranslation";
const MAX_LEN: usize = 1200;

#[derive(Debug, Deserialize)]
pub struct CnkiRes {
    pub msg: String,
    pub code: usize,
    pub data: Data,
}
#[derive(Debug, Deserialize)]
pub struct Data {
    pub words: String,
    pub mResult: String,
    // pub ikWords: Option<Vec<String>>,
    // pub translateType: usize,
    // pub translateMethod: usize,
    pub code: usize,
    // pub dictsVos: Option<Value>,
    // pub adictsVos: Option<Value>,
    // pub relatedSearchs: Option<Value>,
    // pub academicDicts: Option<Value>,
    // pub translateFlag: usize,
    // pub machinetranslateFlag: usize,
    // pub isInputVerificationCode: usize,
}

pub struct CNKI<'a> {
    words: &'a str,
    encode_words: String,
    max_len: usize,
    key: &'a str,
    agent: String, // padding:
    tras_type: usize,
}

impl<'a> Translation<'a> for CNKI<'a> {
    fn set_words(&mut self, words: &'a str) {
        self.words = words;
    }

    fn get_words(&self) -> &'a str {
        self.words
    }

    fn trans(&mut self) -> Result<String> {
        match self.check_len() {
            Ok(_) => self.get_res(self.get_token()?),
            Err(e) => Err(e),
        }
    }

    fn set_from(&mut self, language: crate::Language) {
        match language {
            crate::Language::Zh => self.tras_type = 0,
            crate::Language::En => self.tras_type = 1,
            _ => self.tras_type = 1,
        }
    }

    fn set_to(&mut self, language: crate::Language) {
        match language {
            crate::Language::Zh => self.tras_type = 1,
            crate::Language::En => self.tras_type = 0,
            _ => self.tras_type = 1,
        }
    }
    fn set_appid(&mut self, appid: &'a str) {}

    fn set_secret_key(&mut self, secret_key: &'a str) {
        self.key = secret_key;
    }

    fn set_domain(&mut self, domain: DomainType) {}

    fn from(&self) -> Language {
        match self.tras_type {
            0 => Language::Zh,
            1 => Language::En,
            _ => Language::En,
        }
    }

    fn to(&self) -> Language {
        match self.tras_type {
            0 => Language::En,
            1 => Language::Zh,
            _ => Language::Zh,
        }
    }

    fn appid(&self) -> &'a str {
        ""
    }

    fn secret_key(&self) -> &'a str {
        self.key
    }

    fn domain(&self) -> DomainType {
        DomainType::General
    }

    fn build(&self) -> Box<dyn Translation<'a> + 'a> {
        Box::new(CNKI::new())
    }

    fn set_max_length(&mut self, l: usize) {
        self.max_len = l;
    }

    fn max_length(&mut self) -> usize {
        self.max_len
    }
}

impl<'a> CNKI<'a> {
    pub fn new() -> Self {
        Self {
            words: "",
            key: "",
            agent: rand_user_agent::UserAgent::random().to_string(),
            encode_words: String::default(),
            tras_type: 1,
            max_len: MAX_LEN,
        }
    }
    pub fn default_new() -> Self {
        Self {
            words: "",
            key: "4e87183cfd3a45fe",
            agent: rand_user_agent::UserAgent::random().to_string(),
            encode_words: String::default(),
            tras_type: 1,
            max_len: MAX_LEN,
        }
    }
    fn get_res(&mut self, token: String) -> Result<String> {
        self.aes_ecb_pkcs7_words_no_url();
        let mut map = HashMap::new();
        map.insert("words", self.encode_words.to_string());
        map.insert("translateType", self.tras_type.to_string());
        let client = reqwest::blocking::Client::new();
        let request = client
            .post(TRANSLATION_API)
            .header("Token", token.to_string())
            .header(USER_AGENT, self.agent.as_str())
            .header(REFERER, "https://dict.cnki.net/index")
            .json(&map);

        let response = request.send().map_err(|e| {
            TransError::RequestError(
                ErrInfoBuildle::new()
                    .model("CNKI")
                    .method("get_res(token)")
                    .original(e.to_string())
                    .data(format!("{}", token))
                    .build(),
            )
        })?;
        let status = response.status();
        // let res = response.text();
        // println!("Res:{:?}", res);
        // Ok("".to_owned())
        let res = response.json::<CnkiRes>().map_err(|e| {
            TransError::CNKIError(
                ErrInfoBuildle::new()
                    .model("CNKI")
                    .method("get_res(token)")
                    .original(e.to_string())
                    .data(format!("status code:{}", status))
                    .build(),
            )
        })?;
        Ok(res.data.mResult)
    }
    fn get_token(&self) -> Result<String> {
        let response = reqwest::blocking::get(TOKEN_URL).map_err(|e| {
            TransError::RequestError(
                ErrInfoBuildle::new()
                    .model("CNKI")
                    .method("get_token()")
                    .original(e.to_string())
                    .build(),
            )
        })?;
        let status = response.status();
        let body = response.json::<HashMap<String, Value>>().map_err(|e| {
            TransError::CNKIError(
                ErrInfoBuildle::new()
                    .model("CNKI")
                    .method("get_token())")
                    .original(e.to_string())
                    .data(format!("status code:{}", status))
                    .build(),
            )
        })?;

        match body.get("data") {
            Some(d) => Ok(d.to_string()),
            None => Err(TransError::CNKIError(
                ErrInfoBuildle::new()
                    .model("CNKI")
                    .method("get_token())")
                    .others(map!("data"=>response.text().unwrap()))
                    .build(),
            )),
        }
    }
    fn aes_ecb_pkcs7_words(&mut self) {
        let res = Aes128EcbEnc::new(self.key.as_bytes().into())
            .encrypt_padded_vec_mut::<Pkcs7>(self.words.as_bytes());

        let encoded: String = general_purpose::URL_SAFE_NO_PAD.encode(res);
        self.encode_words = encoded
        // println!("{}", encoded);
    }
    fn check_len(&self) -> Result<bool> {
        if self.words.len() > self.max_len {
            return Err(TransError::CNKIError(
                ErrInfoBuildle::new()
                    .model("CNKI")
                    .method("check_len()")
                    .original(format!("字符太长了,Max:{}", self.max_len))
                    .build(),
            ));
        }
        return Ok(true);
    }
    fn aes_ecb_pkcs7_words_no_url(&mut self) {
        let res = Aes128EcbEnc::new(self.key.as_bytes().into())
            .encrypt_padded_vec_mut::<Pkcs7>(self.words.as_bytes());
        let alphabet = alphabet::Alphabet::new(
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_",
        )
        .unwrap();
        let crazy_config = engine::GeneralPurposeConfig::new();
        let crazy_engine = engine::GeneralPurpose::new(&alphabet, crazy_config);
        let encoded: String = crazy_engine.encode(res);
        self.encode_words = encoded
    }
}

// #[test]
// fn test() {
//     let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";
//     let mut t = CNKI::new(words);
//     let ass_res = r#"患者男，55岁，因"家族性腺瘤性息肉病( FAP ) "背景下十二指肠异型增生性息肉就诊。患者既往有结肠次全切除回直肠吻合术史，1981年形成回肛袋，2000年(早了20多年)行造口还纳术。"#;
//     match t.trans() {
//         Ok(tr_res) => {
//             println!("{}\n{:?}", words, tr_res);
//             assert_eq!(tr_res, ass_res.to_owned())
//         }
//         Err(e) => println!("Err:{}", e),
//     }
// }
