pub struct TreeNode<T> {
    data: T,
    children: Vec<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(data: T) -> Self {
        TreeNode {
            data: data,
            children: Vec::new(),
        }
    }

    pub fn get_children(&mut self) -> &mut Vec<Box<TreeNode<T>>> {
        &mut self.children
    }
}

pub struct Tree<T> {
    root: Option<Box<TreeNode<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn add_root(&mut self, data: T) -> &TreeNode<T> {
        let root_node = TreeNode::new(data);
        self.root = Some(Box::new(root_node));

        &self.root.as_ref().unwrap()
    }

    pub fn add_child(parent: &mut TreeNode<T>, data: T) -> &TreeNode<T> {
        let child_node = TreeNode::new(data);
        {
            let children = &mut parent.get_children();
            children.push(Box::new(child_node));
        }
        let length = parent.get_children().len();
        parent.get_children()[length - 1].as_ref()
    }
}
