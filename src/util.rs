pub fn amount_pretty(amount: u32) -> String {
    format!("${:.02}", (amount) as f32 / 100.0)
}

pub fn parse_cash_value(input_amount: String) -> Result<u32, ()> {
    let sides: Vec<String> = input_amount
        .split(".")
        .map(|chunk| format!("{:0>1}", chunk))
        .collect();

    // sides cannot be > 2, e.g. 2.00.30

    if sides.len() == 2 {
        // try as float
        if let Ok(dollars) = sides[0].parse::<u32>() {
            if let Ok(cents) = format!("{:.2}", sides[1]).parse::<u32>() {
                return Ok(dollars * 100 + cents);
            }
        }
    } else if sides.len() < 2 {
        // try as u32
        if let Ok(total) = sides[0].parse() {
            return Ok(total);
        }
    }

    Err(())
}
