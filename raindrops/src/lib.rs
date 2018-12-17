pub fn raindrops(n: usize) -> String {
    let divides_by =
        |divisors: Vec<usize>| divisors.iter().all(|d: &usize| n % d == 0);

    match n {
        _ if divides_by(vec![3,5,7]) => "PlingPlangPlong".to_string(),
        _ if divides_by(vec![3,5]) => "PlingPlang".to_string(),
        _ if divides_by(vec![3,7]) => "PlingPlong".to_string(),
        _ if divides_by(vec![5,7]) => "PlangPlong".to_string(),
        _ if divides_by(vec![3]) => "Pling".to_string(),
        _ if divides_by(vec![5]) => "Plang".to_string(),
        _ if divides_by(vec![7]) => "Plong".to_string(),
        n => format!("{}", n)
    }
}
