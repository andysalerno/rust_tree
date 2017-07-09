use std::rc::Weak;

pub struct TreeNode<T> {
    data: T,
    parent: Option<Weak<TreeNode<T>>>, 
    children: Vec<Box<TreeNode<T>>>,
}

impl <T> TreeNode<T> {
    pub fn new(data: T) -> Self {
        TreeNode { data: data, parent: None, children: Vec::new() }
    }

    pub fn new_child(data: T, parent: Weak<TreeNode<T>>) -> Self {
        TreeNode { data: data, parent: parent, children: Vec::new() }
    }

    pub fn get_children(&mut self) -> &mut Vec<Box<TreeNode<T>>> {
        &mut self.children
    }
}

pub struct Tree<T> {
    root: Option<Box<TreeNode<T>>>
}

impl <T> Tree<T> {
    pub fn new() -> Self {
        Tree{ root: None }
    }

    pub fn set_root(&mut self, data: T) -> &TreeNode<T> {
        let root_node = TreeNode::new(data);
        self.root = Some(Box::new(root_node));

        &self.root.as_ref().unwrap()
    }

    pub fn add_child(parent: &mut TreeNode<T>, data: T) -> &TreeNode<T> {
        let parent_ptr = 
        let child_node = TreeNode::new_child(data);
        {
            let children = &mut parent.get_children();
            children.push(Box::new(child_node));
        }
        let length = parent.get_children().len();
        parent.get_children()[length - 1].as_ref()
    }
}