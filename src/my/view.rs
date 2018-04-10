use std::cell::UnsafeCell;

use common::TransactionInfo;

use super::transaction::Transaction;
use super::transaction::ModifierTrait;
use super::access::BitMask;
use super::arbiter::Arbiter;

pub struct FieldView<F, O:ObjectViewTrait> {
    inner:UnsafeCell<InnerFieldView<F, O>>
}

struct InnerFieldView<F, O:ObjectViewTrait>{
    offset:usize,
    value:*const F,
    transaction:*const Transaction,
    object:*const ObjectView<O>,
    //object? or resourceAddress
    //modifiers
}

impl<F, O:ObjectViewTrait> FieldView<F, O> {
    pub fn new(object:*const ObjectView<O>, offset:usize, value:*const F, transaction:&Transaction) -> Self {
        let inner=InnerFieldView {
            offset,
            value,
            transaction:unsafe{(transaction as *const Transaction)},
            object,
        };

        FieldView {
            inner:UnsafeCell::new(inner)
        }
    }

    pub fn get(&self) -> &F {
        let field_view=unsafe{ &mut *self.inner.get() };
        //TODO if is modifiers
        unsafe{&*field_view.value}
    }

    pub fn add_modifer(&self, modifier:Box<ModifierTrait>) {
        let field_view=unsafe{ &mut *self.inner.get() };

        unsafe {
            let modifier_address = (&*field_view.transaction).add_modifier(modifier);
            if (&*field_view.object).add_modifier(field_view.offset, modifier_address) {
                (&*field_view.transaction).object_modified();//TODO
            }
        }
    }
}

pub trait FieldViewTrait {
}

impl FieldViewTrait {
    pub fn downcast_ref_unchecked<F: FieldViewTrait>(&self) -> &F {
        unsafe { &*(self as *const FieldViewTrait as *const F) }
    }
}

impl<F, O:ObjectViewTrait> FieldViewTrait for FieldView<F, O> {}

pub struct ObjectView<O:ObjectViewTrait> {
    inner:UnsafeCell<O>,
}

impl<O:ObjectViewTrait> ObjectView<O> {
    pub fn new(object_view:O) -> Self {
        ObjectView {
            inner:UnsafeCell::new(object_view)
        }
    }

    pub fn get_mut(&self) -> &mut O {
        unsafe{ &mut *self.inner.get() }
    }

    pub fn add_modifier(&self, offset:usize, modifier_address:*const Box<ModifierTrait>) -> bool {
        let object_view=unsafe{ &mut *self.inner.get() };

        object_view.add_modifier(offset, modifier_address)
    }
}

impl<O:ObjectViewTrait> BigObjectViewTrait for ObjectView<O>{
    fn release(&self, transaction:&TransactionInfo) {
        self.get_mut().release(transaction);
    }
}

pub trait ObjectViewTrait:Sized {
    fn add_modifier(&mut self, offset:usize, modifier_address:*const Box<ModifierTrait>) -> bool;
    fn release(&self,transaction:&TransactionInfo);
}

pub trait BigObjectViewTrait {
    fn release(&self, transaction:&TransactionInfo);
}

impl BigObjectViewTrait {
    pub fn downcast_ref_unchecked<O: BigObjectViewTrait>(&self) -> &O {
        unsafe { &*(self as *const BigObjectViewTrait as *const O) }
    }
}