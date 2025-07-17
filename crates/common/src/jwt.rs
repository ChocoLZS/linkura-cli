use anyhow::Result;
use base64::{Engine as _, engine::general_purpose};

pub fn extract_jwt_payload(jwt: &str) -> Result<serde_json::Value> {
    let parts: Vec<&str> = jwt.split('.').collect();
    if parts.len() != 3 {
        return Err(anyhow::anyhow!("Invalid JWT format"));
    }
    let payload = parts[1];
    let decoded_payload = general_purpose::URL_SAFE_NO_PAD
        .decode(payload)
        .or_else(|_| general_purpose::URL_SAFE.decode(payload))?;
    let json_payload: serde_json::Value = serde_json::from_slice(&decoded_payload)?;
    Ok(json_payload)
}

#[cfg(test)]
mod tests {
    use super::*;
    const JWT: &'static str = "eyJhbGciOiJIUzUxMiIsInR5cCI6IkpXVCJ9.eyJzZXJ2aWNlX2RvbWFpbiI6Imh0dHBzOi8vYXBpLmxpbmstbGlrZS1sb3ZlbGl2ZS5hcHAiLCJsaW5rX2xpa2VfaWQiOiJBQUFBQUFBQUEiLCJyb29tX2lkIjoiZGVmYXVsdC1mYWNiZGE1MS1iYjlkLTQyNjctYjRhYi01ZWYzYzg3OGJhZWMiLCJyb2xlIjoiYXVkaWVuY2UiLCJwb2QiOnsicm9sZSI6ImF1ZGllbmNlIiwic2NoZW1lIjoidGNwIiwiYWRkcmVzcyI6IjEwLjExNC41MTQuMTkxIiwicG9ydCI6OTgxMH0sImlzcyI6Imh0dHBzOi8vYXBpLmxpbmstbGlrZS1sb3ZlbGl2ZS5hcHAiLCJzdWIiOiJBQUFBQUFBQUEiLCJhdWQiOlsiQUFBQUFBQUFBIl0sImV4cCI6MTc0ODUxODU3NSwibmJmIjoxNzQ4NTE4NTYwLCJpYXQiOjE3NDg1MTg1NjB9.eddiZjzEH_I88w9lmOVBr2Z4BWShIv6yeM9TPZvKIts5rmPFwvBbJEKffkobXglOuUBp80svLoufyzOM_YSmDg";
    #[test]
    fn test_extract_jwt_payload() {
        let jwt = JWT;
        let result = extract_jwt_payload(jwt);
        assert!(result.is_ok());
    }

    #[test]
    fn test_extract_jwt_payload_invalid() {
        let jwt = "invalid.jwt";
        let result = extract_jwt_payload(jwt);
        assert!(result.is_err());
    }

    #[test]
    fn test_jwt_payload_base64_decoding() {
        let payload = extract_jwt_payload(JWT).unwrap();
        assert_eq!(
            payload["service_domain"].as_str().unwrap(),
            "https://api.link-like-lovelive.app"
        );
        assert_eq!(payload["link_like_id"].as_str().unwrap(), "AAAAAAAAA");
        assert_eq!(
            payload["room_id"].as_str().unwrap(),
            "default-facbda51-bb9d-4267-b4ab-5ef3c878baec"
        );
        assert_eq!(payload["pod"]["role"].as_str().unwrap(), "audience");
        assert_eq!(payload["pod"]["scheme"].as_str().unwrap(), "tcp");
        assert_eq!(
            payload["pod"]["address"].as_str().unwrap(),
            "10.114.514.191"
        );
        assert_eq!(payload["pod"]["port"].as_u64().unwrap(), 9810);
        assert_eq!(
            payload["iss"].as_str().unwrap(),
            "https://api.link-like-lovelive.app"
        );
        assert_eq!(payload["sub"].as_str().unwrap(), "AAAAAAAAA");
        assert_eq!(
            payload["aud"].as_array().unwrap()[0].as_str().unwrap(),
            "AAAAAAAAA"
        );
        assert_eq!(payload["exp"].as_u64().unwrap(), 1748518575);
        assert_eq!(payload["nbf"].as_u64().unwrap(), 1748518560);
        assert_eq!(payload["iat"].as_u64().unwrap(), 1748518560);
    }
}
