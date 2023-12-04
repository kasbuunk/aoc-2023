pub fn solution(data: &str) -> Result<(u32, u32), Box<dyn std::error::Error>> {
    let sum_a = puzzle_a(&data)?;
    let sum_b = puzzle_b(&data)?;

    Ok((sum_a, sum_b))
}

fn puzzle_a(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(0)
}

fn puzzle_b(data: &str) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(0)
}
