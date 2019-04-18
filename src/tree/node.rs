pub struct Node<'a, T> {
    name: T,
    children: Vec<&'a Node<'a, T>>
}

impl <'a, T> Node<'a, T> {
    fn append_child(&mut self, child: &'a Node<T>) {
        &self.children.push(child);
    }
}

pub fn make_node<'a, T>(name: T) -> Node<'a, T> {
    return Node {
        name: name,
        children: Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_node_constructor() {
        let name = String::from("some name");
        let node = make_node(name);

        assert_eq!(node.name, name);
    }

    #[test]
    fn append_child_works() {
        let name = String::from("some name");
        let node = make_node(name);
        let another_node = make_node(String::from("another name"));

        node.append_child(&another_node);

        assert_eq!(node.children.len(), 1);
        assert_eq!(node.children[0], &another_node);
    }
}
