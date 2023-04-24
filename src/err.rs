use std::{collections::HashMap, fmt};

pub struct ErrInfoBuildle(HashMap<String, String>);
impl ErrInfoBuildle {
    pub fn new() -> ErrInfoBuildle {
        ErrInfoBuildle(HashMap::default())
    }

    pub fn model<'a>(&mut self, model: &'a str) -> &mut Self {
        self.0.insert("MODEL".to_owned(), model.to_owned());
        self
    }
    pub fn method<'a>(&mut self, method: &'a str) -> &mut Self {
        self.0.insert("METHOD".to_owned(), method.to_owned());
        self
    }
    pub fn data<'a>(&mut self, data: String) -> &mut Self {
        self.0.insert("DATA".to_owned(), data);
        self
    }
    pub fn original(&mut self, err: String) -> &mut Self {
        self.0.insert("ORIGINAL".to_owned(), err);
        self
    }

    pub fn others(&mut self, o: HashMap<String, String>) -> &mut Self {
        for v in o {
            self.0.insert(v.0.to_uppercase(), v.1);
        }
        self
    }
    pub fn build(&mut self) -> ErrInfo {
        ErrInfo::build(self.0.clone())
    }
}

pub struct ErrInfo {
    // Original: String,
    Info: HashMap<String, String>,
}

impl ErrInfo {
    pub fn build(info: HashMap<String, String>) -> Self {
        Self {
            // Original: original,
            Info: info,
        }
    }
    pub fn add<'a>(&mut self, key: &'a str, value: &'a str) -> &mut Self {
        self.Info.insert(key.to_string(), value.to_string());
        self
    }
}
impl fmt::Display for ErrInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // let a = write!(f, "");
        for info in &self.Info {
            write!(f, "\n{}=>{}", info.0, info.1)?;
        }
        Ok(())
    }
}

pub enum TransError {
    RequestError(ErrInfo),
    CNKIError(ErrInfo),
}
impl fmt::Display for TransError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransError::RequestError(e) => {
                write!(f, "【TRANSLATION】 RequestError : {}\n", e.to_string())
            }
            TransError::CNKIError(e) => {
                write!(f, "【TRANSLATION】 CNKIError : {}\n", e.to_string())
            }
        }
    }
}
