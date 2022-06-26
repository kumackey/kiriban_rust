use std::{env, process};
use regex::Regex;

enum KiribanJudgement {
    Kiriban(i32, Vec<KiribanReason>),
    NonKiriban(i32, NonKiribanReason),
}

enum KiribanReason {
    // e.g.) 123, 56789
    Consecutive,

    // e.g.) 100, 200000
    SeriesOfZero,

    // e.g.) 111, 5555555
    Repdigit,
}

enum NonKiribanReason {
    // e.g.) 50
    TooSmall,

    NotHasReasons,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("invalid arguments {:?}", args);
        process::exit(1);
    }

    let value: i32 = args[1].parse().expect("invalid value");

    match judge_kiriban(value) {
        KiribanJudgement::NonKiriban(value, reason) => {
            match reason {
                NonKiribanReason::TooSmall => eprintln!("{} is too small", value),
                NonKiribanReason::NotHasReasons => eprintln!("{} is not kiriban", value),
            }
            process::exit(1);
        }
        KiribanJudgement::Kiriban(value, reasons) => {
            let reasons: Vec<String> = reasons.iter().map(|r| conv_reason_to_string(r)).collect();
            println!("{} is kiriban, because of {}", value, reasons.join(", "))
        }
    };
}

fn judge_kiriban(value: i32) -> KiribanJudgement {
    if value < 100 {
        return KiribanJudgement::NonKiriban(value, NonKiribanReason::TooSmall);
    }

    let reasons: Vec<KiribanReason> = judge_kiriban_reasons(value);
    if reasons.len() == 0 {
        return KiribanJudgement::NonKiriban(value, NonKiribanReason::NotHasReasons);
    }

    return KiribanJudgement::Kiriban(value, reasons);
}

fn judge_kiriban_reasons(value: i32) -> Vec<KiribanReason> {
    let mut reasons: Vec<KiribanReason> = Vec::new();
    let val: String = value.to_string();

    if "123456789".contains(&val) || "987654321".contains(&val) {
        reasons.push(KiribanReason::Consecutive);
    }

    if Regex::new(r"^[1-9]0+$").unwrap().is_match(&val) {
        reasons.push(KiribanReason::SeriesOfZero);
    }

    if Regex::new(r"^(1+|2+|3+|4+|5+|6+|7+|8+|9+)$").unwrap().is_match(&val) {
        reasons.push(KiribanReason::Repdigit);
    }

    return reasons;
}

fn conv_reason_to_string(reason: &KiribanReason) -> String {
    match reason {
        KiribanReason::Consecutive => String::from("consecutive"),
        KiribanReason::SeriesOfZero => String::from("series of zero"),
        KiribanReason::Repdigit => String::from("repdigit"),
    }
}