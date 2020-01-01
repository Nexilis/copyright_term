use rand::Rng;
use std::{cmp, io};

fn main() {
    println!("Welcome to Copyright Term Calculator.\nCheck https://github.com/Nexilis/copyright_term for the newest version.\n\n");
    println!("Type a number of days in a work period alternately with a number of days in a workless period. The first number is for the work period, i.e. '5;2;5' would mean: 5 working days, 2 free days, 5 working days.");
    println!("If you type nothing the default is going to be '5;2;5;2;5;2;5'");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading line");

    if user_input.trim().is_empty() {
        user_input = String::from("5;2;5;2;5;2;5");
    }

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

    print_calendar_summary(&calendar, &periods);
}

fn gen_calendar(days: usize) -> Vec<usize> {
    let h_total = (days * 8) as f64;
    let mut remaining_h_total = (h_total * 0.8).floor() as usize;
    let mut calendar = vec![0; days];
    while remaining_h_total > 0 {
        let i = rand::thread_rng().gen_range(0, days);
        let copyrighted_h = calendar[i];

        if copyrighted_h >= 8 {
        } else {
            let not_copyrighted_h = 8 - copyrighted_h;
            let max_additional_h = cmp::min(remaining_h_total, not_copyrighted_h);
            let additional_h = rand::thread_rng().gen_range(1, max_additional_h + 1);
            calendar[i] = copyrighted_h + additional_h;
            remaining_h_total -= additional_h;
        }
    }
    return calendar;
}

fn print_calendar_summary(calendar: &Vec<usize>, periods: &Vec<usize>) {
    println!("\nCopyright hours:");
    print_formatted(calendar, periods);

    println!("\nRemaining hours:");
    let calc_remainder: Vec<usize> = calendar.iter().map(|x| 8 - *x).collect();
    print_formatted(&calc_remainder, periods);
}

fn print_formatted(calendar: &Vec<usize>, periods: &Vec<usize>) {
    let mut printed_work_days = 0;

    periods.iter().enumerate().for_each(|(i, &period)| {
        if i % 2 == 0 {
            calendar
                .iter()
                .skip(printed_work_days)
                .take(period)
                .for_each(|work_day| println!("{}", work_day));

            printed_work_days += period;
        } else {
            for _ in 0..period {
                println!(" ");
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use crate::gen_calendar;

    #[test]
    fn sum_of_copyrighted_hours_is_correct() {
        let days = 0..101;
        for d in days {
            println!("Testing sum of hours for {} days", d);
            let expected = (d as f32 * 8. * 0.8).floor() as usize;

            let actual = gen_calendar(d).iter().sum();
            assert_eq!(expected, actual);
        }
    }
}
