# Pot-App 文字識別 Microsoft 插件

### [English](./README_EN.md) | 简体中文

## 支持平台
- Windows x64
  - [下載鏈接](https://github.com/omegaduncan/pot-app-recognize-plugin-microsoft/releases/download/v0.0.13/x86_64-pc-windows-msvc.zip)

## 使用方法
1. 下載對應平台的插件壓縮包。
2. 解壓縮得到 `.potext` 文件。
3. 打開 Pot-App，進入偏好設置，選擇服務設置，然後點擊文字辨識。
4. 添加外部插件，選擇安裝外部插件。
5. 選擇剛剛解壓縮得到的 `.potext` 文件，完成安裝。
6. 將插件添加到服務列表中，再設置 `Subscription Key`(API KEY)、 `Endpoint`(端點，https://{resourceName}.cognitiveservices.azure.com/)。

## 其他說明
- **API 密鑰申請**：使用本插件前，您需要申請 Microsoft API 密鑰。申請教程請參考 [這裡](https://learn.microsoft.com/en-us/azure/ai-services/computer-vision/quickstarts-sdk/client-library?tabs=windows%2Cvisual-studio&pivots=programming-language-csharp)。
- **免費定價層**：每月提供 5000 次識別，每分鐘可用 20 次。

## 次要說明
本插件由 Claude Opus 及 ChatGPT 4 Turbo 合作修改自文字識別插件的模板倉庫 (OCR Space)。由於可能存在一些後處理的 bug，導致準確度較低，我們將在後續版本中逐步修復這些問題。