const CFG = path self agent.toml

export def --env setup [] {
    {
        AGENT__PROVIDER__QWEN__API_KEY: (asn --all | get api_key)
    }
    | load-env
}

export def run [] {
    setup
    cargo run --bin krystallizer
}

export def init [] {
    setup
    cargo run --bin init
}

export def 'surreal up' [] {
    let cfg = open $CFG | get database.surreal
    const data = path self data/surrealdb
    $env.SURREAL_STORE = "rocksdb"
    $env.SURREAL_ROCKSDB_BACKGROUND_FLUSH = "true"
    mut args = [
        start
        -A
        --no-banner
        -b 0.0.0.0:9900
        -u $cfg.user
        -p $cfg.pass
        $"rocksdb://($data)"
    ]
    surreal2 ...$args
}

export def 'surrealdb up' [] {
    let img = di | where name =~ surreal | get 0.name
    dcr surrealdb
    let dd = $"($env.PWD)/data/surrealdb"
    let cfg = open $CFG | get database.surreal
    mut args = [
        -d --name surrealdb
        -v $"($dd):/var/lib/surrealdb"
        -p $"($cfg.port):9900"
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
