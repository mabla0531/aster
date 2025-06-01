pub fn amount_pretty(amount: u32) -> String {
    format!("${:.02}", (amount) as f32 / 100.0)
}