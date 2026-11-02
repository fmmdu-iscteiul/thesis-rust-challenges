pub fn parse_employee_id(input: &str) -> Result<u32, String> {
    Ok(input.trim().parse::<u32>().unwrap())
}
