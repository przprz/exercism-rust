use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for i in 1..sum / 2 {
        for j in i..sum / 2 {
            let k = sum - i - j;
            if i.pow(2) + j.pow(2) == k.pow(2) {
                let mut triplet = [i, j, k];
                triplet.sort();
                triplets.insert(triplet);
            }
        }
    }
    triplets
}
