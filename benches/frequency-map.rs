// benches/frequency-map.rs : benchmarking `FrequencyMap`

#![allow(non_snake_case)]
#![feature(custom_inner_attributes)]

use collect_rs::containers::FrequencyMap;

use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    BatchSize,
    Criterion,
};


mod constants {
    #![rustfmt::skip]

    use std::ops as std_ops;


    pub(crate) const I32_SMALL : [i32; 10] = [
        // insert list:
         0,  1,  2,  3,  1,  5,  6,  6,  6,  3,
    ];

    pub(crate) const I32_MEDIUM : [i32; 100] = [
        // insert list:
         0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
        30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
    ];

    pub(crate) const I32_MEDIUM_UNIQ : [i32; 100] = [
        // insert list:
         0,  1,  2,  3,  4,  5,  6,  7,  8,  9,
        10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
        30, 31, 32, 33, 34, 35, 36, 37, 38, 39,
        40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
        50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
        70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
        80, 81, 82, 83, 84, 85, 86, 87, 88, 89,
        90, 91, 92, 93, 94, 95, 96, 97, 98, 99,
    ];

    pub(crate) const I32_N_SMALL : [(i32, isize); 6] = [
        // insert list:
        (0, 1),
        (1, 2),
        (2, 1),
        (3, 2),
        (5, 1),
        (6, 3),
    ];

    pub(crate) const I32_N_MEDIUM : [(i32, isize); 60] = [
        // insert list:
        ( 0, 1),
        ( 1, 1),
        ( 2, 1),
        ( 3, 1),
        ( 4, 1),
        ( 5, 1),
        ( 6, 1),
        ( 7, 1),
        ( 8, 1),
        ( 9, 1),
        (10, 2),
        (11, 2),
        (12, 2),
        (13, 2),
        (14, 2),
        (15, 2),
        (16, 2),
        (17, 2),
        (18, 2),
        (19, 2),
        (20, 1),
        (21, 1),
        (22, 1),
        (23, 1),
        (24, 1),
        (25, 1),
        (26, 1),
        (27, 1),
        (28, 1),
        (29, 1),
        (30, 2),
        (31, 2),
        (32, 2),
        (33, 2),
        (34, 2),
        (35, 2),
        (36, 2),
        (37, 2),
        (38, 2),
        (39, 2),
        (50, 1),
        (51, 1),
        (52, 1),
        (53, 1),
        (54, 1),
        (55, 1),
        (56, 1),
        (57, 1),
        (58, 1),
        (59, 1),
        (60, 3),
        (61, 3),
        (62, 3),
        (63, 3),
        (64, 3),
        (65, 3),
        (66, 3),
        (67, 3),
        (68, 3),
        (69, 3),
    ];

    pub(crate) const I32_N_MEDIUM_UNIQ : [(i32, isize); 100] = [
        // insert list:
        ( 0, 1),
        ( 1, 1),
        ( 2, 1),
        ( 3, 1),
        ( 4, 1),
        ( 5, 1),
        ( 6, 1),
        ( 7, 1),
        ( 8, 1),
        ( 9, 1),
        (10, 1),
        (11, 1),
        (12, 1),
        (13, 1),
        (14, 1),
        (15, 1),
        (16, 1),
        (17, 1),
        (18, 1),
        (19, 1),
        (20, 1),
        (21, 1),
        (22, 1),
        (23, 1),
        (24, 1),
        (25, 1),
        (26, 1),
        (27, 1),
        (28, 1),
        (29, 1),
        (30, 1),
        (31, 1),
        (32, 1),
        (33, 1),
        (34, 1),
        (35, 1),
        (36, 1),
        (37, 1),
        (38, 1),
        (39, 1),
        (40, 1),
        (41, 1),
        (42, 1),
        (43, 1),
        (44, 1),
        (45, 1),
        (46, 1),
        (47, 1),
        (48, 1),
        (49, 1),
        (50, 1),
        (51, 1),
        (52, 1),
        (53, 1),
        (54, 1),
        (55, 1),
        (56, 1),
        (57, 1),
        (58, 1),
        (59, 1),
        (60, 1),
        (61, 1),
        (62, 1),
        (63, 1),
        (64, 1),
        (65, 1),
        (66, 1),
        (67, 1),
        (68, 1),
        (69, 1),
        (70, 1),
        (71, 1),
        (72, 1),
        (73, 1),
        (74, 1),
        (75, 1),
        (76, 1),
        (77, 1),
        (78, 1),
        (79, 1),
        (80, 1),
        (81, 1),
        (82, 1),
        (83, 1),
        (84, 1),
        (85, 1),
        (86, 1),
        (87, 1),
        (88, 1),
        (89, 1),
        (90, 1),
        (91, 1),
        (92, 1),
        (93, 1),
        (94, 1),
        (95, 1),
        (96, 1),
        (97, 1),
        (98, 1),
        (99, 1),
    ];

    pub(crate) const I32_RANGE_EMPTY : std_ops::Range<i32> = 0..0;

    pub(crate) const I32_RANGE_SMALL : std_ops::Range<i32> = 0..10;

    pub(crate) const I32_RANGE_MEDIUM : std_ops::Range<i32> = 0..100;

    pub(crate) const I32_RANGE_MEDIUM_2 : std_ops::Range<i32> = 20..60;
    pub(crate) const I32_RANGE_MEDIUM_3 : std_ops::Range<i32> = 40..80;

    pub(crate) const I32_RANGE_LARGE : std_ops::Range<i32> = 0..10000;

    pub(crate) const I32_RANGE_LARGE_2 : std_ops::Range<i32> = 2000..6000;
    pub(crate) const I32_RANGE_LARGE_3 : std_ops::Range<i32> = 4000..8000;
}

fn BENCHMARK_new(c : &mut Criterion) {
    let id = format!("`frequency_map::new()`");

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm : FrequencyMap<String> = FrequencyMap::new();

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_with_capacity_SMALL(c : &mut Criterion) {
    const INITIAL_CAPACITY : usize = 10;
    let id = format!("`frequency_map::with_capacity({INITIAL_CAPACITY})`");

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm : FrequencyMap<String> = FrequencyMap::with_capacity(black_box(INITIAL_CAPACITY));

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_with_capacity_LARGE(c : &mut Criterion) {
    const INITIAL_CAPACITY : usize = 1000;
    let id = format!("`frequency_map::with_capacity({INITIAL_CAPACITY})`");

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm : FrequencyMap<String> = FrequencyMap::with_capacity(INITIAL_CAPACITY);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_T_SMALL(c : &mut Criterion) {
    let input = constants::I32_SMALL;
    let id = format!("`frequency_map::from([T; {}])`", input.len());

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm = FrequencyMap::from(input);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_T_MEDIUM(c : &mut Criterion) {
    let input = constants::I32_MEDIUM;
    let id = format!("`frequency_map::from([T; {}])`", input.len());

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm = FrequencyMap::from(input);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_T_MEDIUM_UNIQ(c : &mut Criterion) {
    let input = constants::I32_MEDIUM_UNIQ;
    let id = format!("`frequency_map::from([T; {}])` - unique", input.len());

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm = FrequencyMap::from(input);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_T_N_SMALL(c : &mut Criterion) {
    let input = constants::I32_N_SMALL;
    let id = format!("`frequency_map::from([(T, isize); {}])`", input.len());

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm : FrequencyMap<i32> = FrequencyMap::from(input);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_T_N_MEDIUM(c : &mut Criterion) {
    let input = constants::I32_N_MEDIUM;
    let id = format!("`frequency_map::from([(T, isize); {}])`", input.len());

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm : FrequencyMap<i32> = FrequencyMap::from(input);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_T_N_MEDIUM_UNIQ(c : &mut Criterion) {
    let input = constants::I32_N_MEDIUM_UNIQ;
    let id = format!("`frequency_map::from([(T, isize); {}])` - unique", input.len());

    c.bench_function(&id, |b| {
        b.iter(|| {
            let fm : FrequencyMap<i32> = FrequencyMap::from(input);

            let _ = black_box(fm);
        })
    });
}

fn BENCHMARK_from_iter_EMPTY(c : &mut Criterion) {
    let input = constants::I32_RANGE_EMPTY;
    let id = format!("`frequency_map::from_iter({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched(
            || input.clone(),
            |input| {
                let fm = FrequencyMap::from_iter(input.into_iter());

                let _ = black_box(fm);
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_from_iter_SMALL(c : &mut Criterion) {
    let input = constants::I32_RANGE_SMALL;
    let id = format!("`frequency_map::from_iter({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched(
            || input.clone(),
            |input| {
                let fm = FrequencyMap::from_iter(input.into_iter());

                let _ = black_box(fm);
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_from_iter_MEDIUM(c : &mut Criterion) {
    let input = constants::I32_RANGE_MEDIUM;
    let id = format!("`frequency_map::from_iter({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched(
            || input.clone(),
            |input| {
                let fm = FrequencyMap::from_iter(input.into_iter());

                let _ = black_box(fm);
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_from_iter_MEDIUM_OVERLAPPING(c : &mut Criterion) {
    let input2 = constants::I32_RANGE_MEDIUM_2;
    let input3 = constants::I32_RANGE_MEDIUM_3;
    let id = format!("`frequency_map::from_iter({input2:?}.chain({input3:?})])`");

    c.bench_function(&id, |b| {
        b.iter_batched(
            || input2.clone().chain(input3.clone()),
            |input| {
                let fm = FrequencyMap::from_iter(input.into_iter());

                let _ = black_box(fm);
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_from_iter_LARGE(c : &mut Criterion) {
    let input = constants::I32_RANGE_LARGE;
    let id = format!("`frequency_map::from_iter({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched(
            || input.clone(),
            |input| {
                let fm = FrequencyMap::from_iter(input.into_iter());

                let _ = black_box(fm);
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_from_iter_LARGE_OVERLAPPING(c : &mut Criterion) {
    let input2 = constants::I32_RANGE_LARGE_2;
    let input3 = constants::I32_RANGE_LARGE_3;
    let id = format!("`frequency_map::from_iter({input2:?}.chain({input3:?})])`");

    c.bench_function(&id, |b| {
        b.iter_batched(
            || input2.clone().chain(input3.clone()),
            |input| {
                let fm = FrequencyMap::from_iter(input.into_iter());

                let _ = black_box(fm);
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_clear_T_MEDIUM(c : &mut Criterion) {
    let input = constants::I32_MEDIUM;
    let id = format!("`frequency_map::clear([T; {}])`", input.len());

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from(input),
            |fm| {
                black_box(fm.clear());
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_clear_T_MEDIUM_UNIQ(c : &mut Criterion) {
    let input = constants::I32_MEDIUM_UNIQ;
    let id = format!("`frequency_map::clear([T; {}])` - unique", input.len());

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from(input),
            |fm| {
                black_box(fm.clear());
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_EMPTY_TO_EMPTY(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_EMPTY;
    let input2 = constants::I32_RANGE_EMPTY;
    let id = format!("`frequency_map::append()` - empty to empty");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_EMPTY_TO_SMALL(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_SMALL;
    let input2 = constants::I32_RANGE_EMPTY;
    let id = format!("`frequency_map::append()` - empty to small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_SMALL_TO_EMPTY(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_EMPTY;
    let input2 = constants::I32_RANGE_SMALL;
    let id = format!("`frequency_map::append()` - small to empty");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_SMALL_TO_SMALL(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_SMALL;
    let input2 = constants::I32_RANGE_SMALL;
    let id = format!("`frequency_map::append()` - small to small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_LARGE_TO_SMALL(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_SMALL;
    let input2 = constants::I32_RANGE_LARGE;
    let id = format!("`frequency_map::append()` - large to small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_SMALL_TO_LARGE(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_LARGE;
    let input2 = constants::I32_RANGE_SMALL;
    let id = format!("`frequency_map::append()` - small to large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_append_LARGE_TO_LARGE(c : &mut Criterion) {
    let input1 = constants::I32_RANGE_LARGE;
    let input2 = constants::I32_RANGE_LARGE;
    let id = format!("`frequency_map::append()` - large to large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || {
                (
                    FrequencyMap::from_iter(input1.clone().into_iter()),
                    FrequencyMap::from_iter(input2.clone().into_iter()),
                )
            },
            |(dest, src)| {
                black_box(dest.append(src));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_insert_EMPTY(c : &mut Criterion) {
    let id = format!("`frequency_map::insert()` - new value into empty");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::with_capacity(10),
            |fm| {
                black_box(fm.insert(1, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_insert_SMALL_NEW_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::insert()` - new value into small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from([1, 2, 4, 8, 16, 32]),
            |fm| {
                black_box(fm.insert(3, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_insert_SMALL_UPDATE_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::insert()` - update value into small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from([1, 2, 4, 8, 16, 32]),
            |fm| {
                black_box(fm.insert(3, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_insert_LARGE_NEW_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::insert()` - new value into large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.insert(10000000, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_insert_LARGE_UPDATE_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::insert()` - update value into large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.insert(3, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_EMPTY(c : &mut Criterion) {
    let id = format!("`frequency_map::push()` - new value into empty");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::with_capacity(10),
            |fm| {
                black_box(fm.push(1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_SMALL_NEW_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push()` - new value into small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from([1, 2, 4, 8, 16, 32]),
            |fm| {
                black_box(fm.push(3));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_SMALL_UPDATE_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push()` - update value into small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from([1, 2, 4, 8, 16, 32]),
            |fm| {
                black_box(fm.push(3));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_LARGE_NEW_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push()` - new value into large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.push(10000000));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_LARGE_UPDATE_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push()` - update value into large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.push(3));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_n_EMPTY(c : &mut Criterion) {
    let id = format!("`frequency_map::push_n()` - new value into empty");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::with_capacity(10),
            |fm| {
                black_box(fm.push_n(1, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_n_SMALL_NEW_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push_n()` - new value into small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from([1, 2, 4, 8, 16, 32]),
            |fm| {
                black_box(fm.push_n(3, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_n_SMALL_UPDATE_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push_n()` - update value into small");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from([1, 2, 4, 8, 16, 32]),
            |fm| {
                black_box(fm.push_n(3, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_n_LARGE_NEW_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push_n()` - new value into large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.push_n(10000000, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_push_n_LARGE_UPDATE_ITEM(c : &mut Criterion) {
    let id = format!("`frequency_map::push_n()` - update value into large");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.push_n(3, 101));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_contains_key_SMALL_KEY_ABSENT(c : &mut Criterion) {
    let id = format!("`frequency_map::contains_key()` - small, key absent");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_SMALL.into_iter()),
            |fm| {
                black_box(fm.contains_key(&-1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_contains_key_SMALL_KEY_PRESENT(c : &mut Criterion) {
    let id = format!("`frequency_map::contains_key()` - small, key present");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_SMALL.into_iter()),
            |fm| {
                black_box(fm.contains_key(&1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_contains_key_MEDIUM_KEY_ABSENT(c : &mut Criterion) {
    let id = format!("`frequency_map::contains_key()` - medium, key absent");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_MEDIUM.into_iter()),
            |fm| {
                black_box(fm.contains_key(&-1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_contains_key_MEDIUM_KEY_PRESENT(c : &mut Criterion) {
    let id = format!("`frequency_map::contains_key()` - medium, key present");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_MEDIUM.into_iter()),
            |fm| {
                black_box(fm.contains_key(&1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_contains_key_LARGE_KEY_ABSENT(c : &mut Criterion) {
    let id = format!("`frequency_map::contains_key()` - large, key absent");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.contains_key(&-1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_contains_key_LARGE_KEY_PRESENT(c : &mut Criterion) {
    let id = format!("`frequency_map::contains_key()` - large, key present");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.contains_key(&1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_get_SMALL_KEY_ABSENT(c : &mut Criterion) {
    let id = format!("`frequency_map::get()` - small, key absent");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_SMALL.into_iter()),
            |fm| {
                black_box(fm.get(&-1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_get_SMALL_KEY_PRESENT(c : &mut Criterion) {
    let id = format!("`frequency_map::get()` - small, key present");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_SMALL.into_iter()),
            |fm| {
                black_box(fm.get(&1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_get_MEDIUM_KEY_ABSENT(c : &mut Criterion) {
    let id = format!("`frequency_map::get()` - medium, key absent");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_MEDIUM.into_iter()),
            |fm| {
                black_box(fm.get(&-1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_get_MEDIUM_KEY_PRESENT(c : &mut Criterion) {
    let id = format!("`frequency_map::get()` - medium, key present");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_MEDIUM.into_iter()),
            |fm| {
                black_box(fm.get(&1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_get_LARGE_KEY_ABSENT(c : &mut Criterion) {
    let id = format!("`frequency_map::get()` - large, key absent");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.get(&-1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_get_LARGE_KEY_PRESENT(c : &mut Criterion) {
    let id = format!("`frequency_map::get()` - large, key present");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(constants::I32_RANGE_LARGE.into_iter()),
            |fm| {
                black_box(fm.get(&1));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_retain_EMPTY(c : &mut Criterion) {
    let input = constants::I32_RANGE_EMPTY;
    let id = format!("`frequency_map::retain({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(input.clone().into_iter()),
            |fm| {
                black_box(fm.retain(|k, _count| 0 == (k % 2)));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_retain_SMALL(c : &mut Criterion) {
    let input = constants::I32_RANGE_SMALL;
    let id = format!("`frequency_map::retain({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(input.clone().into_iter()),
            |fm| {
                black_box(fm.retain(|k, _count| 0 == (k % 2)));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_retain_MEDIUM(c : &mut Criterion) {
    let input = constants::I32_RANGE_MEDIUM;
    let id = format!("`frequency_map::retain({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(input.clone().into_iter()),
            |fm| {
                black_box(fm.retain(|k, _count| 0 == (k % 2)));
            },
            BatchSize::SmallInput,
        )
    });
}

fn BENCHMARK_retain_LARGE(c : &mut Criterion) {
    let input = constants::I32_RANGE_LARGE;
    let id = format!("`frequency_map::retain({input:?}])`");

    c.bench_function(&id, |b| {
        b.iter_batched_ref(
            || FrequencyMap::from_iter(input.clone().into_iter()),
            |fm| {
                black_box(fm.retain(|k, _count| 0 == (k % 2)));
            },
            BatchSize::SmallInput,
        )
    });
}



criterion_group!(
    benches,
    // new()
    BENCHMARK_new,
    // with_capacity()
    BENCHMARK_with_capacity_SMALL,
    BENCHMARK_with_capacity_LARGE,
    // from([T])
    BENCHMARK_from_T_SMALL,
    BENCHMARK_from_T_MEDIUM,
    BENCHMARK_from_T_MEDIUM_UNIQ,
    // from([T; N])
    BENCHMARK_from_T_N_SMALL,
    BENCHMARK_from_T_N_MEDIUM,
    BENCHMARK_from_T_N_MEDIUM_UNIQ,
    // from_iter()
    BENCHMARK_from_iter_EMPTY,
    BENCHMARK_from_iter_SMALL,
    BENCHMARK_from_iter_MEDIUM,
    BENCHMARK_from_iter_MEDIUM_OVERLAPPING,
    BENCHMARK_from_iter_LARGE,
    BENCHMARK_from_iter_LARGE_OVERLAPPING,
    // clear()
    BENCHMARK_clear_T_MEDIUM,
    BENCHMARK_clear_T_MEDIUM_UNIQ,
    // append()
    BENCHMARK_append_EMPTY_TO_EMPTY,
    BENCHMARK_append_EMPTY_TO_SMALL,
    BENCHMARK_append_SMALL_TO_EMPTY,
    BENCHMARK_append_SMALL_TO_SMALL,
    BENCHMARK_append_LARGE_TO_SMALL,
    BENCHMARK_append_SMALL_TO_LARGE,
    BENCHMARK_append_LARGE_TO_LARGE,
    // insert()
    BENCHMARK_insert_EMPTY,
    BENCHMARK_insert_SMALL_NEW_ITEM,
    BENCHMARK_insert_SMALL_UPDATE_ITEM,
    BENCHMARK_insert_LARGE_NEW_ITEM,
    BENCHMARK_insert_LARGE_UPDATE_ITEM,
    // push()
    BENCHMARK_push_EMPTY,
    BENCHMARK_push_SMALL_NEW_ITEM,
    BENCHMARK_push_SMALL_UPDATE_ITEM,
    BENCHMARK_push_LARGE_NEW_ITEM,
    BENCHMARK_push_LARGE_UPDATE_ITEM,
    // push_n()
    BENCHMARK_push_n_EMPTY,
    BENCHMARK_push_n_SMALL_NEW_ITEM,
    BENCHMARK_push_n_SMALL_UPDATE_ITEM,
    BENCHMARK_push_n_LARGE_NEW_ITEM,
    BENCHMARK_push_n_LARGE_UPDATE_ITEM,
    // retain()
    BENCHMARK_retain_EMPTY,
    BENCHMARK_retain_SMALL,
    BENCHMARK_retain_MEDIUM,
    BENCHMARK_retain_LARGE,
    // contains_key()
    BENCHMARK_contains_key_SMALL_KEY_ABSENT,
    BENCHMARK_contains_key_SMALL_KEY_PRESENT,
    BENCHMARK_contains_key_MEDIUM_KEY_ABSENT,
    BENCHMARK_contains_key_MEDIUM_KEY_PRESENT,
    BENCHMARK_contains_key_LARGE_KEY_ABSENT,
    BENCHMARK_contains_key_LARGE_KEY_PRESENT,
    // get()
    BENCHMARK_get_SMALL_KEY_ABSENT,
    BENCHMARK_get_SMALL_KEY_PRESENT,
    BENCHMARK_get_MEDIUM_KEY_ABSENT,
    BENCHMARK_get_MEDIUM_KEY_PRESENT,
    BENCHMARK_get_LARGE_KEY_ABSENT,
    BENCHMARK_get_LARGE_KEY_PRESENT,
);
criterion_main!(benches);
