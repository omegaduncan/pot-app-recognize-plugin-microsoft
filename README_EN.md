# Pot-App Microsoft OCR Plugin

### English | [繁體中文](./README.md)

## Supported Platforms
- Windows x64
  - [Download Link](https://github.com/omegaduncan/pot-app-recognize-plugin-microsoft/releases/download/v0.0.13/x86_64-pc-windows-msvc.zip)

## Usage
1. Download the plugin package for your platform.
2. Extract the `.potext` file from the downloaded package.
3. Open Pot-App, go to Preferences, select Service Settings, and then Translation.
4. Add an external plugin and choose to install an external plugin.
5. Select the extracted `.potext` file to complete the installation.
6. Add the plugin to the service list, then set up `Subscription Key` (API KEY) and `Endpoint` (https://{resourceName}.cognitiveservices.azure.com/).

## Additional Notes
- **API Key Application**：Before using this plugin, you need to apply for a Microsoft API key. For the application tutorial, please refer to [here](https://learn.microsoft.com/en-us/azure/ai-services/computer-vision/quickstarts-sdk/client-library?tabs=windows%2Cvisual-studio&pivots=programming-language-csharp).
- **Free Tier**：Offers 5000 recognitions per month, with a limit of 20 per minute.

## Secondary Notes
This plugin was modified by Claude Opus and ChatGPT 4 Turbo from the template repository of the text recognition plugin (OCR Space). Due to possible post-processing bugs, the accuracy may be low, and we will gradually fix these issues in subsequent versions.