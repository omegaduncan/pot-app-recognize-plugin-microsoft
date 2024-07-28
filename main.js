async function recognize(base64, lang, options) {
    const { config, utils } = options;
    const { http, readBinaryFile, cacheDir } = utils;
    const { fetch, Body, } = http;
    let { subscription_key: subKey, endpoint } = config;

    if (subKey === undefined || subKey.length === 0) {
        throw "subscription key not found";
    }
    if (endpoint === undefined || endpoint.length === 0) {
        throw "endpoint not found"
    }

    if (!endpoint.startsWith("http")) {
        endpoint = "https://" + endpoint;
    }
    const data = await readBinaryFile(`${cacheDir}/pot_screenshot_cut.png`);

    let res = await fetch(`${endpoint}/vision/v3.2/ocr`, {
        method: "POST",
        headers: {
            "Ocp-Apim-Subscription-Key": subKey,
            "Content-Type": "application/octet-stream"
        },
        query: {
            language: lang,
            detectOrientation: "true",
            "model-version": "latest"
        },
        body: Body.bytes(data)
    })

        if (res.ok) {
            const result = res.data;
            const { regions } = result;
            let text = "";
            if (regions) {
                // 定義一個函數來判斷是否需要保留空格
                const shouldKeepSpaces = (lang) => {
                    // 這裡列出需要保留空格的語言代碼
                    const spaceLanguages = ['en', 'fr', 'de', 'es', 'it', 'pt', 'ru'];
                    return spaceLanguages.some(l => lang.startsWith(l));
                };
    
                const keepSpaces = shouldKeepSpaces(lang);
    
                for (const region of regions) {
                    const { lines } = region;
                    for (const line of lines) {
                        const { words } = line;
                        let lineText = words.map(word => word.text).join('');
                        
                        if (keepSpaces) {
                            // 對於需要空格的語言，我們重新添加適當的空格
                            lineText = lineText.replace(/(\S)(\S)/g, '$1 $2').trim();
                        }
                        
                        text += lineText + "\n";
                    }
                }
                
                // 移除多餘的空行並修剪首尾空白
                text = text.replace(/\n+/g, '\n').trim();
                return text;
            } else {
                throw JSON.stringify(result);
            }
        } else {
            throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
        }
    }