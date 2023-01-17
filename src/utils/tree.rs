use std::cell::RefCell;
use std::fmt::Debug;
use std::mem;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Tree<T: Debug + Sized> {
    pub root: Rc<RefCell<TreeEntity<T>>>,
}

#[derive(Debug)]
struct TreeBuilder<T: Debug + Sized> {
    pub root: Rc<RefCell<TreeEntity<T>>>,
    pub pointer: Weak<RefCell<TreeEntity<T>>>,
}


#[derive(Debug)]
struct TreeEntity<T: Debug + Sized> {
    data: Option<T>,
    child: Vec<Rc<RefCell<TreeEntity<T>>>>,
    parent: Weak<RefCell<TreeEntity<T>>>,
}



impl<T: Sized + Debug> Tree<T> {
    fn builder() -> TreeBuilder<T> {
       TreeBuilder::new()
    }

}


impl<T: Sized + Debug> TreeBuilder<T> {
    fn new() -> TreeBuilder<T> {
        let entity = Rc::new(RefCell::new(TreeEntity { data: None, child: vec![], parent: Default::default() }));
        let weak = Rc::downgrade(&entity);
        TreeBuilder {
            root: entity,
            pointer: weak,
        }
    }
    fn _set(&mut self, new_data: Option<T>) -> Option<T> {
        let mut new_data = new_data;
        if let Some(data) = self.pointer.upgrade() {
            let mut ref_mut = data.borrow_mut();
            mem::swap(&mut ref_mut.data, &mut new_data)
        }
        new_data
    }
    fn remove(&mut self) -> Option<T> {
        self._set(None)
    }
    fn set(&mut self, new_data: T) -> Option<T> {
        self._set(Some(new_data))
    }
    fn add(&mut self, data: T) -> &mut TreeBuilder<T> {
        self._add(Some(data))
    }
    fn _add(&mut self, data: Option<T>) -> &mut TreeBuilder<T> {
        if let Some(item) = self.pointer.upgrade() {
            let mut ref_mut = item.borrow_mut();
            ref_mut.child.push(Rc::new(RefCell::new(TreeEntity {
                data,
                child: vec![],
                parent: Rc::downgrade(&item),
            })))
        }
        self
    }

    fn new_empty_child(&mut self) -> &mut TreeBuilder<T> {
        self
    }
    fn new_child(&mut self, data: T) -> &mut TreeBuilder<T> {
        self.add(data);
        if let Some(item) = self.pointer.upgrade() {
            item.borrow_mut().child.last().unwrap();
        }
        self
    }
}
#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use crate::utils::tree::{Tree, TreeBuilder};

    #[derive(Debug)]
    struct V {}

    impl V {
        fn new() -> Self {
            V {}
        }
    }

    #[test]
    fn test() {}
}
