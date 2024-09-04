// `try_fold` and `try_rfold` are similar to their
// brethren `fold` and `rfold`. The difference is that
// they can exit early without accumulating/consuming
// all values of the iterator. This early exit is
// determined by the return value of the accumulator
// function:
//
// * For `Result<T, E>` an `Err(e)` indicates an early
//   exit.
// * For `Option<T>` a `None` indicates an early exit
// * For `std::ops::ControlFlow` `Break(b)` indicates
//   an early exit, `Continue(c)` would be used to
//   keep going.

// NOTE: `ControlFlow` is technically the same as using
// `Result`, using `Result` can bring some unwanted
// implications since `Err` would be used to return early,
// however it doesn't necessarily have to be an actual error
// for it to be the case. If you want to communicate that
// an early return is possible but not an error case, then use
// `Break` instead.
use std::error::Error;
use std::io::prelude::*;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = std::io::stdin();
    let sum = stdin
        .lock()
        .lines()
        .try_fold(0, |sum, line| -> Result<u64, Box<dyn Error>> {
            Ok(sum + u64::from_str(&line?.trim())?)
        })?;
    println!("{}", sum);

    Ok(())
}

// NOTE: `try_fold` is extremely flexible, it's actually used to
// implement many other iterator consumer methods.
