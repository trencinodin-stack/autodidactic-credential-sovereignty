#![no_std]

#[derive(Debug, PartialEq, Eq)]
pub enum ProtocolStatus {
    ValidTransition,
    ReentrancyBreach,
    SupplyDesync,
    ReplayDetected,
}

pub fn evaluate_state_transition(
    is_locked: bool,
    supply_delta: i64,
    nonce_seen: bool,
) -> ProtocolStatus {
    if is_locked {
        return ProtocolStatus::ReentrancyBreach;
    }
    if nonce_seen {
        return ProtocolStatus::ReplayDetected;
    }
    if supply_delta != 0 {
        return ProtocolStatus::SupplyDesync;
    }
    ProtocolStatus::ValidTransition
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transition() {
        assert_eq!(evaluate_state_transition(false, 0, false), ProtocolStatus::ValidTransition);
    }

    #[test]
    fn test_reentrancy_intercept() {
        assert_eq!(evaluate_state_transition(true, 0, false), ProtocolStatus::ReentrancyBreach);
    }

    #[test]
    fn test_replay_nonce_attack() {
        assert_eq!(evaluate_state_transition(false, 0, true), ProtocolStatus::ReplayDetected);
    }

    #[test]
    fn test_supply_conservation_fail() {
        assert_eq!(evaluate_state_transition(false, 100, false), ProtocolStatus::SupplyDesync);
    }
}
