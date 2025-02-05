pub fn prompt(message_prompt: &str) -> anyhow::Result<String> {
    println!("{}", message_prompt);

    let mut message = String::new();
    std::io::stdin().read_line(&mut message)?;

    if message.is_empty() {
        return Err(anyhow::anyhow!("Empty input"));
    }

    Ok(message.trim().to_string())
}
