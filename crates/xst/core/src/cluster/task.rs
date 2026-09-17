//! Synchronous trampoline for typed mapping continuations. Only executable
//! work is erased; borrowed shard values stay in statically typed slots.
use std::{cell::RefCell, rc::Rc};

use super::ExtractError;

type Work<'a> = Box<dyn FnOnce(&mut Stack<'a>) -> Result<(), ExtractError> + 'a>;
type Start<'a, T> = Box<dyn FnOnce(&mut Stack<'a>, Slot<T>) -> Result<(), ExtractError> + 'a>;
type Slot<T> = Rc<RefCell<Option<T>>>;

struct Stack<'a> {
    work: Vec<Work<'a>>,
}

impl<'a> Stack<'a> {
    fn push<T: 'a>(&mut self, task: MappingTask<'a, T>, output: Slot<T>) {
        self.work.push(Box::new(move |stack| (task.start)(stack, output)));
    }
}

fn take<T>(slot: &Slot<T>) -> Result<T, ExtractError> {
    slot.borrow_mut().take().ok_or(ExtractError::InvalidMapping)
}

/// A deferred mapping computation, executed with an explicit heap stack.
/// Constructing or composing tasks never runs a child shard mapper.
pub struct MappingTask<'a, T> {
    start: Start<'a, T>,
}

impl<'a, T: 'a> MappingTask<'a, T> {
    pub fn ready(value: Result<T, ExtractError>) -> Self {
        Self { start: Box::new(move |_, output| {
            *output.borrow_mut() = Some(value?);
            Ok(())
        }) }
    }

    pub fn defer(f: impl FnOnce() -> Result<Self, ExtractError> + 'a) -> Self {
        Self { start: Box::new(move |stack, output| {
            stack.push(f()?, output);
            Ok(())
        }) }
    }

    pub fn map<U: 'a>(self, f: impl FnOnce(T) -> Result<U, ExtractError> + 'a) -> MappingTask<'a, U> {
        MappingTask { start: Box::new(move |stack, output| {
            let child = Rc::new(RefCell::new(None));
            let result = Rc::clone(&child);
            stack.work.push(Box::new(move |_| {
                *output.borrow_mut() = Some(f(take(&result)?)?);
                Ok(())
            }));
            stack.push(self, child);
            Ok(())
        }) }
    }

    /// Runs left before right, preserving source field evaluation order.
    pub fn zip<U: 'a>(self, right: MappingTask<'a, U>) -> MappingTask<'a, (T, U)> {
        MappingTask { start: Box::new(move |stack, output| {
            let left_slot = Rc::new(RefCell::new(None));
            let right_slot = Rc::new(RefCell::new(None));
            let left_result = Rc::clone(&left_slot);
            let right_result = Rc::clone(&right_slot);
            stack.work.push(Box::new(move |_| {
                *output.borrow_mut() = Some((take(&left_result)?, take(&right_result)?));
                Ok(())
            }));
            stack.push(right, right_slot);
            stack.push(self, left_slot);
            Ok(())
        }) }
    }

    /// Runs each element in order without building a recursively nested chain.
    pub fn collect(tasks: Vec<Self>) -> MappingTask<'a, Vec<T>> {
        MappingTask { start: Box::new(move |stack, output| {
            let slots: Vec<_> = tasks.iter().map(|_| Rc::new(RefCell::new(None))).collect();
            let results = slots.clone();
            stack.work.push(Box::new(move |_| {
                *output.borrow_mut() = Some(results.iter().map(take).collect::<Result<_, _>>()?);
                Ok(())
            }));
            for (task, slot) in tasks.into_iter().zip(slots).rev() {
                stack.push(task, slot);
            }
            Ok(())
        }) }
    }

    pub fn run(self) -> Result<T, ExtractError> {
        let output = Rc::new(RefCell::new(None));
        let mut stack = Stack { work: Vec::new() };
        stack.push(self, Rc::clone(&output));
        while let Some(work) = stack.work.pop() {
            work(&mut stack)?;
        }
        take(&output)
    }
}
