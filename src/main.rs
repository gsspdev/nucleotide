#![allow(unused_imports)]
#![allow(dead_code)]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate atty;
extern crate shlex;
extern crate time;

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use derive_builder::Builder;

use clap::{App, Arg, ArgMatches, Parser};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, value_parser)]
    name: String,

    #[clap(short, long, value_parser, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();
    
    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
}





















/////////////////////////////////////////////////////////////////////
// const DEFAULT_HISTORY_SIZE: usize = 1000;

// fn main() {
//     env_logger::builder().format_timestamp_nanos().init();

//     match real_main() {
//         Ok(exit_code) => std::process::exit(exit_code),
//         Err(err) => {
//             // if pipe is closed, exit silently
//             if err.kind() == std::io::ErrorKind::BrokenPipe {
//                 std::process::exit(0);
//             }
//             std::process::exit(2);
//             }
//         }
//     }

// // #[rustfmt::skip]
// fn real_main() -> Result<i32, std::io::Error> {
//     let mut stdout = std::io::stdout();

//     let mut args = Vec::new();

//     args.push(env::args().next().expect("there should be at least one arg: the application name"));
//     args.extend(env::var("SKIM_DEFAULT_OPTIONS")
//         .ok()
//         .and_then(|val| shlex::split(&val))
//         .unwrap_or_default());
//     for arg in env::args().skip(1) {
//         args.push(arg);
//     }


//     //------------------------------------------------------------------------------
//     // parse options
//     let opts = App::new("nk")
//         .author("Benjamin Hampikian<dev@gssp.io>")
//         // .version(crate_version!())
//         .arg(Arg::with_name("help").long("help").short('h'))
//         .get_matches_from(args);

//         if opts.is_present("help") {
//             write!(stdout, "{}", USAGE)?;
//             return Ok(0);
//         }

//         let mut options = parse_options(&opts);

// }

// fn parse_options(options: &ArgMatches) -> SkimOptions<'_> {
//     SkimOptionsBuilder::default()
//         // .color(options.values_of("color").and_then(|vals| vals.last()))
//         // .min_height(options.values_of("min-height").and_then(|vals| vals.last()))
//         // .no_height(options.is_present("no-height"))
//         // .height(options.values_of("height").and_then(|vals| vals.last()))
//         // .margin(options.values_of("margin").and_then(|vals| vals.last()))
//         // .preview(options.values_of("preview").and_then(|vals| vals.last()))
//         // .cmd(options.values_of("cmd").and_then(|vals| vals.last()))
//         // .query(options.values_of("query").and_then(|vals| vals.last()))
//         // .cmd_query(options.values_of("cmd-query").and_then(|vals| vals.last()))
//         // .interactive(options.is_present("interactive"))
//         // .prompt(options.values_of("prompt").and_then(|vals| vals.last()))
//         // .cmd_prompt(options.values_of("cmd-prompt").and_then(|vals| vals.last()))
//         // .bind(
//         //     options
//         //         .values_of("bind")
//         //         .map(|x| x.collect::<Vec<_>>())
//         //         .unwrap_or_default(),
//         // )
//         // .expect(options.values_of("expect").map(|x| x.collect::<Vec<_>>().join(",")))
//         // .multi(if options.is_present("no-multi") {
//         //     false
//         // } else {
//         //     options.is_present("multi")
//         // })
//         // .layout(options.values_of("layout").and_then(|vals| vals.last()).unwrap_or(""))
//         // .reverse(options.is_present("reverse"))
//         // .no_hscroll(options.is_present("no-hscroll"))
//         // .no_mouse(options.is_present("no-mouse"))
//         // .no_clear(options.is_present("no-clear"))
//         // .no_clear_start(options.is_present("no-clear-start"))
//         // .tabstop(options.values_of("tabstop").and_then(|vals| vals.last()))
//         // .tiebreak(options.values_of("tiebreak").map(|x| x.collect::<Vec<_>>().join(",")))
//         // .tac(options.is_present("tac"))
//         // .nosort(options.is_present("no-sort"))
//         // .exact(options.is_present("exact"))
//         // .regex(options.is_present("regex"))
//         // .delimiter(options.values_of("delimiter").and_then(|vals| vals.last()))
//         // .inline_info(options.is_present("inline-info"))
//         // .header(options.values_of("header").and_then(|vals| vals.last()))
//         // .header_lines(
//         //     options
//         //         .values_of("header-lines")
//         //         .and_then(|vals| vals.last())
//         //         .map(|s| s.parse::<usize>().unwrap_or(0))
//         //         .unwrap_or(0),
//         // )
//         // .layout(options.values_of("layout").and_then(|vals| vals.last()).unwrap_or(""))
//         // .algorithm(FuzzyAlgorithm::of(
//         //     options.values_of("algorithm").and_then(|vals| vals.last()).unwrap(),
//         // ))
//         // .case(match options.value_of("case") {
//         //     Some("smart") => CaseMatching::Smart,
//         //     Some("ignore") => CaseMatching::Ignore,
//         //     _ => CaseMatching::Respect,
//         // })
//         // .keep_right(options.is_present("keep-right"))
//         // .skip_to_pattern(
//         //     options
//         //         .values_of("skip-to-pattern")
//         //         .and_then(|vals| vals.last())
//         //         .unwrap_or(""),
//         // )
//         // .select1(options.is_present("select-1"))
//         // .exit0(options.is_present("exit-0"))
//         // .sync(options.is_present("sync"))
//         // .no_clear_if_empty(options.is_present("no-clear-if-empty"))
//         .build()
//         .unwrap()
// }
