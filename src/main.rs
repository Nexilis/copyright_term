use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("Type a number of days in a work period alternately with a number of days in a workless period, e.g. 5;2;5 where the first number is for the work period:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");

    let x1: Vec<&str> = user_input.split(';').collect();

    let days_with_splits: Vec<i32> = x1
        .iter()
        .map(|x| x.trim().parse().expect("Wrong input"))
        .collect();

    let mut index = 0;
    let working_days: i32 = days_with_splits.iter().fold(0, |acc, x| {
        if index % 2 == 0 {
            index += 1;
            acc + *x
        } else {
            index += 1;
            acc
        }
    });

    let working_hours = (working_days * 8) as f64;
    let copyrighted = (working_hours * 0.8).floor();
    let not_copyrighted = working_hours - copyrighted;
    let copyright_ratio = copyrighted / working_hours;

    println!("Sum of: working days: {}, working hours: {}", working_days, working_hours);

    println!(
        "Hours copyrighted: {}, not copyrighted: {}, ratio: {}\n",
        copyrighted, not_copyrighted, copyright_ratio,
    );

    let calendar = gen_calendar(working_days as usize);

    print(&calendar, &days_with_splits);
}

fn gen_calendar(days: usize) -> Vec<i32> {
    let hours = (days * 8) as f32;
    let mut remaining = (hours * 0.8).floor() as i32;
    let mut calendar = vec![0; days];
    while remaining > 0 {
        let index = rand::thread_rng().gen_range(0, days);
        let val = calendar[index];

        if val == 0 {
            let max_under = cmp::min(remaining + 1, 9);
            let under_this_day = rand::thread_rng().gen_range(0, max_under);
            calendar[index] = under_this_day;
            remaining = remaining - under_this_day;
        } else if val >= 8 {
        } else {
            calendar[index] = val + 1;
            remaining -= 1;
        }
    }
    return calendar;
}

fn print(calendar: &Vec<i32>, breaks: &Vec<i32>) {
    println!("\nCopyright hours:");
    print_with_breaks(&calendar, &breaks);

    println!("\nRemaining hours:");
    let remaining: Vec<i32> = calendar.iter().map(|x| 8 - *x).collect();
    print_with_breaks(&remaining, &breaks);
}

fn print_with_breaks(calendar: &Vec<i32>, breaks: &Vec<i32>) {
    let mut index = 0;
    let mut presented = 0;
    for pair in breaks.iter().map(|x| {
        index += 1;
        (index, *x)
    }) {
        let (i, val) = pair;

        if i % 2 != 0 {
            for x in calendar.iter().skip(presented).take(val as usize) {
                println!("{}", *x);
            }
            presented += val as usize;
        } else {
            for _ in 0..val {
                println!(" ");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gen_calendar;

    #[test]
    fn sum_of_copyrighted_hours_is_correct() {
        let days = 0..101;
        for d in days {
            println!("Testing days: {}", d);
            let expected: i32 = (d as f32 * 8. * 0.8).floor() as i32;

            let actual = gen_calendar(d).iter().sum();
            assert_eq!(expected, actual);
        }
    }
}
