fn main() {
    let n = 12;

    let divides_by =
        |divisors: Vec<usize>| divisors.iter().all(|d: &usize| n % d == 0);

    println!("{}", divides_by(vec![1, 3]));
    println!("{}", divides_by(vec![1, 5]));
    println!("{}", divides_by(vec![1, 4, 6]));
}