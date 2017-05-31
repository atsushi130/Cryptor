
mod base64_tests {

    use cryptor::cryptor::{ Base64, Algorithm };

    #[test]
    fn encryptable() {
        let mut base64 = Base64;
        match base64.encrypt(&"A") {
            Ok(ref result) => assert_eq!("QQ==", result.text),
            Err(_)         => assert!(false)
        }
    }

    #[test]
    fn decryptable() {
        let mut base64 = Base64;
        match base64.decrypt("QQ==") {
            Ok(ref result) => assert_eq!("A", result.text),
            Err(_)         => assert!(false)
        }
    }
}