/// The time of cache expiration.
///
/// # Examples
///
/// ```
/// use pocketcache::time::Expiration;
///
/// let expiration = Expiration::Second(30); // 30ces
/// let expiration = Expiration::Minute(5); // 5min
/// let expiration = Expiration::Hour(3); // 3 hour
/// let expiration = Expiration::Default; // 1 hour
/// ```
#[derive(Debug, Clone)]
pub enum Expiration {
    Second(u64),
    Minute(u64),
    Hour(u64),
    Default,
}

impl Expiration {
    pub fn to_sec(&self) -> u64 {
        match *self {
            Expiration::Second(count) => count,
            Expiration::Minute(count) => count * Expiration::Second(60).to_sec(),
            Expiration::Hour(count) => count * Expiration::Minute(60).to_sec(),
            Expiration::Default => Expiration::Hour(1).to_sec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_sec() {
        assert_eq!(Expiration::Second(10).to_sec(), 10);
        assert_eq!(Expiration::Second(300).to_sec(), 300);
        assert_eq!(Expiration::Minute(1).to_sec(), 60);
        assert_eq!(Expiration::Minute(20).to_sec(), 1200);
        assert_eq!(Expiration::Hour(1).to_sec(), 3600);
        assert_eq!(Expiration::Hour(24).to_sec(), 86400);
        assert_eq!(Expiration::Default.to_sec(), Expiration::Hour(1).to_sec());
    }
}
