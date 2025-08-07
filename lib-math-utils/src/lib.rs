/// Calcula el Máximo Común Divisor (MCD)
pub fn mcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { mcd(b, a % b) }
}

/// Calcula el Mínimo Común Múltiplo (MCM)
pub fn mcm(a: u64, b: u64) -> u64 {
    (a * b) / mcd(a, b)
}
