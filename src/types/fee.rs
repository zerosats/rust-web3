use crate::types::U256;
use serde::{Deserialize, Serialize};

/// Gas estimation response for Optimism, including L2 gas and L1 diff size.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RollupGasEstimate {
    /// Estimated L2 gas
    pub gas: U256,
    /// Estimated L1 diff size in bytes
    pub l1_diff_size: U256,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rollup_gas_estimate() {
        let gas_estimate = RollupGasEstimate {
            gas: 0x5208u64.into(),
            l1_diff_size: 0x11u64.into(),
        };

        let serialized = serde_json::to_value(gas_estimate.clone()).unwrap();
        assert_eq!(serialized.to_string(), "{\"gas\":\"0x5208\",\"l1DiffSize\":\"0x11\"}");

        let deserialized = serde_json::from_value(serialized).unwrap();
        assert_eq!(gas_estimate, deserialized);
    }
}