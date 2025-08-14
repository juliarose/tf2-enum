use criterion::{criterion_group, criterion_main, Criterion};
use tf2_enum::{ItemLevel, Level};

fn loop_levels<'a>(score: i32, levels: &'a [Level]) -> &'a Level {
    let mut prev_level = &levels[levels.len() - 1];
    
    for i in (0..levels.len()).rev() {
        if score >= levels[i].required_score {
            return prev_level;
        } else {
            prev_level = &levels[i];
        }
    }
    
    &levels[0]
}

fn binary_search<'a>(score: i32, levels: &'a [Level]) -> &'a Level {
    // First index where required_score > score
    let i = levels.partition_point(|lvl| lvl.required_score <= score);
    
    // If score exceeds all thresholds, clamp to last level
    &levels[i.min(levels.len() - 1)]
}

fn criterion_benchmark(c: &mut Criterion) {
    let levels = ItemLevel::KillEaterRank.levels();
    
    c.bench_function("loop (score 9000)", |b| b.iter(|| {
        loop_levels(9000, levels)
    }));
    
    c.bench_function("loop (score 500)", |b| b.iter(|| {
        loop_levels(500, levels)
    }));
    
    c.bench_function("loop (score 5)", |b| b.iter(|| {
        loop_levels(5, levels)
    }));
    
    c.bench_function("binary_search (score 9000)", |b| b.iter(|| {
        binary_search(9000, levels)
    }));
    
    c.bench_function("binary_search (score 500)", |b| b.iter(|| {
        binary_search(500, levels)
    }));
    
    c.bench_function("binary_search (score 5)", |b| b.iter(|| {
        binary_search(5, levels)
    }));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}
 
criterion_main!(benches);