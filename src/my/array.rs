
use super::resource::ResourceTrait;
use super::resource::ResourceReference;
use super::resource::FieldTrait;

pub struct Array<T> {
    vec:Vec<T>
}

impl<T> Array<T> {
    pub fn new() -> Self {
        Array {
            vec:Vec::new()
        }
    }

    //pub fn add
}

impl<T> FieldTrait for Array<T> {

}