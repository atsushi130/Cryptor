
// Copyright (c) 2017 Atsushi Miyake
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or http://apache.org/licenses/LICENSE-2.0>
// or the MIT license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to those terms.

pub enum IndexResult {
    Exist(usize),
    None
}

pub trait GetIndex<T: PartialEq> {
    fn get_index(&self, value: &T) -> IndexResult;
}

impl<T: PartialEq> GetIndex<T> for Vec<T> {
    /// Get index of Vector<T>
    fn get_index(&self, value: &T) -> IndexResult {
        use self::IndexResult::*;
        for (index, element) in self.iter().enumerate() {
            if element == value {
                return Exist(index);
            }
        }

        return None
    }
}
