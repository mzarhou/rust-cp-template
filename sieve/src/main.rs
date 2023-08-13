fn sieve(n: usize) -> Vec<bool> {
    let mut v = vec![true; n + 1];
    v[0] = false;
    v[1] = false;

    for i in 2..=n {
        if !v[i] {
            continue;
        }
        for j in (i * i..=n).step_by(i) {
            v[j] = false;
        }
    }
    return v;
}

fn main() {
    println!(
        "{}",
        sieve(20)
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
