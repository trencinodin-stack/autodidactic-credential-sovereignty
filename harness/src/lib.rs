#![no_std]

pub const MAX_LATENCY_NS: u64 = 11_990_000; // 11.99 ms ceiling
pub const MAX_ALLOC_BYTES: usize = 0;        // M_alloc == 0 floor
pub const MAX_DRIFT_TICKS: u32 = 50;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u8)]
pub enum PosetStatus {
    Pass = 0,
    Freeze = 10,
    Pwc = 12,
    MemoryViolation = 30,
    Refusal = 32,
    Breach = 40,
}

pub fn evaluate_execution(duration_ns: u64, alloc_bytes: usize, drift_ticks: u32) -> PosetStatus {
    if alloc_bytes > MAX_ALLOC_BYTES {
        return PosetStatus::MemoryViolation;
    }
    if duration_ns > MAX_LATENCY_NS {
        return PosetStatus::Refusal;
    }
    if drift_ticks > MAX_DRIFT_TICKS {
        return PosetStatus::Freeze;
    }
    PosetStatus::Pass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_execution() {
        assert_eq!(evaluate_execution(1_000_000, 0, 10), PosetStatus::Pass);
    }

    #[test]
    fn test_memory_violation() {
        assert_eq!(evaluate_execution(1_000_000, 1, 10), PosetStatus::MemoryViolation);
    }

    #[test]
    fn test_latency_ceiling() {
        assert_eq!(evaluate_execution(12_000_000, 0, 10), PosetStatus::Refusal);
    }

    #[test]
    fn test_drift_clamp() {
        assert_eq!(evaluate_execution(1_000_000, 0, 55), PosetStatus::Freeze);
    }
}
