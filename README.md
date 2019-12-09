To transparently open a `Read` stream with gzip compression, you need to
detect if the stream itself is gzip compressed before starting compression.
Unfortunately, this requires reading from the stream itself to see if the gz
magic bytes are present and then re-inserting those bytes in the stream so
that the `GzDecoder` doesn't blow up.

It's also possible to do this in a "zero overhead" fashion if the reader
implements `Seek`, but some readers (e.g. stdin) don't.

This is a little dummy comparison to check the overhead induced by wrapping
the reader in different ways to fool `GzDecoder`. There's a smaller gz file
in the repo to test with, but for a more-real world test I used a large file
I had lying around. I also checked a couple `BUF_SIZE` and `N_CHUNKS`
parameters, but nothing appeared to separate the performances much.


Results:
```
warning: file found to be present in multiple build targets: /Users/roderick/Documents/inline_gzip/src/lib.rs
    Finished bench [optimized] target(s) in 0.04s
     Running target/release/deps/inline_gzip-2731615059d3ca86

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

     Running target/release/deps/benches-18e8e62b4d3154e3
Gnuplot not found, disabling plotting
gzip detection/wrap with bufread
                        time:   [2.8165 ms 2.8265 ms 2.8370 ms]
                        change: [-2.1421% -0.7243% +0.6734%] (p = 0.38 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe
gzip detection/chain iterators
                        time:   [2.8123 ms 2.8204 ms 2.8297 ms]
                        change: [-1.2599% +0.1817% +1.4781%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe
gzip detection/use directly
                        time:   [2.8228 ms 2.8341 ms 2.8457 ms]
                        change: [-1.3466% -0.1031% +1.3339%] (p = 0.89 > 0.05)
                        No change in performance detected.
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

Gnuplot not found, disabling plotting
```
