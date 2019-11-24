use dotecnf::ECnfLoader;

fn main() {
    let input: &str = r#"
        # app version
        VERSION :  "4.2.23"
        DOMAIN: "http://example.com/"

        # screen
        SCREEN:{
            # empty setting so empty string
            ZERO: "日本語"
            SC_ONE:
            SC_TWO: ""

            # value is "hoge hoge " and key is SCREEN.SC_THREE
            SC_THREE: "hoge hoge "
        }"#;
    let mut loader = ECnfLoader::new();
    loader.load_from_str(input).unwrap();

    println!("result:");
    let ecnf = loader.build_ecnf();
    let mut keys: Vec<String> = ecnf.keys().map(|s| s.to_string()).collect();
    keys.sort();
    for k in keys {
        println!("\t{}\t:\t{:?}", k, ecnf.get(&k).unwrap());
    }
}
