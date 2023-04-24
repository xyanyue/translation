use std::{collections::HashMap, fmt::Display};

use crypto::{digest::Digest, md5::Md5};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;

use crate::{
    err::{ErrInfoBuildle, TransError},
    Language, Translation,
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
#[derive(Debug, Clone, Copy)]
pub enum DomainType {
    Electronics, //电子科技领域
    Finance,     //金融财经领域
    Mechanics,   //水利机械领域
    Medicine,    //生物医药领域
    Novel,       //网络文学领域
}
impl From<DomainType> for &str {
    fn from(value: DomainType) -> Self {
        match value {
            DomainType::Electronics => "electronics",
            DomainType::Finance => "finance",
            DomainType::Mechanics => "mechanics",
            DomainType::Medicine => "medicine",
            DomainType::Novel => "novel",
        }
    }
}

pub struct Baidu<'a> {
    words: &'a str,
    appid: &'a str,
    sign: String,
    from: Language,
    to: Language,
    salt: String,
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
        let mut map = HashMap::new();
        map.insert("q", self.words.to_owned());
        map.insert("appid", self.appid.to_owned());
        map.insert("sign", self.sign.to_string());
        map.insert("salt", self.salt.to_string());
        map.insert("domain", Into::<&str>::into(self.domain).to_owned());
        map.insert("from", Into::<&str>::into(self.from).to_owned());
        map.insert("to", Into::<&str>::into(self.to).to_owned());

        let client = reqwest::blocking::Client::new();
        let request = client.post(BAIDU_API).form(&map);
        // .header(CONTENT_TYPE, "application/x-www-form-urlencoded");
        // .send();

        let response = request.send().map_err(|e| {
            TransError::RequestError(
                ErrInfoBuildle::new()
                    .model("BAIDU")
                    .method("trans()")
                    .original(e.to_string())
                    // .others(map!("token"=>token))
                    .build(),
            )
        })?;
        // let status = response.status();
        // let res = response.text();
        // println!("Res:{:?}", res);
        // Ok("".to_owned())
        // let a = ;
        let res = response.json::<BaiduRes>().map_err(|e| {
            TransError::CNKIError(
                ErrInfoBuildle::new()
                    .model("Baidu")
                    .method("trans()")
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
}

impl<'a> Baidu<'a> {
    pub fn new(words: &'a str, appid: &'a str, secret_key: &'a str) -> Baidu {
        let mut baidu = Baidu {
            words,
            appid,
            sign: String::default(),
            from: Language::Auto,
            to: Language::Zh,
            salt: String::default(),
            domain: DomainType::Medicine,
            secret_key,
        };
        baidu.set_salt();
        baidu.set_sign();
        baidu
    }
    fn set_apiid(&mut self, apiid: &'a str) {
        self.appid = apiid;
    }
    fn set_secret_key(&mut self, secret_key: &'a str) {
        self.secret_key = secret_key;
    }

    fn set_salt(&mut self) {
        self.salt = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
    }
    fn set_sign(&mut self) {
        // appid+q+salt+domain+密钥
        let s1 = format!(
            "{}{}{}{}{}",
            self.appid,
            self.words,
            self.salt,
            Into::<&str>::into(self.domain),
            self.secret_key
        );
        let mut hasher = Md5::new();

        hasher.input_str(&s1);
        self.sign = hasher.result_str();
    }
}
