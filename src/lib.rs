//! Semaphore.

use std::sync::{Condvar, Mutex};
use std::time::Duration;

/// Semaphore.
pub struct Semaphore {
    lock: Mutex<usize>,
    cvar: Condvar,
}

/// Semaphore guard.
pub struct SemaphoreGuard<'a> {
    semaphore: &'a Semaphore,
}

impl Semaphore {
    /// Creates a new 'Semaphore' with a specific counter value.
    pub fn new(count: usize) -> Semaphore {
        Semaphore {
            lock: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    /// Acquires the 'Semaphore' (blocking operation).
    pub fn acquire(&self) {
        let mut count = self.lock.lock().unwrap();
        while *count == 0 {
            count = self.cvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    /// Tries to acquire the 'Semaphore' for the duration specified (blocking operation)
    /// and returns true on success and false on failure.
    pub fn acquire_timeout(&self, dur: Duration) -> bool {
        let mut count = self.lock.lock().unwrap();
        match self.cvar.wait_timeout(count, dur) {
            Ok((new_count, _)) => {
                count = new_count;
                if *count > 0 {
                    *count -= 1;
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    /// Tries to acquire the 'Semaphore' immediately (non-blocking operation)
    /// and returns true on success and false on failure.
    pub fn try_acquire(&self) -> bool {
        let mut count = self.lock.lock().unwrap();
        if *count > 0 {
            *count -= 1;
            true
        } else {
            false
        }
    }

    /// Releases the 'Semaphore'.
    pub fn release(&self) {
        *self.lock.lock().unwrap() += 1;
        self.cvar.notify_one();
    }

    /// Returns current value of the 'Semaphore''s counter.
    pub fn get_value(&self) -> usize {
        *self.lock.lock().unwrap()
    }
}

impl<'a> SemaphoreGuard<'a> {
    /// Acquires the 'Semaphore' and returns a 'SemaphoreGuard'.
    pub fn acquire(semaphore: &'a Semaphore) -> Self {
        semaphore.acquire();
        SemaphoreGuard { semaphore }
    }
}

impl<'a> Drop for SemaphoreGuard<'a> {
    /// Releases the acquired 'Semaphore'.
    fn drop(&mut self) {
        self.semaphore.release();
    }
}
