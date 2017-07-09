mod tree;
use tree::{Tree, TreeNode};

struct TestStruct;

fn buildit() {
    let mut tree = Tree::new();

    tree.set_root(TestStruct{});
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
