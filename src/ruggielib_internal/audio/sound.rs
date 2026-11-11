use std::rc::Rc;

///Sound asset loaded in memory
#[derive(Clone)]
pub struct Sound{
    pub(crate) handle: Rc<()>,
    pub(crate) path: String
}