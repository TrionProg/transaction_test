use std::cell::UnsafeCell;

use super::transaction::Transaction;
use super::transaction::ModifierTrait;

pub struct FieldView<F:'static, O:'static+ObjectViewTrait> {
    inner:UnsafeCell<InnerFieldView<F, O>>
}
struct InnerFieldView<F:'static, O:'static+ObjectViewTrait>{
    offset:usize,
    value:&'static F,
    transaction:&'static Transaction,
    object:&'static O,
    //object? or resourceAddress
    //modifiers
}

impl<F:'static, O:'static+ObjectViewTrait> FieldView<F, O> {
    pub fn new(object:&O, offset:usize, value:&F, transaction:&Transaction) -> Self {
        let inner=InnerFieldView {
            offset,
            value:unsafe{&*(value as *const F)},
            transaction:unsafe{&*(transaction as *const Transaction)},
            object:unsafe{&*(object as *const O)},
        };

        FieldView {
            inner:UnsafeCell::new(inner)
        }
    }

    pub fn get(&self) -> &F {
        let field_view=unsafe{ &mut *self.inner.get() };
        //TODO if is modifiers
        field_view.value
    }

    pub fn add_modifer(&self, modifier:Box<ModifierTrait>) {
        let field_view=unsafe{ &mut *self.inner.get() };

        let modifier_address=field_view.transaction.add_modifier(modifier);
        field_view.object.add_modifier(field_view.offset, modifier_address);
    }
}

pub struct ObjectView<O:ObjectViewTrait> {
    inner:UnsafeCell<O>,
}

impl<O:ObjectViewTrait> ObjectView<O> {
    pub fn add_modifier(&self, offset:usize, modifier_address:&Box<ModifierTrait>) {
        let object_view=unsafe{ &mut *self.inner.get() };

        object_view.add_modifier(offset, modifier_address);//TODO
    }
}

pub trait ObjectViewTrait:Sized {
    fn add_modifier(&mut self, offset:usize, modifier_address:&Box<ModifierTrait>);
}