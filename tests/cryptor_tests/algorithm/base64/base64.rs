
mod base64_tests {

    use cryptor::cryptor::{ Base64, Algorithm };

    #[test]
    fn encryptable() {
        let mut base64 = Base64;
        let result = base64.encrypt("A");
        assert_eq!("QQ==", result.text);
    }

    #[test]
    fn decryptable() {
        let mut base64 = Base64;
        let result = base64.decrypt("QQ==");
        assert_eq!("A", result.text);
    }
}