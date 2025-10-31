export def --env init [] {
    {
        AGENT__PROVIDER__QWEN__API_KEY: (asn --all | get api_key)
    }
    | load-env
}
