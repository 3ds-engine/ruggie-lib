use std::rc::Rc;

use ctru::linear::LinearAllocator;

///Sound asset loaded in memory
#[derive(Clone)]
pub struct Sound(pub(crate)Rc<[u8], &'static LinearAllocator>);