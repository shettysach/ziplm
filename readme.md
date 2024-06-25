## ziplm.rs

[Futrell's ziplm](https://github.com/Futrell/ziplm), CLI and Rust library implementation. WIP.

### CLI Instructions

#### Install
```bash
cargo install --git https://github.com/ShettySach/ziplm --features cli
```

#### Usage
```
Usage: ziplm [OPTIONS]

Options:
  -l, --length <LENGTH>  Maximum length of sample generated [default: 100]
  -p, --prefix <PREFIX>  Prefix for sample generated [default: ]
  -t, --temp <TEMP>      Temperature for sample generated [default: 0.25]
  -v, --vocab <VOCAB>    Path of vocabulary data (.txt file) [Defaults to qwertyuiopasdfghjklzxcvbnm,. '"]
  -d, --data <DATA>      Path of training data (.txt file) [Defaults to Mary Shelley's Frankenstein]
  -c, --conv <COMP>      Conversion factor [default: 5.545177444479562]
  -h, --help             Print help
  -V, --version          Print version
```

### Library Instructions

#### Install
```bash
cargo add --git https://github.com/ShettySach/ziplm
```

#### Usage
```Rust
use ziplm::ZipModel;

fn main() {
    let vocab = "qwertyuiopasdfghjklzxcvbnm,. '";
    let data = include_str!("../data.txt");
    let ln256 = 5.545177444479562;

    let model = ZipModel::new(vocab, data, ln256);
    let _output = model.sample_sequence(100, "arch", 0.25);
}
```

### Credits
- [Original ziplm repo by Futrell](https://github.com/Futrell/ziplm)
- [Frankenstein; Or, The Modern Prometheus by Mary Wollstonecraft Shelley](https://www.gutenberg.org/cache/epub/84/pg84.txt)
