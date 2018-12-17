const ISBN_LENGTH: usize = 10;

#[derive(Debug)]
enum Isbn {
    Invalid,
}

/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let chars: Vec<char> = isbn.chars().filter(|&ch| ch != '-').collect();

    if chars.len() != ISBN_LENGTH {
        return false;
    }

    let sum = chars.iter().rev().fuse().enumerate()
        .map(|(i, &ch)|
            calculate_value(i, ch).ok_or_else(|| Isbn::Invalid)
        )
        .fold(Ok(0), |a, x|
            a.and_then(|a| x.map(|x| a + x)),
        );

    match sum {
        Ok(n) => n % 11 == 0,
        Err(_) => false
    }
}

fn calculate_value(idx: usize, ch: char) -> Option<u32> {
    println!("{}", idx);
    let value = if ch == 'X' {
        if idx != 0 { return None; };
        10
    } else {
        match ch.to_digit(10) {
            Some(n) => n,
            None => { return None; }
        }
    };

    let multiplier = (idx + 1) as u32;

    Some(multiplier * value)
}
