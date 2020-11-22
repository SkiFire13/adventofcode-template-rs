use clap::{App, Arg, ArgMatches};
pub fn get_clap_matches<'a>(default: &'a str) -> ArgMatches<'a> {
    App::new("My Super Program")
        .version("0.1.0")
        .author("Giacomo Stevanato <giaco.stevanato@gmail.com>")
        .about("My solutions to Advent of code 2020")
        .arg(Arg::with_name("day")
            .short("d")
            .long("day")
            .value_name("DAY")
            .help("Run the solution for the day $DAY")
            .default_value(default)
            .takes_value(true))
        .get_matches()
}

#[macro_export]
macro_rules! advent_of_code {
    ($year:literal => $($d:ident),+ $(,)?) => {
        pub mod prelude;
        $( mod $d; )+

        fn main() {
            const DEFAULT_DAY: &str = advent_of_code!(@LATEST_DAY $( $d )+);
            let matches = $crate::main_impl::get_clap_matches(&DEFAULT_DAY[3..]);
            let requested_day = matches.value_of("day");
            let mut total = ::std::time::Duration::default();
            advent_of_code!(@MAIN $year requested_day total $( $d )+);
            println!("Took in total: {:.3?}", total);
            println!();
        }
    };
    (@LATEST_DAY $d:ident) => { stringify!($d) };
    (@LATEST_DAY $d:ident $($o:ident)+) => { advent_of_code!(@LATEST_DAY $( $o )+) };
    (@MAIN $year:literal $requested_day:ident $total:ident) => {};
    (@MAIN $year:literal $requested_day:ident $total:ident $($d:ident)*) => {
        $(
            if $requested_day == Some(&stringify!($d)[3..]) || $requested_day == Some("all") {
                const DAY: &str = stringify!($d);
                print!("Day {:<2}", &DAY[3..]);
                let input = std::fs::read_to_string(format!("./input/{}/{}.txt", $year, DAY))
                    .expect("Couldn't read input file");
            
                let now = std::time::Instant::now();
                let input = $d::input_generator(input.trim_end());
                let elapsed = now.elapsed();
                $total += elapsed;
                println!(      "  - Input parsed");
                println!("          Took {:.3?}", elapsed);
                println!();
            
                let now = std::time::Instant::now();
                let part1_solution = $d::part1(&input);
                let elapsed = now.elapsed();
                $total += elapsed;
                println!("        - Part 1: {}", part1_solution);
                println!("          Took {:.3?}", elapsed);
                println!();
            
                advent_of_code!(@PART2 input $total $d);
            }
        )*
    };
    (@PART2 $input:ident $total:ident day25) => {};
    (@PART2 $input:ident $total:ident $d:ident) => {
        let now = std::time::Instant::now();
        let part2_solution = $d::part2(&$input);
        let elapsed = now.elapsed();
        $total += elapsed;
        println!("        - Part 2: {}", part2_solution);
        println!("          Took {:.3?}", elapsed);
        println!();
    };
}
