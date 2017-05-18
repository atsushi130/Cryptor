
/// enigma module
use super::{ Router, RouterProtocol, Digit };

pub struct RouterManager {
    pub routers: Vec<Router>
}

impl RouterManager {

    pub fn encrypt_to_reflector(&self, character: &char) -> char {

        let mut encrypt = *character;
        for router in &self.routers {
            encrypt = router.encrypt(&encrypt);
        }

        return encrypt
    }

    pub fn encrypt_from_reflector(&self, character: &char) -> char {

        let mut encrypt = *character;
        for router in self.routers.iter().rev() {
            encrypt = router.reverse(&encrypt);
        }

        return encrypt
    }

    pub fn increment(&mut self) {
        for router in &mut self.routers {
            match router.increment() {
                Digit::Up       => continue,
                Digit::NoChange => break
            }
        }
    }
}