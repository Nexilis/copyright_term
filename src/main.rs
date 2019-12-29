use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("Type a number of days in a work period alternately with a number of days in a workless period, e.g. 5;2;5 where the first number is for the work period:");
    let mut user_input = String::new();
    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");

    let user_input_split: Vec<&str> = user_input.split(';').collect();

    let periods: Vec<usize> = user_input_split
        .iter()
        .map(|x| x.trim().parse().expect("Wrong input"))
        .collect();

    let working_days: usize =
        periods
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &val)| if i % 2 == 0 { acc + val } else { acc });

    let working_hours = (working_days * 8) as f64;
    let copyrighted = (working_hours * 0.8).floor();
    let not_copyrighted = working_hours - copyrighted;
    let copyright_ratio = copyrighted / working_hours;

    println!(
        "Sum of working days: {}, working hours: {}",
        working_days, working_hours
    );

    println!(
        "Hours copyrighted: {}, not copyrighted: {}, ratio: {}\n",
        copyrighted, not_copyrighted, copyright_ratio,
    );

    let calendar = gen_calendar(working_days as usize);

    print(&calendar, &periods);
}

fn gen_calendar(days: usize) -> Vec<usize> {
    let hours = (days * 8) as f32;
    let mut remaining = (hours * 0.8).floor() as usize;
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

fn print(calendar: &Vec<usize>, periods: &Vec<usize>) {
    println!("\nCopyright hours:");
    print_with_breaks(calendar, periods);

    println!("\nRemaining hours:");
    let calc_remainder: Vec<usize> = calendar.iter().map(|x| 8 - *x).collect();
    print_with_breaks(&calc_remainder, periods);
}

fn print_with_breaks(calendar: &Vec<usize>, periods: &Vec<usize>) {
    let mut printed_work_days = 0;

    for (i, &period) in periods.iter().enumerate() {
        if i % 2 == 0 {
            for work_day in calendar.iter().skip(printed_work_days).take(period) {
                println!("{}", *work_day);
            }
            printed_work_days += period;
        } else {
            for _ in 0..period {
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
            let expected = (d as f32 * 8. * 0.8).floor() as usize;

            let actual = gen_calendar(d).iter().sum();
            assert_eq!(expected, actual);
        }
    }
}
