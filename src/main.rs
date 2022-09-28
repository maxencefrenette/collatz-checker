fn main() {
    let mut n: u64 = 1;
    let mut residues: Vec<u64> = vec![0];

    for _ in 0..32 {
        residues.extend(residues.clone().iter().map(|r| n + r));
        n *= 2;

        residues = residues
            .into_iter()
            .filter(|r| !check_collatz(n, *r))
            .collect();

        println!("Residues mod {}: {}", n, residues.len());
    }
}

/// Checks if numbers of the form a*k + b satisfy the collatz conjecture for all k
fn check_collatz(a: u64, b: u64) -> bool {
    let a_start = a;
    let mut a = a;
    let mut b = b;

    while a >= a_start {
        if a % 2 == 1 {
            return false;
        }

        if b % 2 == 0 {
            a /= 2;
            b /= 2;
        } else {
            a *= 3;
            b = 3 * b + 1;
        }
    }

    return true;
}
