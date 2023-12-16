use makebench::make_benches;

make_benches! {
    INPUT_PATH: "bench_inputs";
    DAYS: [1, 1p2, 2, 2p2, 3, 3p2, 4, 4p2, 5, 5p2, 6, 6p2];
}
