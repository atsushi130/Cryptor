
/// encrypter crate
extern crate encrypter;

#[cfg(test)]
mod router_manager_tests {

    use encrypter::domain::{ Router, RouterManager, SubstitutionTable };
    use encrypter::domain::{ ALPHABETS };

    #[test]
    fn incrementable() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.increment();

        let expects = vec![1, 0, 0];
        for i in 0..3 {
            assert_eq!(expects[i], router_manager.routers[i].position);
        }
    }

    #[test]
    fn incrementable_digit_up() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.routers[0].position = 25;
        router_manager.routers[1].position = 25;
        router_manager.routers[2].position = 24;
        router_manager.increment();

        let expects = vec![0, 0, 25];
        for i in 0..3 {
            assert_eq!(expects[i], router_manager.routers[i].position);
        }
    }

    #[test]
    fn encryptable_to_reflector() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        let result = router_manager.encrypt_to_reflector(&'A');
        assert_eq!('A', result);
    }

    #[test]
    fn encryptable_from_reflector() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        let result = router_manager.encrypt_to_reflector(&'A');
        assert_eq!('A', result);
    }

    #[test]
    fn encryptable_to_reflector_after_increment() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.increment();
        let result = router_manager.encrypt_to_reflector(&'A');
        assert_eq!('B', result);
    }

    #[test]
    fn encryptable_from_reflector_after_increment() {

        #![allow(unused_mut)]
        let mut router_manager = RouterManager {
            routers: vec![
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec())),
                Router::new(SubstitutionTable::new(ALPHABETS.to_vec()))
            ]
        };

        router_manager.increment();
        let result = router_manager.encrypt_from_reflector(&'A');
        assert_eq!('Z', result);
    }
}