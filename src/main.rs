use atty::Stream;
use clap::Clap;
use cli_table::{print_stdout, Color, Table, WithTitle};
use std::fs::read_to_string;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Clap, Debug)]
#[clap(version = VERSION, author = "Takashi I. <mail@takashiidobe.com>")]
struct Opts {
    file_path: String,
}

fn stddev(nums: &[i64]) -> Option<f64> {
    match (mean(nums), nums.len()) {
        (Some(nums_mean), count) if count > 0 => {
            let variance = nums
                .iter()
                .map(|value| {
                    let diff = nums_mean - (*value as f64);

                    diff * diff
                })
                .sum::<f64>()
                / count as f64;

            Some(variance.sqrt())
        }
        _ => None,
    }
}

fn mean(nums: &[i64]) -> Option<f64> {
    let sum = nums.iter().sum::<i64>() as f64;
    let count = nums.len();

    match count {
        positive if positive > 0 => Some(sum / count as f64),
        _ => None,
    }
}

#[derive(Debug, Table)]
struct Stats {
    #[table(color = "Color::Red")]
    max: i64,
    #[table(color = "Color::Green")]
    min: i64,
    #[table(color = "Color::Cyan")]
    total: i64,
    #[table(color = "Color::Magenta")]
    count: usize,
    #[table(color = "Color::Yellow")]
    mean: f64,
    #[table(color = "Color::Rgb(173, 215, 230)")]
    stddev: f64,
}

impl Stats {
    fn new(nums: &[i64]) -> Stats {
        let total: i64 = nums.iter().sum();
        let min = *nums.iter().min().unwrap();
        let max = *nums.iter().max().unwrap();
        let count = nums.len();
        let mean = mean(nums).unwrap();
        let stddev = stddev(nums).unwrap();
        Stats {
            max,
            min,
            total,
            count,
            mean,
            stddev,
        }
    }
}

fn print_simple(stats: &Stats) {
    println!("max min total count mean stddev");
    println!(
        "{} {} {} {} {} {}",
        stats.max, stats.min, stats.total, stats.count, stats.mean, stats.stddev
    );
}

fn main() {
    let opts = Opts::parse();

    let str_file = read_to_string(&opts.file_path).unwrap();

    let nums: Vec<i64> = str_file.lines().map(|s| s.parse().unwrap()).collect();

    let stats = Stats::new(&nums);
    if atty::is(Stream::Stdout) {
        let _ = print_stdout(vec![stats].with_title());
    } else {
        print_simple(&stats);
    }
}
