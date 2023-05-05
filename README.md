这是一个翻译软件包。默认主要用于英文=>中文 的医学文档翻译

现阶段一共封装了[CNKI API](https://dict.cnki.net/)和[百度翻译API](https://fanyi-api.baidu.com/)，
CNKI为逆向的其网页版*不稳定*，百度使用的官方API，需要APIID*稳定*,使用量大需付费

# Example

### CNKI 单句翻译
```rust
let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";

let check_res = r#"患者男，55岁，因"家族性腺瘤性息肉病( FAP ) "背景下十二指肠异型增生性息肉就诊。患者既往有结肠次全切除回直肠吻合术史，1981年形成回肛袋，2000年(早了20多年)行造口还纳术。"#;
let mut cnki = Translate::Cnki.create();
//此值是反编译JS得到,默认值是此。若变化，自行修改时可传递
//cnki.set_secret_key("4e87183cfd3a45fe");
//修改为中 => 英. 默认为英 => 中
//cnki.set_from(Language::Zh);
cnki.set_words(words);
match cnki.trans() {
Ok(tr_res) => {
println!("{}\n{:?}", words, tr_res);
assert_eq!(tr_res, check_res.to_owned())
}
Err(e) => println!("Err:{}", e),
}

```

只支持中英互译，无需调整参数，和网页使用效果相同。但是若网站调整加密算法或者接口，将失效

最大支持1200英文字符，600中文。翻译专业文档和术语相对最准确

### CNKI 文档翻译
```no run
let docs = "Left masticator space shows an altered signal intensity heterogeneous expansile soft tissue mass, showing iso-intense signal intensity on T1W, heterogeneously hyperintense to intermediate signal intensity on T2W lesion with low signal striations, seen arising from the posterior body and ramus of the mandible. The lesion shows diffusion restriction and heterogeneous enhancement on post-contrast images.
The lesion extends from the anterior to the posterior border of the ramus of the mandible with thinning of the ramus of the mandible.  The lesion is infiltrating the left masseter muscle laterally and medial pterygoid muscle medially. The left parotid gland is seen compressed along the inferolateral margin of the lesion and is medially displacing the left parapharyngeal space towards the midline without infiltrating it.
Ultrasound-guided core needle biopsy with needle tip within the lesion showing reverberation artifact.Histopathology reports suggest a poorly differentiated malignancy.
Fluorescence in situ hybridization (FISH) is positive for EWSR1 (Ewing sarcoma breakpoint region 1) gene re-arrangement.
The immunohistochemistry (IHC) panel shows the tumor is positive for CK (moderately), NKX 2.2, and CD99, which are features of Ewing sarcoma.";
let mut cnki = Translate::Cnki.create();
// cnki.set_secret_key("4e87183cfd3a45fe");
println!("{:?}", cnki.docs(docs));
```
文档翻译,使用简单的标点【'!', '.', '?', ';', '？', '。', '；', '！'】切分。
结果生成一个Vec<String>。

未检测max_len,如果分割之后，还有超过max_len的情况，将返回固定的" ---translation err--- "。

所有的错误情况，都在对应句子位置 返回 " ---translation err--- "。

若有更精确的需求，请自行实现，或提issue

### Baidu API
```rust
let words = "A 55-year-old man presented for preoperative assessment following diagnosis of a dysplastic duodenal polyp on a background of familial adenomatous polyposis (FAP). The patient had prior surgical history of subtotal colectomy with ileorectal anastomosis with formation of ileo-anal pouch in 1981 with subsequent stoma reversal in 2000 (more than 20 years earlier).";

let check_res = r#"一名55岁的男性在家族性腺瘤性息肉病（FAP）的背景下诊断为十二指肠息肉增生后进行术前评估。该患者曾于1981年接受结肠次全切除术，回肠直肠吻合术并形成回肠肛囊，随后于2000年（20多年前）造口逆转。"#;
let mut baidu = Translate::Baidu.create();

baidu.set_appid("xxxxxxxxxxxx"); // 必须
baidu.set_secret_key("xxxxxxxxxxxx"); // 必须
baidu.set_from(Language::Auto); // default:Language::Auto
baidu.set_to(Language::Zh); // default:Language::Zh
baidu.set_domain(DomainType::Medicine); // default:DomainType::Medicine
baidu.set_words(words);

match baidu.trans() {
Ok(tr_res) => {
println!("{:?}\n{:?}", words, tr_res);
assert_eq!(tr_res, check_res.to_owned())
}
Err(e) => println!("Err:{}", e),
}
// BAIDU_APPID & BAIDU_secret_key 请自行前往官网注册。 垂直领域，每月50万免费字符,超出后仅收取超出部分费用，49元/百万字符
```

支持多种翻译。默认英文=>中文。

#### Baidu文档类型
```rust
let docs = "Left masticator space shows an altered signal intensity heterogeneous expansile soft tissue mass, showing iso-intense signal intensity on T1W, heterogeneously hyperintense to intermediate signal intensity on T2W lesion with low signal striations, seen arising from the posterior body and ramus of the mandible. The lesion shows diffusion restriction and heterogeneous enhancement on post-contrast images.
The lesion extends from the anterior to the posterior border of the ramus of the mandible with thinning of the ramus of the mandible.  The lesion is infiltrating the left masseter muscle laterally and medial pterygoid muscle medially. The left parotid gland is seen compressed along the inferolateral margin of the lesion and is medially displacing the left parapharyngeal space towards the midline without infiltrating it.
Ultrasound-guided core needle biopsy with needle tip within the lesion showing reverberation artifact.Histopathology reports suggest a poorly differentiated malignancy.
Fluorescence in situ hybridization (FISH) is positive for EWSR1 (Ewing sarcoma breakpoint region 1) gene re-arrangement.
The immunohistochemistry (IHC) panel shows the tumor is positive for CK (moderately), NKX 2.2, and CD99, which are features of Ewing sarcoma.";
let mut baidu = Translate::Baidu.create();
baidu.set_appid("xxxxxxxxxxxx");
baidu.set_secret_key("xxxxxxxxxxxxxx");
baidu.set_from(Language::En);
baidu.set_to(Language::Zh);
baidu.set_domain(DomainType::Medicine);
println!("{:?}", baidu.docs(docs));
```
使用垂直行业接口,若使用其他行业，或通用领域，请自行更换接口API

 ### TUDO
 - [ ] Xunfei,
 - [ ] Youdao,
 - [ ] Deepl,
 - [ ] Ali,
 - [ ] Tencent,
 - [ ] 多尝试逆向免费版