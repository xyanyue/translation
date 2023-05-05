use std::{collections::HashMap, fmt::Display};

use crypto::{digest::Digest, md5::Md5};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::{
    err::{ErrInfoBuildle, TransError},
    DomainType, Language, Translation,
};

const BAIDU_API: &str = "https://fanyi-api.baidu.com/api/trans/vip/fieldtranslate";
type Result<T> = std::result::Result<T, TransError>;

#[derive(Debug, Deserialize)]
pub struct BaiduRes {
    pub from: String,
    pub to: String,
    pub trans_result: Vec<ReturnResult>,
}
#[derive(Debug, Deserialize)]
pub struct ReturnResult {
    pub src: String,
    pub dst: String,
}
impl Display for BaiduRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in &self.trans_result {
            write!(f, "{}", v.dst);
        }
        Ok(())
    }
}

pub struct Baidu<'a> {
    words: &'a str,
    appid: &'a str,
    // sign: String,
    from: Language,
    to: Language,
    max_len: usize,
    domain: DomainType,
    secret_key: &'a str,
}

impl<'a> Translation<'a> for Baidu<'a> {
    fn set_words(&mut self, words: &'a str) {
        self.words = words;
    }

    fn get_words(&self) -> &'a str {
        self.words
    }

    fn trans(&mut self) -> Result<String> {
        self.check()?;
        let map = self.get_data();
        let client = reqwest::blocking::Client::new();
        let request = client.post(BAIDU_API).form(&map);

        let response = request.send().map_err(|e| {
            TransError::RequestError(
                ErrInfoBuildle::new()
                    .model("BAIDU")
                    .method("trans() response")
                    .original(e.to_string())
                    .build(),
            )
        })?;
        // let status = response.status();
        // let res = response.text();
        // println!("Res:{:?}", res);
        // Ok("".to_owned())
        // let a = ;
        let res = response.json::<BaiduRes>().map_err(|e| {
            TransError::BaiduError(
                ErrInfoBuildle::new()
                    .model("Baidu")
                    .method("trans() res")
                    .original(e.to_string())
                    .data(serde_json::to_string(&map).unwrap())
                    .build(),
            )
        })?;
        Ok(res.to_string())
    }

    fn set_from(&mut self, language: Language) {
        self.from = language
    }

    fn set_to(&mut self, language: Language) {
        self.to = language
    }

    fn set_appid(&mut self, appid: &'a str) {
        self.appid = appid
    }

    fn set_secret_key(&mut self, secret_key: &'a str) {
        self.secret_key = secret_key
    }

    fn set_domain(&mut self, domain: DomainType) {
        self.domain = domain
    }

    fn build(&self) -> Box<dyn Translation<'a> + 'a> {
        Box::new(Baidu::new())
    }

    fn from(&self) -> Language {
        self.from
    }

    fn to(&self) -> Language {
        self.to
    }

    fn appid(&self) -> &'a str {
        self.appid
    }

    fn secret_key(&self) -> &'a str {
        self.secret_key
    }

    fn domain(&self) -> DomainType {
        self.domain
    }

    fn set_max_length(&mut self, l: usize) {
        self.max_len = l;
    }

    fn max_length(&mut self) -> usize {
        self.max_len
    }
}

impl<'a> Baidu<'a> {
    pub fn new() -> Baidu<'a> {
        Baidu {
            words: "",
            appid: "",
            from: Language::Auto,
            to: Language::Zh,
            domain: DomainType::Medicine,
            secret_key: "",
            max_len: 1500,
        }
    }
    fn salt(&self) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect()
    }
    fn sign(&self) -> (String, String) {
        let salt = self.salt();
        // appid+q+salt+domain+密钥
        let s1 = format!(
            "{}{}{}{}{}",
            self.appid,
            self.words,
            salt,
            Into::<&str>::into(self.domain),
            self.secret_key
        );
        let mut hasher = Md5::new();

        hasher.input_str(&s1);
        (salt, hasher.result_str())
    }
    fn get_data(&self) -> HashMap<&str, String> {
        let mut map = HashMap::new();
        let (salt, sign) = self.sign();
        map.insert("q", self.words.to_owned());
        map.insert("appid", self.appid.to_owned());
        map.insert("sign", sign);
        map.insert("salt", salt);
        map.insert("domain", Into::<&str>::into(self.domain).to_owned());
        map.insert("from", Into::<&str>::into(self.from).to_owned());
        map.insert("to", Into::<&str>::into(self.to).to_owned());
        map
    }
    fn check(&self) -> Result<()> {
        let l = self.words.len();
        if l > self.max_len {
            return Err(TransError::BaiduError(
                ErrInfoBuildle::new()
                    .model("Baidu")
                    .method("check len")
                    .original(format!("字符太长了,Max:{}", self.max_len))
                    .build(),
            ));
        }

        if l <= 0 {
            return Err(TransError::BaiduError(
                ErrInfoBuildle::new()
                    .model("Baidu")
                    .method("check len")
                    .original(format!("字符words未设置了"))
                    .build(),
            ));
        }

        if self.appid().len() <= 0 || self.secret_key().len() <= 0 {
            return Err(TransError::BaiduError(
                ErrInfoBuildle::new()
                    .model("Baidu")
                    .method("check")
                    .original(format!("未设置必要的Appid和秘钥：请调用baidu.set_appid(X) & baidu.set_secret_key(X)"))
                    .build(),
            ));
        }

        return Ok(());
    }
}
