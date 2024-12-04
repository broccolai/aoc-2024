use yaah::{aoc_lib, aoc_year};

aoc_year!(2024);

#[allow(unused)]
mod utilities {
    pub mod grid;
    pub mod point;
    pub mod string;
    pub mod tuple;
}

mod days {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
}

aoc_lib!(with_benchmarks);
