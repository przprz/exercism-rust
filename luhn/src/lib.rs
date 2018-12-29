/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|&ch| !ch.is_whitespace())
        .rev()
        .try_fold((0, 0), |(count, sum), ch| {
            ch.to_digit(10)
                .map(|n| if count % 2 == 1 { 2 * n } else { n })
                .map(|n| if n > 9 { n - 9 } else { n })
                .map(|n| (count + 1, sum + n))
        }).map_or(false, |(count, sum)| sum % 10 == 0 && count > 1)
}

