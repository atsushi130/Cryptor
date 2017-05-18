
use std::marker::PhantomData;

/// algorithm module
use super::super::{ Algorithm, EncryptValue };

/// enigma module
use super::{ Router, Reflector, RouterManager, Plugboard };

pub struct Enigma {
    router_manager: RouterManager,
    plugboard: Plugboard,
    reflector: Reflector
}

impl Algorithm for Enigma {
    type V = Enigma;
    fn encrypt(&mut self, character: &char) -> EncryptValue<Self::V> {

        let intput = self.plugboard.input(character);

        let in_encrypted  = self.router_manager.encrypt_to_reflector(&intput);
        let reflected     = self.reflector.reflect(&in_encrypted);
        let out_encrypted = self.router_manager.encrypt_from_reflector(&reflected);

        let output = self.plugboard.output(&out_encrypted);

        let hash_value: EncryptValue<Enigma> = EncryptValue {
            text: output.to_string(),
            phantom: PhantomData {}
        };

        self.router_manager.increment();
        return hash_value
    }
}

impl Enigma {
    pub fn new(routers: Vec<Router>, plugboard: Plugboard, reflector: Reflector) -> Self {
        Enigma {
            router_manager: RouterManager {
                routers: routers
            },
            plugboard,
            reflector
        }
    }
}
