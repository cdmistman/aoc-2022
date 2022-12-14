#![feature(assert_matches)]
#![feature(iter_array_chunks)]
#![feature(maybe_uninit_uninit_array)]

#[macro_use]
extern crate aoc_runner_derive;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

aoc_lib! {
  year = 2022
}
