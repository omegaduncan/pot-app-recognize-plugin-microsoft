use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[no_mangle]
pub fn recognize(
    base64: &str,
    lang: &str,
    needs: HashMap<String, String>,
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

    let detect_orientation = true;
    let model_version = "latest";

    let url = format!(
        "{}/vision/v3.2/ocr?language={}&detectOrientation={}&model-version={}",
        endpoint, lang, detect_orientation, model_version
    );

    let response = client
        .post(&url)
        .header("Ocp-Apim-Subscription-Key", &subscription_key)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"url":"data:image/png;base64,{}"}}"#, base64))
        .send()?;

    let status = response.status();
    if !status.is_success() {
        return Err(format!("HTTP error: {}", status).into());
    }

    let res: Value = response.json()?;

    fn extract_text(res: &Value) -> String {
        let mut text = String::new();
        if let Some(regions) = res.get("regions").and_then(|v| v.as_array()) {
            for region in regions {
                if let Some(lines) = region.get("lines").and_then(|v| v.as_array()) {
                    for line in lines {
                        if let Some(words) = line.get("words").and_then(|v| v.as_array()) {
                            for word in words {
                                if let Some(word_text) = word.get("text").and_then(|v| v.as_str()) {
                                    text.push_str(word_text);
                                }
                            }
                            text.push(' ');
                        }
                        text.push('\n');
                    }
                }
            }
        }
        text.trim().to_string()
    }

    let extracted_text = extract_text(&res);
    Ok(Value::String(extracted_text))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_request() {
        let mut needs = HashMap::new();
        needs.insert("subscription_key".to_string(), "YOUR_SUBSCRIPTION_KEY".to_string());
        needs.insert("endpoint".to_string(), "https://YOUR_RESOURCE_NAME.cognitiveservices.azure.com".to_string());
        let result = recognize("iVBORw0KGgoAAAANSUhEUgAAADsAAAAeCAYAAACSRGY2AAAAAXNSR0IArs4c6QAAArNJREFUWEftl19IU1Ecxz+O5uQiNTCJkNj0ZWhkSOyh7CEy0CWZQQoTWYgvk17KFAdr9GBBYGb/qD0oUpgSCZViGkTRQ/hwEVOYIIhlMF8kUjbGZGPFdGtrGvcWzTa79/Gec+79fb7fc36/38nQ6/Xf+E+eDAV2mzqdns6WtDNRqYP5UQ71D8i2RoGVLdW/mqg4K6287G3sqHtEdYEP8clrdpZXYdCCxzWE/dkHjp5poXa/AMEVZodvU+ea2/Dn0n2NnK8wYsgVQAWEAng+TfHiZTddy75NI83LtdBRfSS2xruIONKNNftccs9sFPbLkpqcXUCmei1At2uO3YU6CKnR7AhDLDJ204bdH4u/tKSdjkodmvCrEKz6A2iE9fWEVhAftmF1JwBnmxm0msjPinzHH2A1U42GFcSJZYzGJCaodVhYnRqgZngUCmw8rStC419gzOnA7iuio8HG8b3wccTC2clIkFkWhppPkKcK4H7bTev7cWbDQ5kHcZxqorpQAO8M929dp+eHPgJtNXepNajh6wx9j+9E3BeoONBCc7mOnCx18rJxFDYGYmbwson85Sm67nXSB9SXO7loFPCIDzj2anwtdOPhTpxlueB+h7W3BzF+w6pM9F8wYxACTPc30jAfHTTR22ymeMP78HicEMkqPX8Ku5kAMV6Ba/VOKvQJu4GIkCzx5sYlWuOOxE8CphcsbBQxjBOFXeD5VQftiekr2aUnOc4qsNvV2W12ZuVlYx9irxWrO82zMXLqbFz5WseVqLNlOnKyU7DOhkP/qx2Uysf05BLFJVvQQf1uUxHdmIY9Fq5UxfW5wQCezxK9sbYKx+mTGPMi/fRW9cbSd4rUnyH71pP6KNIRKrDSGqXnDMXZ9PRNOmrF2USNtFotXq+XYDAoLV8Kz5DlrAKbwg7+KrTvuhRWXxXeDuUAAAAASUVORK5CYII=", "zh-Hant", needs).unwrap();
        println!("{result}");
    }
}