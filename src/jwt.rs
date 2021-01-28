#[cfg(test)]
mod jwt_test {
    use hmac::{Hmac, NewMac};
    use jwt::SignWithKey;
    use sha2::Sha256;
    use std::collections::BTreeMap;
    #[test]
    pub fn test_jwt() {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("sub", "someone");
        let token_str = claims.sign_with_key(&key).unwrap();

        assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");
    }

}