#![no_std]

pub const MAX_TICK_TO_TRADE_NS: u64 = 450;
pub const MAX_DRAWDOWN_BPS: u32 = 150; // 1.5%

#[derive(Debug, PartialEq, Eq)]
pub enum QuantStatus {
    Pass,
    LatencyBreach,
    DrawdownViolation,
    StateDesync,
}

pub fn evaluate_trade_action(
    tick_to_trade_ns: u64,
    drawdown_bps: u32,
    balance_delta: i64,
) -> QuantStatus {
    if balance_delta != 0 {
        return QuantStatus::StateDesync;
    }
    if drawdown_bps > MAX_DRAWDOWN_BPS {
        return QuantStatus::DrawdownViolation;
    }
    if tick_to_trade_ns > MAX_TICK_TO_TRADE_NS {
        return QuantStatus::LatencyBreach;
    }
    QuantStatus::Pass
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_trade() {
        assert_eq!(evaluate_trade_action(350, 100, 0), QuantStatus::Pass);
    }

    #[test]
    fn test_tick_latency_breach() {
        assert_eq!(evaluate_trade_action(500, 100, 0), QuantStatus::LatencyBreach);
    }

    #[test]
    fn test_drawdown_breach() {
        assert_eq!(evaluate_trade_action(300, 160, 0), QuantStatus::DrawdownViolation);
    }

    #[test]
    fn test_balance_unconserved() {
        assert_eq!(evaluate_trade_action(300, 50, -1), QuantStatus::StateDesync);
    }
}
