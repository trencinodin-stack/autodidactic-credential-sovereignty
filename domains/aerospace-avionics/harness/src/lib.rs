#![no_std]

pub const MAX_RECOVERY_MS: u32 = 30;
pub const MAX_ALLOC_BYTES: usize = 0;
pub const MAX_STARVATION_TICKS: u32 = 3;

#[derive(Debug, PartialEq, Eq)]
pub enum AvionicsStatus {
    NominalFlight,
    DynamicAllocDetected,
    RecoveryWindowExceeded,
    TelemetryStarvation,
}

pub fn evaluate_flight_loop(
    recovery_ms: u32,
    alloc_bytes: usize,
    starvation_ticks: u32,
) -> AvionicsStatus {
    if alloc_bytes > MAX_ALLOC_BYTES {
        return AvionicsStatus::DynamicAllocDetected;
    }
    if starvation_ticks > MAX_STARVATION_TICKS {
        return AvionicsStatus::TelemetryStarvation;
    }
    if recovery_ms > MAX_RECOVERY_MS {
        return AvionicsStatus::RecoveryWindowExceeded;
    }
    AvionicsStatus::NominalFlight
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_flight() {
        assert_eq!(evaluate_flight_loop(15, 0, 1), AvionicsStatus::NominalFlight);
    }

    #[test]
    fn test_alloc_in_flight() {
        assert_eq!(evaluate_flight_loop(15, 4, 1), AvionicsStatus::DynamicAllocDetected);
    }

    #[test]
    fn test_recovery_timeout() {
        assert_eq!(evaluate_flight_loop(35, 0, 1), AvionicsStatus::RecoveryWindowExceeded);
    }

    #[test]
    fn test_telemetry_starvation() {
        assert_eq!(evaluate_flight_loop(10, 0, 5), AvionicsStatus::TelemetryStarvation);
    }
}
