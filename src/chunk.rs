/// A chunk is a consecutive group of iterations assigned together to be
/// executed by a thread.
pub(crate) struct Chunk {
    /// Lower and upper bounds
    lb: i32,
    ub: i32,
}

impl Chunk {
    pub fn new(lb: i32, ub: i32) -> Chunk {
        Chunk {
            lb,
            ub,
        }
    }
    pub fn first(&self) -> i32 {
        self.lb
    }
    pub fn second(&self) -> i32 {
        self.ub
    }
}
