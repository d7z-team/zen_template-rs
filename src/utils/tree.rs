use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::{Rc, Weak};
use std::slice::Iter;

struct Tree<T: Debug + Sized> {
    pub root: Rc<RefCell<TreeEntity<T>>>,
    pub cache: TreeCache<T>,
}

struct TreeCache<T: Debug + Sized> {
    pub list: Vec<Weak<RefCell<TreeEntity<T>>>>,
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


impl<T: Sized + Debug> Tree<T> {
    fn iter(&self) -> Iter<'_, TreeEntity<T>> {
        todo!()
    }
}

impl<T: Sized + Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}


impl<T: Debug + Sized> TreeCache<T> {
    fn new(root: &Rc<RefCell<TreeEntity<T>>>) -> TreeCache<T> {
        todo!()
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
    #[inline]
    fn _unwrap_point<F>(&mut self, func: F) -> &mut TreeBuilder<T>
        where F: FnOnce(&Rc<RefCell<TreeEntity<T>>>) -> () {
        if let Some(entity) = &self.pointer.upgrade() {
            func(entity)
        } else {
            panic!("此时不应该得到 None .")
        }
        self
    }
    /// add new child node`
    fn push(&mut self, data: T) -> &mut TreeBuilder<T> {
        self._unwrap_point(|entity| {
            let mut ref_mut = entity.borrow_mut();
            ref_mut.child.push(Rc::new(RefCell::new(
                TreeEntity {
                    data: Some(data),
                    child: vec![],
                    parent: Rc::downgrade(entity),
                }
            )))
        })
    }
    fn set(&mut self, data: T) -> &mut TreeBuilder<T> {
        self._unwrap_point(|e| {
            let mut ref_mut = e.borrow_mut();
            ref_mut.data = Some(data)
        })
    }
    fn new_child(&mut self, data: T) -> &mut TreeBuilder<T> {
        self.push(data);
        self.pointer = Rc::downgrade(self.pointer.upgrade().unwrap().borrow().child.last().unwrap());
        self
    }
    fn new_empty_child(&mut self) -> &mut TreeBuilder<T> {
        self._unwrap_point(|entity| {
            let mut ref_mut = entity.borrow_mut();
            ref_mut.child.push(Rc::new(RefCell::new(
                TreeEntity {
                    data: None,
                    child: vec![],
                    parent: Rc::downgrade(entity),
                }
            )))
        });
        self.pointer = Rc::downgrade(self.pointer.upgrade().unwrap().borrow().child.last().unwrap());
        self
    }

    fn current_count(&mut self) -> usize {
        self.pointer.upgrade().unwrap().borrow().child.len()
    }

    fn join_child(&mut self, index: usize) -> &mut TreeBuilder<T> {
        self.pointer = Rc::downgrade(self.pointer.upgrade().unwrap().borrow().child.get(index).unwrap());
        self
    }
    fn end_child(&mut self) -> &mut TreeBuilder<T> {
        self.pointer = Rc::downgrade(&self.pointer.upgrade().expect("已到达根节点").borrow().parent.upgrade().unwrap());
        self
    }
    fn remove_child(&mut self, index: usize) -> &mut TreeBuilder<T> {
        self._unwrap_point(|e| {
            e.borrow_mut().child.remove(index);
        })
    }
    fn build(self) -> Tree<T> {
        let cache = TreeCache::new(&self.root);
        Tree {
            root: self.root,
            cache,
        }
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
    fn test() {
        let mut builder: TreeBuilder<&'static str> = Tree::builder();
        builder.push("asas").new_empty_child();
        let tree = builder.build();
        println!("{:?}", tree);
    }
}
