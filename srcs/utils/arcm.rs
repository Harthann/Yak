use crate::spin::Mutex;
use alloc::sync::Arc;
use core::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Arcm<T> {
    arc: Arc<Mutex<T>>
}

impl<T> Arcm<T> {
    pub fn new(data: T) -> Self {
        Self {
            arc: Arc::new(Mutex::new(data))
        }
    }

    pub fn execute<R>(&self, mut closure: impl FnMut(Arc<Mutex<T>>) -> R) -> R {
        closure(self.arc.clone())
    }
}

impl<T> Deref for Arcm<T> {
	type Target = Arc<Mutex<T>>;
    fn deref(&self) -> &Arc<Mutex<T>> {
        &self.arc
    }
}

impl<T> DerefMut for Arcm<T> {
    fn deref_mut(&mut self) -> &mut Arc<Mutex<T>> {
        &mut self.arc
    }
}

mod test {
    use super::Arcm;
    use crate::boxed::Box;

    #[sys_macros::test_case]
    fn test_arcm_closure() {
        let arcm: Arcm<u32> = Arcm::new(5);
        arcm.execute(|cloned| {
            let mut guard = cloned.lock();
            *guard = 6;
        });
        assert_eq!(*arcm.lock(), 6);
    }

    #[sys_macros::test_case]
    fn test_arcm_with_box() {
        let arcm: Arcm<Box<u32>> = Arcm::new(Box::new(5));
        arcm.execute(|cloned| {
            let mut guard = cloned.lock();
            **guard = 6;
        });
        assert_eq!(**arcm.lock(), 6);
    }
}
