use std::collections::HashMap;
use std::ops::{Sub, Div};
use itertools::Itertools;
use error::*;
use eval::Env;
use types::{Expr, List, Vector, Function, Lambda};
use util::*;

pub fn env<'a>() -> Env<'a> {
    let table: Vec<(&str, Lambda)> = vec![
        ("not", not),
        ("or", or),
        ("and", and),
        ("print", print),
        ("+", add),
        ("-", sub),
        ("*", mul),
        ("/", div),
        ("=", equal),
        ("<", less),
        ("<=", less_eq),
        (">", greater),
        (">=", greater_eq),
        ("first", first),
        ("rest", rest),
        ("cons", cons),
        ("exit", exit),
    ];

    let builtins = table
        .into_iter()
        .map(|(symbol, f)| {
            (
                String::from(symbol),
                Expr::from(Function::builtin(symbol, f)),
            )
        })
        .collect::<HashMap<_, _>>();

    Env::new(builtins, None)
}

fn numeric_op<F, G>(name: &str, args: &[Expr], fn_int: F, fn_flt: G) -> Result<Expr>
where
    F: Fn(&[i64]) -> Result<i64>,
    G: Fn(&[f64]) -> Result<f64>,
{
    // Check all arguments are numeric
    if args.iter().all(Expr::is_num) {
        if args.iter().any(Expr::is_flt) {
            // If any are float, promote to float
            let floats = args.iter()
                .map(|x| match x {
                    &Expr::Int(y) => y as f64,
                    &Expr::Flt(y) => y,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            fn_flt(&floats).map(Expr::from)
        } else {
            // Otherwise perform integer operation
            let ints = args.iter()
                .map(|x| match x {
                    &Expr::Int(y) => y,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            fn_int(&ints).map(Expr::from)
        }
    } else {
        Err(format!("#[{}] expected numeric", name).into())
    }
}

pub fn add(args: &[Expr], _env: &Env) -> Result<Expr> {
    numeric_op("+", args,
        |ints| Ok(ints.iter().sum::<i64>()),
        |floats| Ok(floats.iter().sum::<f64>())
    )
}

pub fn sub(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_min_args("-", args, 1)?;

    // Check all arguments are numeric
    if !args.iter().all(Expr::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If one argument, negate and return
    if args.len() == 1 {
        return match args[0] {
            Expr::Int(x) => Ok(Expr::from(-x)),
            Expr::Flt(x) => Ok(Expr::from(-x)),
            _ => Err("invalid type".into())
        }
    }

    numeric_op("-", args,
        |ints| Ok(ints[1..].iter().fold(ints[0], Sub::sub)),
        |floats| Ok(floats[1..].iter().fold(floats[0], Sub::sub))
    )
}

pub fn mul(args: &[Expr], _env: &Env) -> Result<Expr> {
    numeric_op("*", args,
        |ints| Ok(ints.iter().product::<i64>()),
        |floats| Ok(floats.iter().product::<f64>())
    )
}

pub fn div(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_min_args("[/]", args, 1)?;

    // Check all arguments are numeric
    if !args.iter().all(Expr::is_num) {
        return Err("#[-] expected numeric".into());
    }

    // If one argument, invert and return
    if args.len() == 1 {
        return match args[0] {
            Expr::Int(x) => Ok(Expr::from(1.0f64 / x as f64)),
            Expr::Flt(x) => Ok(Expr::from(1.0f64 / x)),
            _ => Err("invalid type".into())
        }
    }

    let int_div = |ints: &[i64]| {
        ints[1..].iter()
            .map(|&x| if x == 0i64 { Err("division by zero".into()) } else { Ok(x) })
            .fold_results(ints[0], Div::div)
    };

    numeric_op("/", args,
        int_div,
        |floats| Ok(floats[1..].iter().fold(floats[0], Div::div))
    )
}

pub fn equal(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[=]", args, 2)?;
    Ok(Expr::from(args[0] == args[1]))
}

pub fn less(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[<]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a < b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a < b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a < b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn less_eq(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[<=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a <= b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a <= b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a <= b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn greater(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[>]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a > b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a > b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a > b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn greater_eq(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[>=]", args, 2)?;
    match (&args[0], &args[1]) {
        (&Expr::Int(ref a), &Expr::Int(ref b)) => Ok(Expr::from(a >= b)),
        (&Expr::Flt(ref a), &Expr::Flt(ref b)) => Ok(Expr::from(a >= b)),
        (&Expr::Str(ref a), &Expr::Str(ref b)) => Ok(Expr::from(a >= b)),
        _ => Err(
            format!("comparison undefined for: {}, {}", args[0], args[1]).into(),
        ),
    }
}

pub fn not(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("[not]", args, 1)?;
    Ok(Expr::from(!args[0].truthiness()))
}

// TODO: convert to special form
pub fn and(args: &[Expr], _env: &Env) -> Result<Expr> {
    args
        .into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[and] expected boolean argument".into())
        .map(|bools| bools.iter().all(|b| *b))
        .map(Expr::from)
}

// TODO: convert to special form
pub fn or(args: &[Expr], _env: &Env) -> Result<Expr> {
    args
        .into_iter()
        .map(|a| a.boolean())
        .collect::<Option<Vec<_>>>()
        .ok_or("#[or] expected boolean argument".into())
        .map(|bools| bools.iter().any(|b| *b))
        .map(Expr::from)
}

pub fn print(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("#[print]", args, 1)?;
    println!("{}", args[0]);
    Ok(Expr::Nil)
}

pub fn first(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("#[first]", args, 1)?;

    match &args[0] {
        &Expr::List(ref l) => Ok(l.0.first().cloned().unwrap_or(Expr::Nil)),
        &Expr::Vector(ref q) => Ok(q.0.first().cloned().unwrap_or(Expr::Nil)),
        _ => Err("#[first] expected list".into()),
    }
}

pub fn rest(args: &[Expr], _env: &Env) -> Result<Expr> {
    match &args[0] {
        &Expr::List(ref l) => {
            Ok(
                l.0
                    .split_first()
                    .map(|(_, rest)| Expr::List(List(rest.to_vec())))
                    .unwrap_or(Expr::Nil),
            )
        }
        &Expr::Vector(ref v) => {
            Ok(
                v.0
                    .split_first()
                    .map(|(_, rest)| Expr::Vector(Vector(rest.to_vec())))
                    .unwrap_or(Expr::Nil),
            )
        }
        _ => Err("#[rest] expected list".into()),
    }
}

// (cons item seq)
pub fn cons(args: &[Expr], _env: &Env) -> Result<Expr> {
    ensure_args("cons]", args, 2)?;

    match &args[2] {
        &Expr::List(ref l) => {
            let mut new = l.clone();
            new.0.insert(0, args[1].clone());
            Ok(Expr::List(new))
        }
        &Expr::Vector(ref v) => {
            let mut new = v.clone();
            new.0.push(args[1].clone());
            Ok(Expr::Vector(new))
        }
        _ => Err("#[cons] expected list".into()),
    }
}

// (exit)
pub fn exit(_args: &[Expr], _env: &Env) -> Result<Expr> {
    Err(ErrorKind::Exit(0).into())
}
