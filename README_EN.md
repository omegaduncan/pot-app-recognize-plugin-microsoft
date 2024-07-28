# Pot-App Microsoft OCR Plugin

### English | [繁體中文](./README.md)

## Usage Instructions
1. Go to [Releases](https://github.com/omegaduncan/pot-app-recognize-plugin-microsoft/releases) to download the `plugin.com.omegaduncan.microsoft.potext` file.
2. Open Pot-App, go to Preferences, select Service Settings, then click on Text Recognition.
3. Add an external plugin by selecting "Install External Plugin".
4. Choose the recently downloaded `plugin.com.omegaduncan.microsoft.potext` file to complete the installation.
5. Add the plugin to the service list, then set the `Subscription Key` (API KEY) and `Endpoint` (https://{resourceName}.cognitiveservices.azure.com/).

## Additional Information
- **API Key Application**: Before using this plugin, you need to apply for a Microsoft API key. For application tutorial, please refer to [here](https://learn.microsoft.com/en-us/azure/ai-services/computer-vision/quickstarts-sdk/client-library?tabs=windows%2Cvisual-studio&pivots=programming-language-csharp).
- **Free Pricing Tier**: Provides 5000 recognitions per month, with 20 uses available per minute.

## Secondary Notes
This plugin is a collaboration modified by Claude Opus, ChatGPT 4 Turbo, and Claude 3.5 Sonnet (added in v1.1.0) from the text recognition plugin template repository (OCR Space). Due to possible post-processing bugs that may result in lower accuracy, we will gradually address these issues in subsequent versions.