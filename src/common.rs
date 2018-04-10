use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Point {
    x:f32,
    y:f32
}

impl Point {
    pub fn new(x:f32,y:f32) -> Self {
        Point {
            x,
            y
        }
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    a:Point,
    b:Point
}

impl BoundingBox {
    pub fn new(a:Point, b:Point) -> Self {
        BoundingBox {
            a,
            b
        }
    }
}

#[derive(Clone,Eq,PartialEq)]
pub struct TransactionInfo{
    pub class_code:u32,
    pub id:u32
}

impl TransactionInfo {
    pub fn new(class_code:u32, id:u32) -> Self {
        TransactionInfo {
            class_code,
            id
        }
    }
}

impl Hash for TransactionInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.class_code.hash(state);
        self.id.hash(state);
    }
}