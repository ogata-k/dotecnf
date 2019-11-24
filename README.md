# dotecnf
## What is .ecnf
".ecnf" is "Environment CoNFig" file such as ".env" file.
.env is simple key-value store. So .enc cannot treat "key = {key = value}" and "key="(value is null). It is not useful. 
.ecnf is my original formatting file. .ecnf can treat them which .env cannot treat.
.ecnf has following format.

```
# comment
# null (None)
ONLY_UPPER_START :

# empty string
EMPTY: ""

# not empty string
NOT_EMPTY :"value"

# fail
# sTart: 
# _START :""

ZERO : {
    FIRST : ""
    SECOND: {
        THIRD: "日本語"
    }
    FOURTH :
}

SUCCESS: {
}

# ERROR: {}
``` 

## usage

``` rust:example.rs
    // init
    let mut loader = ECnfLoader::new();
    
     // load: self.from<R: Read>(R), self.load_from_file(&Path), self.load_from_str(&str))
    loader.load_from_str(input);
     
    let ecnf: HashMap<String, Option<String>> = loader.build_ecnf();
    // use ecnf...
```