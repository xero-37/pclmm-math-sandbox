//! # AMM Math Sandbox
//!
//! WARNING: While actual Solana smart contracts strictly ban floating-point numbers (f64)
//! to ensure determinism and avoid precision issues, this specific off-chain simulator
//! uses f64 to easily translate standard algebraic formulas for learning purposes.
//! //! I intentionally took examples out of the sessions , so please keep that in mind.

#[allow(unused_variables)]
pub mod v2_math {
    /// Calculates the output amount for a swap given reserves and input.
    /// Formula: dy = (y * dx * (1 - fee)) / (x + dx * (1 - fee))
    pub fn calculate_swap_output(
        reserve_x: f64,
        reserve_y: f64,
        input_x: f64,
        fee_percentage: f64,
    ) -> f64 {
        // todo!("Implement V2 swap logic")
        return (reserve_y*input_x*(1.0-fee_percentage))/(reserve_x+input_x*(1.0-fee_percentage));
    }

    /// Calculates the active capital (L) for a given k and price range.
    pub fn calculate_active_capital(k: f64, p_lower: f64, p_upper: f64) -> f64 {
        // todo!("Implement active capital formula")
        /*The x reserve fluctuates between x_2 and x_1. In stable coin V2 model, only 5% of the capital moves rest 99.5% is 
        sitting idle. I am not sure what to return in this function,since active capital is defined for Uniswap V3 model, so I returned
        liquidity formula of V3 model*/
        let x_1=(k/p_lower).sqrt();
        let x_2=(k/p_upper).sqrt();
        return k.sqrt();
    }
}

#[allow(unused_variables)]
pub mod v3_math {
    /// Calculates real reserves for a given liquidity depth and price range.
    pub fn calculate_real_reserves(
        l_depth: f64,
        current_price: f64,
        p_a: f64,
        p_b: f64,
    ) -> (f64, f64) {
        // todo!("Implement V3 real reserves")
        return (l_depth*((1.0/current_price.sqrt())-(1.0/p_b.sqrt())),l_depth*(current_price.sqrt()-p_a.sqrt()));
    }
}

#[cfg(test)]
mod tests {
    use super::v2_math::*;

    #[test]
    fn test_v2_swap() {
        let reserve_x = 100.0;
        let reserve_y = 15000.0;
        let input_x = 10.0;
        let fee_percentage = 0.003;

        let output = calculate_swap_output(reserve_x, reserve_y, input_x, fee_percentage);

        // Assert that the output is approximately 1359.92
        assert!(
            (output - 1359.92).abs() < 0.01,
            "Expected ~1359.92, got {}",
            output
        );
    }

    #[test]
    fn test_active_capital() {
        // todo!("Implement test for active capital")
        let k=1000000000000.0;
        let p_lower=0.99;
        let p_upper=1.01;
        let output=calculate_active_capital(k,p_lower,p_upper);
        assert!(output==1000000.0);
    }

    #[test]
    fn test_real_reserves() {
        // todo!("Implement test for real reserves")
        let l_depth=1000000.0;
        let current_price=1.0;
        let p_a=0.99;
        let p_b=1.01;
        let (x,y) = calculate_real_reserves(l_depth,current_price,p_a,p_b);
        //Assert that x_real is 4962.81 and y_real is 5012.56 approx.
        assert!((x-4962.81).abs()<0.01 && (y-5012.56).abs()<0.01);
    }
}
