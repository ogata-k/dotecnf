use dotecnf::ECnfLoader;
use std::path::Path;

fn main() {
    let path = Path::new("./examples/example.ecnf");
    let mut loader = ECnfLoader::new();
    loader.load_from_file(path).unwrap();

    println!("result:");
    let ecnf = loader.build_ecnf();
    let mut keys: Vec<String> = ecnf.keys().map(|s| s.to_string()).collect();
    keys.sort();
    for k in keys {
        println!("\t{}\t:\t{:?}", k, ecnf.get(&k).unwrap());
    }
}
