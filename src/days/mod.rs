mod day1;
mod day2;
#[allow(clippy::precedence, clippy::identity_op)]
mod day3;
#[allow(clippy::redundant_closure, clippy::len_zero)]
mod day4;
mod day5;
#[allow(clippy::map_collect_result_unit)]
mod day6;
#[allow(clippy::stable_sort_primitive)]
mod day7;
mod day8;

mod day1_revised;
mod day2_revised;
mod day3_revised;
mod day7_revised;

crate::days!(
    day1
    day2
    day3
    day4
    day5
    day6
    day7
    day8

    day1_revised
    day2_revised
    day3_revised
    day7_revised
);
