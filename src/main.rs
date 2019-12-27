use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("For how many days: ");
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

    let calendar = gen_calendar(days as usize, under as i32);

    println!("Copyright hours");
    for x in &calendar {
        println!("{}", x);
    }

    println!("Remaining hours");
    for x in &calendar {
        println!("{}", 8 - *x);
    }
}

fn gen_calendar(days: usize, mut remaining: i32) -> Vec<i32> {
    let mut calendar = vec![0; days];
    let mut i_count = 0;
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
        i_count += 1;
    }
    println!("Iterations: {}", i_count);
    return calendar;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
