use pclmm_math_sandbox::v2_math;
use pclmm_math_sandbox::v3_math;

fn main() {
    println!("--- AMM Math Sandbox ---");

    // you can add CLI logic or manual simulations here.

    // Example parameters for simulation
    let r_x = 100.0;
    let r_y = 15000.0;
    let amount_in = 10.0;
    let fee = 0.003;

    println!(
        "Simulation inputs: ReserveX={}, ReserveY={}, InX={}, Fee={}",
        r_x, r_y, amount_in, fee
    );

    // The following call will panic until implemented in lib.rs
    // let amount_out = v2_math::calculate_swap_output(r_x, r_y, amount_in, fee);
    // println!("Swap Result: {} Y", amount_out);
}
