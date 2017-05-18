
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
