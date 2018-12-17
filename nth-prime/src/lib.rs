pub fn nth(n: usize) -> Result<usize, ()> {
    // NOTE: this solution doesn't work
    //  I have yet to find why.

    // math taken from: https://primes.utm.edu/notes/faq/p_n.html for details
    //
    // nth prime = 1 + (sum from m=1 to j=2^n) [ [ n/(1 + pi(m)) ] ^ 1/n ]
    // where:
    //   pi(n) = 1 + (sum from j=3 to n) ( (j-2)! - j[(j-2)!/j] ).
    //
    let r = (1 .. 1 + 2u32.pow(n as u32))
        .fold(1, |acc: usize, m: u32| 
            (n as f32/(1 + pi(m)) as f32).floor()
            .powf(1f32/n as f32)
            .floor() as usize
                + acc 
        );

    Ok(r)
}

// pi(3) == 1 + 1 = 2
// pi(4) == 2 + (2 - 4*(0)) = 4
// pi(5) == 4 + (6 - 5*(1)) = 5
// pi(6) == 5 + (24 - 6*(4)) = 5
pub fn pi(n: u32) -> i32 {
    if n == 1 { return 0; }
    if n == 2 { return 1; }

    (3 .. n + 1).fold(1, |acc: i32, j: u32|
        acc as i32 + (
            fact(j-2) as i32 -
                j as i32 * ((fact(j-2) as f32)/j as f32).floor() as i32
        )
    )
}

fn fact(n: u32) -> u32 {
    (2 .. n + 1).fold(1, |acc: u32, i: u32| acc * i)
}

