use std::cmp::Reverse;
use std::collections::binary_heap::BinaryHeap;
use std::sync::Mutex;

use crate::core::support::emulated_time::EmulatedTime;

use super::event::Event;

/// A queue of [`Event`]s ordered by their times.
#[derive(Debug)]
pub struct EventQueue {
    queue: BinaryHeap<Reverse<PanickingOrd<Event>>>,
    last_popped_event_time: EmulatedTime,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            last_popped_event_time: EmulatedTime::SIMULATION_START,
        }
    }

    /// Push a new [`Event`] on to the queue.
    ///
    /// Will panic if two events are pushed that have no relative order
    /// (`event_a.partial_cmp(&event_b) == None`). Will be non-deterministic if two events are
    /// pushed that are equal (`event_a == event_b`).
    pub fn push(&mut self, event: Event) {
        self.queue.push(Reverse(event.into()));
    }

    /// Pop the earliest [`Event`] from the queue.
    pub fn pop(&mut self) -> Option<Event> {
        let event = self.queue.pop().map(|x| x.0.into_inner());

        // make sure time never moves backward
        if let Some(ref event) = event {
            assert!(event.time() >= self.last_popped_event_time);
            self.last_popped_event_time = event.time();
        }

        event
    }

    /// The time of the next [`Event`] (the time of the earliest event in the queue).
    pub fn next_event_time(&self) -> Option<EmulatedTime> {
        self.queue.peek().map(|x| x.0.time())
    }
}

/// A wrapper type that implements [`Ord`] for types that implement [`PartialOrd`]. If the two
/// objects cannot be compared (`PartialOrd::partial_cmp` returns `None`), the comparison will
/// panic.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct PanickingOrd<T: PartialOrd + Eq>(T);

impl<T: PartialOrd + Eq> PanickingOrd<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: PartialOrd + Eq> std::convert::From<T> for PanickingOrd<T> {
    fn from(x: T) -> Self {
        PanickingOrd(x)
    }
}

impl<T: PartialOrd + Eq> PartialOrd for PanickingOrd<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: PartialOrd + Eq> Ord for PanickingOrd<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

impl<T: PartialOrd + Eq> std::ops::Deref for PanickingOrd<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PartialOrd + Eq> std::ops::DerefMut for PanickingOrd<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

mod export {
    use super::*;
    use crate::core::support::simulation_time::SimulationTime;
    use crate::cshadow as c;

    /// A wrapper for [`EventQueue`] that uses interior mutability to make the ffi simpler.
    pub struct ThreadSafeEventQueue {
        event_queue: Mutex<EventQueue>,
    }

    impl ThreadSafeEventQueue {
        pub fn new() -> Self {
            Self {
                event_queue: Mutex::new(EventQueue::new()),
            }
        }

        pub fn push(&self, event: Event) {
            self.event_queue.lock().unwrap().push(event);
        }

        pub fn pop(&self) -> Option<Event> {
            self.event_queue.lock().unwrap().pop()
        }

        pub fn next_event_time(&self) -> Option<EmulatedTime> {
            self.event_queue.lock().unwrap().next_event_time()
        }
    }

    #[no_mangle]
    pub unsafe extern "C" fn eventqueue_new() -> *mut ThreadSafeEventQueue {
        Box::into_raw(Box::new(ThreadSafeEventQueue::new()))
    }

    #[no_mangle]
    pub unsafe extern "C" fn eventqueue_free(queue: *mut ThreadSafeEventQueue) {
        assert!(!queue.is_null());
        unsafe { Box::from_raw(queue) };
    }

    /// Takes ownership of the event.
    #[no_mangle]
    pub unsafe extern "C" fn eventqueue_push(
        queue: *const ThreadSafeEventQueue,
        event: *mut Event,
    ) {
        assert!(!event.is_null());
        let queue = unsafe { queue.as_ref() }.unwrap();
        let event = unsafe { Box::from_raw(event) };
        queue.push(*event);
    }

    #[no_mangle]
    pub unsafe extern "C" fn eventqueue_pop(queue: *const ThreadSafeEventQueue) -> *mut Event {
        let queue = unsafe { queue.as_ref() }.unwrap();
        queue
            .pop()
            .map(Box::new)
            .map(Box::into_raw)
            .unwrap_or(std::ptr::null_mut())
    }

    #[no_mangle]
    pub unsafe extern "C" fn eventqueue_nextEventTime(
        queue: *const ThreadSafeEventQueue,
    ) -> c::SimulationTime {
        let queue = unsafe { queue.as_ref() }.unwrap();
        let time = queue.next_event_time().map(EmulatedTime::to_abs_simtime);
        SimulationTime::to_c_simtime(time)
    }
}