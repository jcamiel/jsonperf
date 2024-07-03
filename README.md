# Perf Analysis JSON Parser

JSON parsing analysis:

- `serde-json`: 1.0.119
- `hurl_core`: 4.3.0

## How to run

```
jsonperf [parse-serde|parse-hurl|read-hurl|read-bytes] FILE
```

### Usage

```shell
$ gzip --decompress --stdout large.json.gz > large.json
$ cargo build --release
$ # Run Serde JSON parsing
$ time ./target/release/jsonperf parse-serde large.json
11341
./target/release/jsonperf parse-serde large.json  0.09s user 0.02s system 92% cpu 0.120 total
$ # Run Hurl JSON parsing
$ time ./target/release/jsonperf parse-hurl large.json
11341
./target/release/jsonperf parse-hurl large.json  2.01s user 0.27s system 94% cpu 2.406 total
$ # Run Hurl read (unicode char by unicode char)
$ time ./target/release/jsonperf read-hurl large.json
true
./target/release/jsonperf read-hurl large.json  0.08s user 0.02s system 82% cpu 0.123 total
$ # Run Hurl read (byte by byte)
$ time ./target/release/jsonperf read-bytes large.json
true
./target/release/jsonperf read-bytes large.json  0.00s user 0.01s system 71% cpu 0.010 total
```


## Results

### M1 Pro 4-inch, 2021

| serde-json 1.0.119 | hurl_core (JSON) 4.3.0 | hurl_core (char) 4.3.0 | hurl_core (bytes) 4.3.0 |
|--------------------|------------------------|------------------------|-------------------------|
| 0.120              | 2.406                  | 0.123                  | 0.010                   |
