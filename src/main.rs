use std::fs;
use std::env;
use std::path::Path;

mod tree;

#[derive(Debug)]
struct Node {
    name: String,
    children: Vec<Node>
}

fn walk(path_to_dir: &Path) -> Node {
    let node_name = String::from(path_to_dir.file_name().unwrap().to_str().unwrap());
    let mut node = Node {
        name: node_name,
        children: Vec::new()
    };

    if path_to_dir.is_file() {
        return node;
    }

    let entries = fs::read_dir(Path::new(path_to_dir)).unwrap();

    for entry in entries {
        let item = entry.unwrap();
        let item_type = item.file_type().unwrap();

        if item_type.is_dir() {
            node.children.push(
                walk(&item.path())
            );
        } else {
            node.children.push(Node {
                name: item.file_name().into_string().unwrap(),
                children: Vec::new()
            });
        }
    }

    return node;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let root = walk(Path::new(path));
    println!("the root is: {:?}", root);
}

