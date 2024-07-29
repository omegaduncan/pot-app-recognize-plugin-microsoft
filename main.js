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
            for (const region of regions) {
                const { lines } = region;
                for (const line of lines) {
                    const { words } = line;
                    let lineText = "";
                    let prevWordIsChinese = false;
                    for (let i = 0; i < words.length; i++) {
                        const currentWord = words[i].text;
                        const isChineseWord = /^[\u4e00-\u9fa5]+$/.test(currentWord);
                        
                        if (i > 0) {
                            if (isChineseWord && prevWordIsChinese) {
                                // 如果當前詞和前一個詞都是中文，不添加空格
                                lineText += currentWord;
                            } else {
                                // 其他情況下添加空格
                                lineText += " " + currentWord;
                            }
                        } else {
                            lineText += currentWord;
                        }
                        
                        prevWordIsChinese = isChineseWord;
                    }
                    text += lineText + "\n";
                }
            }
            return text.trim();
        } else {
            throw JSON.stringify(result);
        }
    } else {
        throw `Http Request Error\nHttp Status: ${res.status}\n${JSON.stringify(res.data)}`;
    }
}