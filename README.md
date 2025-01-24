Inspired by
[libchata](https://github.com/Slackadays/Chata/blob/main/libchata/src/generate_instruction_search.py)'s
generated code for string lookup on RISC-V instructions. The python script to
generate the rust code is copied from there
([generate_instruction_search.py](https://github.com/Slackadays/Chata/blob/main/libchata/src/generate_instruction_search.py)).

---

# Summary

Below are some probably terrible benchmarks taken on my computer of string
lookups to compare the code generated by [gen.py](./gen.py) and some map
implementations.

|     | Lower bound | Estimate | Upper bound |
| --- | ----------- | -------- | ----------- |
| [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) | 23.337 ns | 23.365 ns | 23.408 ns |
| [BTreeMap](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html) | 77.149 ns | 77.233 ns | 77.354 ns |
| [FxHashMap](https://docs.rs/rustc-hash/latest/rustc_hash/type.FxHashMap.html) | **10.854 ns** | **10.864 ns** | **10.876 ns** |
| [AHashMap](https://docs.rs/ahash/latest/ahash/struct.AHashMap.html) | 12.862 ns | 12.872 ns | 12.886 ns |
| [generated](./src/generated.rs) | 35.405 ns | 35.444 ns | 35.495 ns |

The full report by [criterion](https://docs.rs/criterion/latest/criterion/index.html) is in [results](./results).

# How to run

You just need rust and cargo. I used rustc version 1.81.0:

```sh
rustup show
# ...
# active toolchain
# ----------------
# 
# stable-x86_64-unknown-linux-gnu (default)
# rustc 1.81.0 (eeb90cda1 2024-09-04)
```

Clone the repo and run the benchmarks with cargo:

```sh
git clone https://github.com/ned-gh/instr_search
cd instr_search
cargo bench
```

You can then look at the results generated by criterion in `./target/criterion`.
