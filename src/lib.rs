use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use flate2::read::MultiGzDecoder;
 
// const EXAMPLE: &str = "./src/test.fa.gz";
const EXAMPLE: &str = "/Users/roderick/Downloads/472Mb_test.gz";
// this helps keep the runtime within criterion's wheelhouse and also helps
// weigh the balance between initial load time and the actual gziping
const N_CHUNKS: usize = 100;
// the size of the buffer to read once we're in the hot loop
const BUF_SIZE: usize = 10000;

fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("gzip detection");

    group.measurement_time(Duration::new(20, 0));
    group.bench_function("wrap with bufread", |b| b.iter(|| {
        let f = File::open(EXAMPLE).unwrap();

        let mut reader = io::BufReader::new(f);
        let peek = reader.fill_buf().unwrap();
        let first = &peek[0..4];
        black_box(&first);

        let mut amt_read = 1;
        let mut gz_reader = MultiGzDecoder::new(reader);

        let mut n_passes = 0;
        while amt_read > 0 && n_passes < N_CHUNKS {
            n_passes += 1;
            let mut data = vec![0; BUF_SIZE];
            amt_read = gz_reader.read(&mut data).unwrap();
            black_box(data);
        }
    }));

    group.bench_function("chain iterators", |b| b.iter(|| {
        let mut f = File::open(EXAMPLE).unwrap();

        let mut first = vec![0; 4];
        let mut amt_read = f.read(&mut first).unwrap();
        black_box(&first);

        let cursor = io::Cursor::new(first);
        let mut gz_reader = MultiGzDecoder::new(cursor.chain(f));

        let mut n_passes = 0;
        while amt_read > 0 && n_passes < N_CHUNKS {
            n_passes += 1;
            let mut data = vec![0; BUF_SIZE];
            amt_read = gz_reader.read(&mut data).unwrap();
            black_box(data);
        }
    }));

    group.bench_function("use directly", |b| b.iter(|| {
        let f = File::open(EXAMPLE).unwrap();

        let mut amt_read = 1;
        let mut gz_reader = MultiGzDecoder::new(f);

        let mut n_passes = 0;
        while amt_read > 0 && n_passes < N_CHUNKS {
            n_passes += 1;
            let mut data = vec![0; BUF_SIZE];
            amt_read = gz_reader.read(&mut data).unwrap();
            black_box(data);
        }
    }));

    group.finish();
}


criterion_group!(benches, bench);
criterion_main!(benches);
