//! Callbacks contains function that can be called at a later time.
//! This is used in containing an event listener attached to an DOM element.
use std::{
    any::TypeId,
    fmt,
    rc::Rc,
};

/// A generic sized representation of a function that can be
/// attached to a Node. The callback will essentially be owned by the element
///
/// Limitations:
/// The callback takes an Fn instead of FnMut,
/// therefore it can not mutate the environment variables
///
/// In effect callbacks attached to DOM events are limited
/// to only passing an OUT to the program and not complex statements.
///
/// Note: It would have been nice to have the inner value be
/// `Rc<FnMut(IN) -> OUT> +'a`
/// but there are a lot of issues that this becomes infeasible:
///  1 - wasm_bindgen::Closure requires that 'static references to the closure
///  2 - Accessing `Rc::get_mut` requires that there is no other `Rc` or `Weak` references
///     else where.
///         - We could be iterating on the elements for recursively setting the attributes
///             which is not allowed to have recursive mutable iteration
///         - Attributes of the same name are merged therefore cloning the attributes, hence the
///         callback is necessary.
///
pub struct Listener<IN, OUT> {
    /// the function to be executed
    func: Rc<dyn Fn(IN) -> OUT>,
    /// the type_id of the function
    func_type_id: TypeId,
    /// the type type_id of the event this callback will be attached to
    event_type_id: TypeId,
    /// the type_id of the return type of this callback when executed.
    msg_type_id: TypeId,
}

impl<IN, F, OUT> From<F> for Listener<IN, OUT>
where
    F: Fn(IN) -> OUT + 'static,
    OUT: 'static,
    IN: 'static,
{
    fn from(func: F) -> Self {
        Self {
            func: Rc::new(func),
            func_type_id: TypeId::of::<F>(),
            event_type_id: TypeId::of::<IN>(),
            msg_type_id: TypeId::of::<OUT>(),
        }
    }
}

/// Note:
/// using the #[derive(Debug)] needs IN and OUT to also be Debug
///
/// The reason this is manually implemented is, so that IN and OUT
/// doesn't need to be Debug as it is part of the Listener objects and are not shown.
impl<IN, OUT> fmt::Debug for Listener<IN, OUT> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "in: {:?}, out: {:?}, func: {:?}",
            self.event_type_id, self.msg_type_id, self.func_type_id
        )
    }
}

impl<IN, OUT> Listener<IN, OUT>
where
    IN: 'static,
    OUT: 'static,
{
    /// This method calls the actual callback.
    pub fn emit(&self, input: IN) -> OUT {
        (self.func)(input)
    }

    /// map this callback using another callback such that
    /// OUT becomes MSG2
    pub fn map_callback<MSG2>(
        self,
        cb: Listener<OUT, MSG2>,
    ) -> Listener<IN, MSG2>
    where
        MSG2: 'static,
    {
        let func_wrap = move |input| {
            let out = self.emit(input);
            cb.emit(out)
        };
        Listener::from(func_wrap)
    }
}

/// Note:
/// using the #[derive(Clone)] needs IN and OUT to also be Clone
///
/// The reason this is manually implemented is, so that IN and OUT
/// doesn't need to be Clone as it is part of the Listener objects and cloning here
/// is just cloning the pointer of the actual callback function
impl<IN, OUT> Clone for Listener<IN, OUT> {
    fn clone(&self) -> Self {
        Self {
            func: Rc::clone(&self.func),
            func_type_id: self.func_type_id,
            event_type_id: self.event_type_id,
            msg_type_id: self.msg_type_id,
        }
    }
}

/// This is the best approximation of comparison whether 2 callbacks are equal.
///
/// There is no 100% guarante that this is true since we can not compare 2 closures event if they
/// have the same logic.
///
/// This is done by comparing the type_id of the input and type_id of the output.
///
impl<IN, OUT> PartialEq for Listener<IN, OUT> {
    fn eq(&self, other: &Self) -> bool {
        self.event_type_id == other.event_type_id
            && self.msg_type_id == other.msg_type_id
            && self.func_type_id == other.func_type_id
    }
}
