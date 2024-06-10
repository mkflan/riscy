use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
};

/// A simple spinlock allowing mutual exclusive access to the underlying data.
#[derive(Debug)]
pub struct Spinlock<T: ?Sized> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}

#[derive(Debug)]
pub struct SpinlockGuard<'a, T: ?Sized> {
    lock: &'a Spinlock<T>,
}

unsafe impl<T: ?Sized + Send> Send for Spinlock<T> {}
unsafe impl<T: ?Sized + Send> Sync for Spinlock<T> {}

unsafe impl<T: ?Sized + Sync> Sync for SpinlockGuard<'_, T> {}

impl<T> Spinlock<T> {
    pub const fn new(data: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    /// Return whether the lock is held.
    pub fn is_locked(&self) -> bool {
        self.locked.load(Ordering::Relaxed)
    }

    /// Acquire the lock.
    pub fn acquire(&self) -> SpinlockGuard<'_, T> {
        // Execute without interrupts.
        riscv::interrupt::supervisor::free(|| {
            while self
                .locked
                .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                .is_err()
                && self.is_locked()
            {
                core::hint::spin_loop();
            }
        });

        SpinlockGuard { lock: self }
    }
}

impl<T: ?Sized> Deref for SpinlockGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: If we have a guard, the lock has been acquired and we have mutual exclusive access to the underlying data.
        unsafe { &*self.lock.data.get() }
    }
}

impl<T: ?Sized> DerefMut for SpinlockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: If we have a guard, the lock has been acquired and we have mutual exclusive access to the underlying data.
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<T: ?Sized> Drop for SpinlockGuard<'_, T> {
    fn drop(&mut self) {
        // Release the lock the guard is for.
        self.lock.locked.store(false, Ordering::Release);
    }
}
