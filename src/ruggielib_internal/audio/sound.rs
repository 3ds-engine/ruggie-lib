use std::rc::Rc;

///Sound asset loaded in memory
#[derive(Clone)]
pub struct Sound(pub(crate)Rc<()>);