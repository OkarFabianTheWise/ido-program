// Helper functions
pub fn calculate_vested_amount(
    cliff_end: i64,
    vesting_end: i64,
    current_time: i64,
    total_amount: u64,
) -> u64 {
    if current_time < cliff_end {
        0
    } else if current_time >= vesting_end {
        total_amount
    } else {
        let elapsed = current_time - cliff_end;
        let total_duration = vesting_end - cliff_end;
        ((elapsed as u128 * total_amount as u128) / total_duration as u128) as u64
    }
}
