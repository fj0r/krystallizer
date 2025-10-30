export def --env init [] {
    {
        AGENT_PROVIDER_QWEN_API_KEY: (asn --all | get api_key)
    }
    | load-env
}
