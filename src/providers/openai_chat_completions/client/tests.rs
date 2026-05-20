use super::types::Usage;

#[test]
fn usage_deserializes_when_detail_fields_are_missing() {
    let usage: Usage = serde_json::from_str(
        r#"{
            "prompt_tokens": 100,
            "completion_tokens": 50,
            "total_tokens": 150,
            "prompt_tokens_details": {},
            "completion_tokens_details": {}
        }"#,
    )
    .expect("usage should deserialize when optional detail fields are omitted");

    assert_eq!(usage.prompt_tokens_details.unwrap().cached_tokens, 0);
    assert_eq!(usage.completion_tokens_details.unwrap().reasoning_tokens, 0);
}
