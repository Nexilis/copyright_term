use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("Type the number of work days with breaks, e.g. 5;2;5");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Error reading line");

    let x1: Vec<&str> = user_input.split(';').collect();

    let days_with_splits: Vec<i32> = x1
        .iter()
        .map(|x| x.trim().parse().expect("Wrong input"))
        .collect();

    let mut index = 0;
    let days: i32 = days_with_splits.iter().fold(0, |acc, x| {
        if index % 2 == 0 {
            index += 1;
            acc + *x
        } else {
            index += 1;
            acc
        }
    });

    let hours = (days * 8) as f64;
    let under = (hours * 0.8).floor();
    let non = hours - under;
    let precise = under / hours;

    println!("Whole period\n  days: {}\n  hours: {}\n", days, hours);

    println!(
        "Copyright hours\n  under: {}\n  non: {}\n\nPrecise percentage: {}\n",
        under, non, precise,
    );

    let calendar = gen_calendar(days as usize);

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
    print_with_breaks(&calendar, &breaks);
    let remaining: Vec<i32> = calendar.iter().map(|x| 8 - *x).collect();
    print_with_breaks(&remaining, &breaks);
}

fn print_with_breaks(calendar: &Vec<i32>, breaks: &Vec<i32>) {
    println!("Copyright hours");

    //let lines: i32 = breaks.iter().sum();

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
    fn sum_of_hours_under_copyright_is_correct() {
        let days = (0..101);
        for d in days {
            println!("Testing days: {}", d);
            let expected: i32 = (d as f32 * 8. * 0.8).floor() as i32;

            let actual = gen_calendar(d).iter().sum();

            assert_eq!(expected, actual);
        }
    }
}
