use std::collections::HashMap;

fn prime_fact(n: usize) -> Vec<usize> {
    let mut n = n;
    let mut v = vec![];
    while n % 2 == 0 {
        v.push(2);
        n /= 2;
    }
    for i in (3..=(n as f64).sqrt() as usize).step_by(2) {
        while n % i == 0 {
            v.push(i);
            n /= i;
        }
    }
    if n > 2 {
        print!(" {}", n);
        v.push(n);
    }
    return v;
}

fn get_pf_map(n: i32) -> HashMap<i32, i32> {
    let mut n = n;
    let mut map = HashMap::new();
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let p = i;
            let mut e = 0;
            while n % i == 0 {
                e += 1;
                n /= i;
            }
            map.insert(p, e);
        }
        i += 1;
    }
    if n != 1 {
        map.insert(n, 1);
    }
    return map;
}

fn get_pf_vec(n: i32) -> Vec<(i32, i32)> {
    let mut n = n;
    let mut pf_vec = vec![];
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            let p = i;
            let mut e = 0;
            while n % i == 0 {
                e += 1;
                n /= i;
            }
            pf_vec.push((p, e));
        }
        i += 1;
    }
    if n != 1 {
        pf_vec.push((n, 1));
    }
    return pf_vec;
}

fn main() {
    let pf_m = get_pf_map(100);
    for (key, val) in pf_m {
        println!("{key}^{val}");
    }
    println!("--");
    let pf_vec = get_pf_vec(100);
    for (key, val) in pf_vec {
        println!("{key}^{val}");
    }
    println!("--");
    let pf_vec = prime_fact(100);
    for val in pf_vec {
        println!("{val}");
    }
}
