/// Calcula el Máximo Común Divisor (MCD)
pub fn mcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { mcd(b, a % b) }
}

/// Calcula el Mínimo Común Múltiplo (MCM)
pub fn mcm(a: u64, b: u64) -> u64 {
    (a * b) / mcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mcd_basic() {
        assert_eq!(mcd(12, 8), 4);
        assert_eq!(mcd(54, 24), 6);
        assert_eq!(mcd(48, 18), 6);
    }

    #[test]
    fn test_mcd_with_zero() {
        assert_eq!(mcd(0, 5), 5);
        assert_eq!(mcd(10, 0), 10);
    }

    #[test]
    fn test_mcd_same_number() {
        assert_eq!(mcd(7, 7), 7);
        assert_eq!(mcd(0, 0), 0);
    }

    #[test]
    fn test_mcd_coprime() {
        assert_eq!(mcd(13, 7), 1);
        assert_eq!(mcd(17, 19), 1);
    }

    #[test]
    fn test_mcm_basic() {
        assert_eq!(mcm(4, 6), 12);
        assert_eq!(mcm(8, 12), 24);
        assert_eq!(mcm(5, 7), 35);
    }

    #[test]
    fn test_mcm_with_common_divisor() {
        assert_eq!(mcm(12, 18), 36);
        assert_eq!(mcm(15, 25), 75);
    }

    #[test]
    fn test_mcm_same_number() {
        assert_eq!(mcm(7, 7), 7);
    }

    #[test]
    fn test_mcm_with_one() {
        assert_eq!(mcm(1, 5), 5);
        assert_eq!(mcm(10, 1), 10);
    }
}
