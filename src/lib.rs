use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[no_mangle]
pub fn recognize(
    image_path: &str,
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

    let mut file = File::open(Path::new(image_path))?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let base64_image = base64::encode(&buffer);

    let response = client
        .post(&url)
        .header("Ocp-Apim-Subscription-Key", &subscription_key)
        .header("Content-Type", "application/json")
        .body(format!(r#"{{"url":"data:image/png;base64,{}"}}"#, base64_image))
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
        let image_path = "C:\\Users\\{用户名}\\AppData\\Local\\com.pot-app.desktop\\pot_screenshot_cut.png";
        let result = recognize(image_path, "zh-Hant", needs).unwrap();
        println!("{result}");
    }
}