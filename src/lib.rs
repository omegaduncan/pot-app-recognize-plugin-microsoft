use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn recognize(
    base64: &str, // 图像Base64
    lang: &str,   // 识别语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码，无需再次转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let subscription_key = match needs.get("subscription_key") {
        Some(key) => key.to_string(),
        None => return Err("subscription_key not found".into()),
    };

    let base64 = format!("data:image/png;base64,{}", base64);
    let mut form_data = HashMap::new();
    form_data.insert("url", base64);
    form_data.insert("language", lang.to_string());

    let res: Value = client
        .post("https://pandaocr-free.cognitiveservices.azure.com/vision/v3.2/read/analyze")
        .header("Ocp-Apim-Subscription-Key", subscription_key)
        .header("Content-Type", "application/json")
        .json(&form_data)
        .send()?
        .json()?;

    fn parse_result(res: Value) -> Option<Result<Value, Box<dyn Error>>> {
        println!("{res:?}");
        if let Some(error) = res.as_object()?.get("error") {
            return Some(Err(error.to_string().into()));
        }
        let result_url = res.as_object()?.get("analyzeResult")?.as_object()?.get("readResults")?.as_str()?;
        
        let mut result = String::new();
        let res: Value = client.get(result_url).send()?.json()?;
        let lines = res.as_array()?;
        for line in lines {
            let text = line.as_object()?.get("text")?.as_str()?;
            result.push_str(text);
            result.push('\n');
        }
        Some(Ok(Value::String(result)))
    }

    if let Some(result) = parse_result(res) {
        return result;
    } else {
        return Err("Response Parse Error".into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("subscription_key".to_string(), "YOUR_SUBSCRIPTION_KEY".to_string());
        let result = recognize("iVBORw0KGgoAAAANSUhEUgAAADsAAAAeCAYAAACSRGY2AAAAAXNSR0IArs4c6QAAArNJREFUWEftl19IU1Ecxz+O5uQiNTCJkNj0ZWhkSOyh7CEy0CWZQQoTWYgvk17KFAdr9GBBYGb/qD0oUpgSCZViGkTRQ/hwEVOYIIhlMF8kUjbGZGPFdGtrGvcWzTa79/Gec+79fb7fc36/38nQ6/Xf+E+eDAV2mzqdns6WtDNRqYP5UQ71D8i2RoGVLdW/mqg4K6287G3sqHtEdYEP8clrdpZXYdCCxzWE/dkHjp5poXa/AMEVZodvU+ea2/Dn0n2NnK8wYsgVQAWEAng+TfHiZTddy75NI83LtdBRfSS2xruIONKNNftccs9sFPbLkpqcXUCmei1At2uO3YU6CKnR7AhDLDJ204bdH4u/tKSdjkodmvCrEKz6A2iE9fWEVhAftmF1JwBnmxm0msjPinzHH2A1U42GFcSJZYzGJCaodVhYnRqgZngUCmw8rStC419gzOnA7iuio8HG8b3wccTC2clIkFkWhppPkKcK4H7bTev7cWbDQ5kHcZxqorpQAO8M929dp+eHPgJtNXepNajh6wx9j+9E3BeoONBCc7mOnCx18rJxFDYGYmbwson85Sm67nXSB9SXO7loFPCIDzj2anwtdOPhTpxlueB+h7W3BzF+w6pM9F8wYxACTPc30jAfHTTR22ymeMP78HicEMkqPX8Ku5kAMV6Ba/VOKvQJu4GIkCzx5sYlWuOOxE8CphcsbBQxjBOFXeD5VQftiekr2aUnOc4qsNvV2W12ZuVlYx9irxWrO82zMXLqbFz5WseVqLNlOnKyU7DOhkP/qx2Uysf05BLFJVvQQf1uUxHdmIY9Fq5UxfW5wQCezxK9sbYKx+mTGPMi/fRW9cbSd4rUnyH71pP6KNIRKrDSGqXnDMXZ9PRNOmrF2USNtFotXq+XYDAoLV8Kz5DlrAKbwg7+KrTvuhRWXxXeDuUAAAAASUVORK5CYII=", "eng", needs).unwrap();
        println!("{result}");
    }
}