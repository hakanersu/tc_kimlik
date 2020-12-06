use rand::Rng;

pub fn validate(tc: &String) -> bool {
    let parts = number_to_vec(&tc);
    
    if parts.len() < 11 {
        return false
    }

    let even: u32 = parts.iter().take(9).step_by(2).sum();
    let odd: u32 = parts[1..9].iter().step_by(2).sum();
  
    let ten: u32 = ( even * 7 - odd) % 10;

    if parts.get(9).is_none() || parts[9] != ten {
        return false
    }

    let eleven: u32 = parts[0..10].iter().sum::<u32>() % 10;

    if parts.get(10).is_none() || parts[10] != eleven {
        return false
    }

    true
}
pub fn generate() -> String {
    let mut rng = rand::thread_rng();
    let random: u32 = rng.gen_range(100000000, 999999999);
    let parts = number_to_vec(&random.to_string());
    let even: u32 = parts.iter().take(9).step_by(2).sum();
    let odd: u32 = parts[1..9].iter().step_by(2).sum();
    let ten: u32 = ( even * 7 - odd) % 10;
    let new_number = format!("{}{}", random, ten);
    let new_parts = number_to_vec(&new_number);
    let eleven: u32 = new_parts[0..10].iter().sum::<u32>() % 10;
    return format!("{}{}{}", random, ten, eleven)
}

fn number_to_vec(n: &String) -> Vec<u32> {
    n.trim().chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect()
}