use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("Type the number of work days (default 20)");
    let mut days = String::new();
    io::stdin()
        .read_line(&mut days)
        .expect("Failed to read line!");
    let days: u32 = match days.trim().parse() {
        Ok(num) => num,
        Err(_) => 20,
    };

    let hours = (days * 8) as f32;
    let under = (hours * 0.8).floor();
    let non = hours - under;
    let precise = under / hours;

    println!("Whole period\n  days: {}\n  hours: {}\n", days, hours);

    println!(
        "Copyright hours\n  under: {}\n  non: {}\n\nPrecise percentage: {}\n",
        under, non, precise,
    );

    let calendar = gen_calendar(days as usize);

    println!("Copyright hours");
    for x in &calendar {
        println!("{}", x);
    }

    println!("Remaining hours");
    for x in &calendar {
        println!("{}", 8 - *x);
    }
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
