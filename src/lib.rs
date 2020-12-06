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

fn number_to_vec(n: &String) -> Vec<u32> {
    n.trim().chars()
    .map(|c| c.to_digit(10).unwrap())
    .collect()
}