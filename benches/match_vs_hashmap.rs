use criterion::{criterion_group, criterion_main, Criterion};
use std::collections::HashMap;

fn match_group(defindex: u32) -> Option<&'static str> {
    match defindex {
        0 => Some("Bat"),
        1 => Some("Bottle"),
        2 => Some("Fire Axe"),
        3 => Some("Kukri"),
        4 => Some("Knife"),
        5 => Some("Fists"),
        6 => Some("Shovel"),
        7 => Some("Wrench"),
        8 => Some("Bonesaw"),
        9 | 10 | 11 | 12 => Some("Shotgun"),
        13 => Some("Scattergun"),
        14 => Some("Sniper Rifle"),
        15 => Some("Minigun"),
        16 => Some("SMG"),
        17 => Some("Syringe Gun"),
        18 => Some("Rocket Launcher"),
        19 => Some("Grenade Launcher"),
        20 => Some("Stickybomb Launcher"),
        21 => Some("Flame Thrower"),
        22 | 23  => Some("Pistol"),
        24 => Some("Revolver"),
        25 => Some("Construction PDA"),
        26 => Some("Destruction PDA"),
        27 => Some("Disguise Kit"),
        28 => Some("PDA"),
        29 => Some("Medi Gun"),
        30 => Some("Invis Watch"),
        _ => None,
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let items = vec![
        (0, "Bat"),
        (1, "Bottle"),
        (2, "Fire Axe"),
        (3, "Kukri"),
        (4, "Knife"),
        (5, "Fists"),
        (6, "Shovel"),
        (7, "Wrench"),
        (8, "Bonesaw"),
        (9, "Shotgun"),
        (10, "Shotgun"),
        (11, "Shotgun"),
        (12, "Shotgun"),
        (13, "Scattergun"),
        (14, "Sniper Rifle"),
        (15, "Minigun"),
        (16, "SMG"),
        (17, "Syringe Gun"),
        (18, "Rocket Launcher"),
        (19, "Grenade Launcher"),
        (20, "Stickybomb Launcher"),
        (21, "Flame Thrower"),
        (22, "Pistol"),
        (23, "Pistol"),
        (24, "Revolver"),
        (25, "Construction PDA"),
        (26, "Destruction PDA"),
        (27, "Disguise Kit"),
        (28, "PDA"),
        (29, "Medi Gun"),
        (30, "Invis Watch"),
    ];
    let hash_map: HashMap<u32, &'static str> = items.into_iter().collect();
    
    c.bench_function("match", |b| b.iter(|| {
        match_group(30)
    }));
    
    c.bench_function("hash", |b| b.iter(|| {
        hash_map.get(&30)
    }));
}

criterion_group!{
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
}
 
criterion_main!(benches);