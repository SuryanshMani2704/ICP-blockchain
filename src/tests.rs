#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_tokens() {
        // Initialize the state
        init("ICP".to_string(), "Internet Computer Protocol".to_string(), Nat::from(1000));

        // Test sending tokens
        assert_eq!(send("owner".to_string(), "recipient".to_string(), Nat::from(100)).is_ok(), true);
        assert_eq!(balance_of("owner".to_string()), Nat::from(900));
        assert_eq!(balance_of("recipient".to_string()), Nat::from(100));
    }
}
