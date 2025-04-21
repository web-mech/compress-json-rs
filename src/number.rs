const ITO_S: &str = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
const N: usize = ITO_S.len();

/// Convert base-N string to integer index
pub fn s_to_int(s: &str) -> usize {
    let mut acc = 0;
    let mut pow = 1;
    for c in s.chars().rev() {
        let idx = ITO_S
            .find(c)
            .expect("invalid character in s_to_int");
        acc += idx * pow;
        pow *= N;
    }
    acc
}

/// Convert integer to base-N string (unused)
#[allow(dead_code)]
pub fn int_to_s(value: usize) -> String {
    if value == 0 {
        return ITO_S.chars().next().unwrap().to_string();
    }
    let mut val = value;
    let mut acc = Vec::new();
    while val != 0 {
        let i = val % N;
        let c = ITO_S.chars().nth(i).unwrap();
        acc.push(c);
        val /= N;
    }
    acc.iter().rev().collect()
}

/// Reverse a string (unused)
#[allow(dead_code)]
fn reverse(s: &str) -> String {
    s.chars().rev().collect()
}