const MAX_BOTTLES: u32 = 99;

pub fn sing(start: i32, end: i32) -> String {
    let vs: Vec<String> = (end..start + 1).rev().map(|n| verse(n)).collect();

    format!("{}", vs.join("\n"))
}

pub fn verse(n: i32) -> String {
    let current = current_amount(n);
    let left = left_after_action(n);

    String::from(format!("{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n", current, current.to_lowercase(), action(n), left))
}

fn left_after_action(n: i32) -> String {
    let left = match n {
        1 => format!("no more"),
        0 => format!("{}", MAX_BOTTLES),
        _ => format!("{}", n - 1)
    };
    let plural = if n == 2 { "" } else { "s" };
    format!("{} bottle{}", left, plural)
}

fn current_amount(n: i32) -> String {
    let amount = match n {
        0 => format!("No more"),
        _ => format!("{}", n)
    };
    let plural = if n == 1 { "" } else { "s" };
    format!("{} bottle{}", amount, plural)
}

fn action(n: i32) -> String {
    match n {
        0 => format!("Go to the store and buy some more"),
        n => format!("Take {} down and pass it around", if n == 1 { "it" } else { "one" }),
    }
}

