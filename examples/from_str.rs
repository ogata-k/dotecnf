use dotecnf::parser::ECnfParser;

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
    let mut parser = ECnfParser::new();
    parser.load_from_str(input);

    println!("result:");
    let ecnf = parser.build_ecnf();
    let mut keys: Vec<String> = ecnf.keys().map(|s| s.to_string()).collect();
    keys.sort();
    for k in keys {
        println!("\t{}\t:\t{:?}", k, ecnf.get(&k).unwrap());
    }
}