 这是一个翻译软件包。默认主要用于英文=>中文 的医学文档翻译

 现阶段一共封装了[CNKI API](https://dict.cnki.net/)和[百度翻译API](https://fanyi-api.baidu.com/)，
 CNKI为逆向的其网页版，*不稳定*，百度使用的官方API，需要注册APIID，*稳定*,使用量大需付费

 # Example

 ### CNKI
 ```rust
 let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";

 let check_res = r#"患者男，55岁，因"家族性腺瘤性息肉病( FAP ) "背景下十二指肠异型增生性息肉就诊。患者既往有结肠次全切除回直肠吻合术史，1981年形成回肛袋，2000年(早了20多年)行造口还纳术。"#;

 match Translate::Cnki.words(words).trans() {
     Ok(tr_res) => {
         println!("{}\n{:?}", words, tr_res);
         assert_eq!(tr_res, check_res.to_owned())
     }
     Err(e) => println!("Err:{}", e),
 }
 ```

 只支持中英互译，无需调整参数，和网页使用效果相同。但是若网站调整加密算法或者接口，将失效

 最大支持1200英文字符，600中文。翻译专业文档和术语相对最准确

 #### 修改翻译类型
 ```rust
 let mut tr = Translate::Cnki.words(words);
 tr.set_from(Language::Zh); //修改为中 => 英
 match tr.trans() {
     Ok(tr_res) => {
         println!("{}\n{:?}", words, tr_res);
         assert_eq!(tr_res, check_res.to_owned())
     }
     Err(e) => println!("Err:{}", e),
 }
 ```

 ### Baidu API
 ```rust
 let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";

 let check_res = r#"一名55岁的男性在家族性腺瘤性息肉病（FAP）的背景下诊断为十二指肠息肉增生后进行术前评估。该患者曾于1981年接受结肠次全切除术，回肠直肠吻合术并形成回肠肛囊，随后于2000年（20多年前）造口逆转。"#;
 // 此处可使用环境变量一类的
 match Translate::Baidu(("BAIDU_APIID", "BAIDU_SECRT"))
     .words(words)
     .trans()
 {
     Ok(tr_res) => {
         println!("{:?}\n{:?}", words, tr_res);
         assert_eq!(tr_res, check_res.to_owned())
     }
     Err(e) => println!("Err:{}", e),
 }
  // BAIDU_APIID & BAIDU_SECRT 请自行前往官网注册。 垂直领域，每月50万免费字符,超出后仅收取超出部分费用，49元/百万字符
 ```

 支持多种翻译。默认英文=>中文。

 #### 修改翻译类型
 ```rust
 let mut tr = Translate::Baidu(("BAIDU_APIID", "BAIDU_SECRT")).words(words);
 tr.set_from(Language::Zh); // Language::Auto 系统自动识别语言类型
 tr.set_to(Language::Jp);
 match tr.trans() {
     Ok(tr_res) => {
         println!("{}\n{:?}", words, tr_res);
         assert_eq!(tr_res, check_res.to_owned())
     }
     Err(e) => println!("Err:{}", e),
 }
 ```
 使用垂直行业接口,若使用其他行业，或通用领域，请自行更换接口API