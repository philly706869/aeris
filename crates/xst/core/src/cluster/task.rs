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
        self.work
            .push(Box::new(move |stack| (task.start)(stack, output)));
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
        Self {
            start: Box::new(move |_, output| {
                *output.borrow_mut() = Some(value?);
                Ok(())
            }),
        }
    }

    pub fn defer(f: impl FnOnce() -> Result<Self, ExtractError> + 'a) -> Self {
        Self {
            start: Box::new(move |stack, output| {
                stack.push(f()?, output);
                Ok(())
            }),
        }
    }

    pub fn map<U: 'a>(
        self,
        f: impl FnOnce(T) -> Result<U, ExtractError> + 'a,
    ) -> MappingTask<'a, U> {
        MappingTask {
            start: Box::new(move |stack, output| {
                let child = Rc::new(RefCell::new(None));
                let result = Rc::clone(&child);
                stack.work.push(Box::new(move |_| {
                    *output.borrow_mut() = Some(f(take(&result)?)?);
                    Ok(())
                }));
                stack.push(self, child);
                Ok(())
            }),
        }
    }

    /// Runs left before right, preserving source field evaluation order.
    pub fn zip<U: 'a>(self, right: MappingTask<'a, U>) -> MappingTask<'a, (T, U)> {
        MappingTask {
            start: Box::new(move |stack, output| {
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
            }),
        }
    }

    /// Runs each element in order without building a recursively nested chain.
    pub fn collect(tasks: Vec<Self>) -> MappingTask<'a, Vec<T>> {
        MappingTask {
            start: Box::new(move |stack, output| {
                let slots: Vec<_> = tasks.iter().map(|_| Rc::new(RefCell::new(None))).collect();
                let results = slots.clone();
                stack.work.push(Box::new(move |_| {
                    *output.borrow_mut() =
                        Some(results.iter().map(take).collect::<Result<_, _>>()?);
                    Ok(())
                }));
                for (task, slot) in tasks.into_iter().zip(slots).rev() {
                    stack.push(task, slot);
                }
                Ok(())
            }),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn descend(depth: usize) -> MappingTask<'static, usize> {
        MappingTask::defer(move || {
            Ok(if depth == 0 {
                MappingTask::ready(Ok(0))
            } else {
                descend(depth - 1).map(|value| Ok(value + 1))
            })
        })
    }

    #[test]
    fn ten_thousand_continuations_fit_on_a_small_native_stack() {
        std::thread::Builder::new()
            .stack_size(64 * 1024)
            .spawn(|| {
                assert_eq!(descend(10_000).run(), Ok(10_000));
            })
            .unwrap()
            .join()
            .unwrap();
    }

    struct Guard(Rc<Cell<usize>>);
    impl Drop for Guard {
        fn drop(&mut self) {
            self.0.set(self.0.get() + 1);
        }
    }

    fn fail(depth: usize, dropped: Rc<Cell<usize>>) -> MappingTask<'static, ()> {
        MappingTask::defer(move || {
            if depth == 0 {
                Ok(MappingTask::ready(Err(ExtractError::InvalidMapping)))
            } else {
                let guard = Guard(Rc::clone(&dropped));
                Ok(fail(depth - 1, dropped).map(move |_| {
                    drop(guard);
                    Ok(())
                }))
            }
        })
    }

    #[test]
    fn error_releases_pending_continuations_without_running_them() {
        std::thread::Builder::new()
            .stack_size(64 * 1024)
            .spawn(|| {
                let dropped = Rc::new(Cell::new(0));
                let ran = Rc::new(Cell::new(false));
                let observer = Rc::clone(&ran);
                let task = fail(10_000, Rc::clone(&dropped)).zip(MappingTask::defer(move || {
                    observer.set(true);
                    Ok(MappingTask::ready(Ok(())))
                }));
                assert_eq!(task.run(), Err(ExtractError::InvalidMapping));
                assert_eq!(dropped.get(), 10_000);
                assert!(!ran.get());
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn collection_preserves_order_and_nonclone_borrowed_outputs() {
        struct Borrowed<'a>(&'a str);
        let input = String::from("abc");
        let order = RefCell::new(Vec::new());
        let tasks = (0..3)
            .map(|index| {
                let input = &input;
                let order = &order;
                MappingTask::defer(move || {
                    order.borrow_mut().push(index);
                    Ok(MappingTask::ready(Ok(Borrowed(&input[index..index + 1]))))
                })
            })
            .collect();
        assert!(order.borrow().is_empty());
        let values = MappingTask::collect(tasks).run().unwrap();
        assert_eq!(*order.borrow(), vec![0, 1, 2]);
        assert_eq!(
            values.into_iter().map(|value| value.0).collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
    }
}
