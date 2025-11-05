const CFG = path self agent.toml

export def --env init [] {
    {
        AGENT__PROVIDER__QWEN__API_KEY: (asn --all | get api_key)
    }
    | load-env
}

export def run [] {
    init
    cargo run
}

export def 'surrealdb up' [] {
    let img = di | where name =~ surreal | get 0.name
    dcr surrealdb
    let dd = $"($env.PWD)/data/surrealdb"
    let cfg = open $CFG | get database.surreal
    mut args = [
        -d --name surrealdb
        -v $"($dd):/var/lib/surrealdb"
        -p $"($cfg.port):8000"
        -e $"SURREAL_EXPERIMENTAL_GRAPHQL=true"
        -e $"SURREAL_STORE=rocksdb"
        -e $"SURREAL_EXPERIMENTAL_GRAPHQL='true'"
        -e $"SURREAL_ROCKSDB_BACKGROUND_FLUSH='true'"
        -e $"SURREAL_USER=($cfg.user)"
        -e $"SURREAL_PASS=($cfg.pass)"
        $img
    ]
    ^$env.CNTRCTL run ...$args
}
