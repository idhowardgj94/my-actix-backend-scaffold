use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::{BTreeMap};

pub fn sign_for_login(c: BTreeMap<String, String>) -> String {
    // TODO config
    let key: Hmac<Sha256> = Hmac::new_varkey(b"howardishandsome").unwrap();
    c.sign_with_key(&key).unwrap()
}


pub fn verification(t: String) -> Option<BTreeMap<String, String>> {
    // TODO config
    let key: Hmac<Sha256> = Hmac::new_varkey(b"howardishandsome").unwrap();
    let claims: BTreeMap<String, String> = t.as_str().verify_with_key(&key).unwrap();
    Some(claims)
}

#[cfg(test)]
mod jwt_test {
    use hmac::{Hmac, NewMac};
    use jwt::{SignWithKey, VerifyWithKey};
    use sha2::Sha256;
    use std::collections::BTreeMap;
    #[test]
    pub fn test_sign() {
        // 簽名
        let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("sub", "someone");
        let token_str = claims.sign_with_key(&key).unwrap();

        assert_eq!(token_str, "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0");
    }

    #[test]
    pub fn test_verifi() {
        let key: Hmac<Sha256> = Hmac::new_varkey(b"some-secret").unwrap();
        // 取得 key
        let token_str = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJzb21lb25lIn0.5wwE1sBrs-vftww_BGIuTVDeHtc1Jsjo-fiHhDwR8m0";

        let claims: BTreeMap<String, String> = token_str.verify_with_key(&key).unwrap();

        assert_eq!(claims["sub"], "someone");
    }

}

