use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn recognize(
    base64: &str, // 图像Base64
    _lang: &str,  // 识别语言
    // (pot会根据info.json 中的 language 字段传入插件需要的语言代码,无需再次转换)
    needs: HashMap<String, String>, // 插件需要的其他参数,由info.json定义
) -> Result<Value, Box<dyn Error>> {
    let client = reqwest::blocking::ClientBuilder::new().build()?;

    let subscription_key = match needs.get("subscription_key") {
        Some(key) => key.to_string(),
        None => return Err("subscription_key not found".into()),
    };

    let endpoint = match needs.get("endpoint") {
        Some(endpoint) => endpoint.to_string(),
        None => return Err("endpoint not found".into()),
    };

    let url = format!("{}/vision/v3.2/read/analyze", endpoint);

    let response = client
        .post(&url)
        .header("Ocp-Apim-Subscription-Key", &subscription_key)
        .header("Content-Type", "application/octet-stream")
        .body(base64::decode(base64).map_err(|e| format!("Failed to decode base64: {}", e))?)
        .send()?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!("HTTP error: {}", status).into());
    }

    let res: Value = response.json()?;

    let operation_location = res
        .as_object()
        .and_then(|obj| obj.get("Operation-Location"))
        .and_then(|location| location.as_str())
        .ok_or("Failed to get Operation-Location")?;

    let mut recognize_result: Option<Value> = None;

    for _ in 0..30 {
        let response = client
            .get(operation_location)
            .header("Ocp-Apim-Subscription-Key", &subscription_key)
            .send()?;

        let status = response.status();
        if !status.is_success() {
            return Err(format!("HTTP error: {}", status).into());
        }

        let res: Value = response.json()?;

        if res["status"] == "succeeded" {
            recognize_result = Some(res["analyzeResult"].clone());
            break;
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    match recognize_result {
        Some(result) => Ok(result),
        None => Err("Failed to get recognize result".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("subscription_key".to_string(), "YOUR_SUBSCRIPTION_KEY".to_string());
        needs.insert("endpoint".to_string(), "https://YOUR_RESOURCE_NAME.cognitiveservices.azure.com/".to_string());
        let result = recognize("iVBORw0KGgoAAAANSUhEUgAAADsAAAAeCAYAAACSRGY2AAAAAXNSR0IArs4c6QAAArNJREFUWEftl19IU1Ecxz+O5uQiNTCJkNj0ZWhkSOyh7CEy0CWZQQoTWYgvk17KFAdr9GBBYGb/qD0oUpgSCZViGkTRQ/hwEVOYIIhlMF8kUjbGZGPFdGtrGvcWzTa79/Gec+79fb7fc36/38nQ6/Xf+E+eDAV2mzqdns6WtDNRqYP5UQ71D8i2RoGVLdW/mqg4K6287G3sqHtEdYEP8clrdpZXYdCCxzWE/dkHjp5poXa/AMEVZodvU+ea2/Dn0n2NnK8wYsgVQAWEAng+TfHiZTddy75NI83LtdBRfSS2xruIONKNNftccs9sFPbLkpqcXUCmei1At2uO3YU6CKnR7AhDLDJ204bdH4u/tKSdjkodmvCrEKz6A2iE9fWEVhAftmF1JwBnmxm0msjPinzHH2A1U42GFcSJZYzGJCaodVhYnRqgZngUCmw8rStC419gzOnA7iuio8HG8b3wccTC2clIkFkWhppPkKcK4H7bTev7cWbDQ5kHcZxqorpQAO8M929dp+eHPgJtNXepNajh6wx9j+9E3BeoONBCc7mOnCx18rJxFDYGYmbwson85Sm67nXSB9SXO7loFPCIDzj2anwtdOPhTpxlueB+h7W3BzF+w6pM9F8wYxACTPc30jAfHTTR22ymeMP78HicEMkqPX8Ku5kAMV6Ba/VOKvQJu4GIkCzx5sYlWuOOxE8CphcsbBQxjBOFXeD5VQftiekr2aUnOc4qsNvV2W12ZuVlYx9irxWrO82zMXLqbFz5WseVqLNlOnKyU7DOhkP/qx2Uysf05BLFJVvQQf1uUxHdmIY9Fq5UxfW5wQCezxK9sbYKx+mTGPMi/fRW9cbSd4rUnyH71pP6KNIRKrDSGqXnDMXZ9PRNOmrF2USNtFotXq+XYDAoLV8Kz5DlrAKbwg7+KrTvuhRWXxXeDuUAAAAASUVORK5CYII=", "eng", needs).unwrap();
        println!("{result}");
    }
}