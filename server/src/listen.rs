use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Registree<T> {
    items: Vec<T>,
    children: BTreeMap<String, Registree<T>>,
}
impl<T> Default for Registree<T> {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            children: Default::default(),
        }
    }
}

impl<T> Registree<T> {
    pub fn insert<S: Into<String>>(&mut self, path: Vec<S>, item: T) {
        let mut cur = self;
        for key in path {
            cur = cur.children.entry(key.into()).or_default();
        }
        cur.items.push(item);
    }
    pub fn traverse<S: AsRef<str>>(&mut self, path: &[S], mut f: impl FnMut(&mut Vec<T>) -> ()) {
        let mut cur = self;
        for key in path {
            f(&mut cur.items);
            cur = match cur.children.get_mut(key.as_ref()) {
                Some(child) => child,
                None => return,
            };
        }
        cur.traverse_descendents(&mut f);
    }
    fn traverse_descendents(&mut self, f: &mut impl FnMut(&mut Vec<T>) -> ()) {
        f(&mut self.items);
        for child in self.children.values_mut() {
            child.traverse_descendents(f);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Registree;

    #[test]
    fn smoke_test() {
        let mut tree = Registree::default();
        tree.insert(vec!["a"], 0);
        tree.insert(vec!["a", "b"], 1);
        tree.insert(vec!["a", "b", "c"], 2);
        tree.insert(vec!["a", "b", "d"], 3);
        tree.insert(vec!["x", "y", "z"], 4);

        let mut vis = Vec::new();
        tree.traverse(&["a", "b"], |items| vis.extend_from_slice(items));
        assert_eq!(vis, [0, 1, 2, 3]);
    }
}
