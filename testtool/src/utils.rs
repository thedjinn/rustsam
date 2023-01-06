use std::cmp::Ordering;

/// A two-tuple that can be used with a BinaryHeap to construct a MinHeap-based proprity queue.
pub struct MinPrioritized<P: Ord, T> {
    pub priority: P,
    pub value: T
}

impl<P: Ord, T> Eq for MinPrioritized<P, T> {}

impl<P: Ord, T> Ord for MinPrioritized<P, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.priority.cmp(&self.priority)
    }
}

impl<P: Ord, T> PartialOrd for MinPrioritized<P, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P: Ord, T> PartialEq for MinPrioritized<P, T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<P: Ord, T> From<(P, T)> for MinPrioritized<P, T> {
    fn from(value: (P, T)) -> Self {
        Self {
            priority: value.0,
            value: value.1
        }
    }
}
