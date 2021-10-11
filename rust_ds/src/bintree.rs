pub struct BTNode<T> {
    pub left: Option<Box<BTNode<T>>>,
    pub right: Option<Box<BTNode<T>>>,
    pub op: Op<T>,
}

pub enum Op<T> {
    Add,
    Sub,
    Div,
    Mul,
    Id(T),
}

pub struct BinaryTree<T> {
    pub head: Option<BTNode<T>>,
}

impl<T> BTNode<T> {
    pub fn new(op: Op<T>, left: BTNode<T>, right: BTNode<T>) -> Self {
        BTNode::<T> {
            op: op,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

pub fn add_node<T>(left: BTNode<T>, right: BTNode<T>) -> BTNode<T> {
    BTNode::new(Op::Add, left, right)
}

pub fn sub_node<T>(left: BTNode<T>, right: BTNode<T>) -> BTNode<T> {
    BTNode::new(Op::Sub, left, right)
}

pub fn div_node<T>(left: BTNode<T>, right: BTNode<T>) -> BTNode<T> {
    BTNode::new(Op::Div, left, right)
}

pub fn mul_node<T>(left: BTNode<T>, right: BTNode<T>) -> BTNode<T> {
    BTNode::new(Op::Mul, left, right)
}

pub fn id_node<T>(value: T) -> BTNode<T> {
    BTNode::<T> {
        op: Op::Id(value),
        left: None,
        right: None,
    }
}

impl<T> BinaryTree<T>{
    pub fn new(head:BTNode<T>)->Self{
        BinaryTree::<T>{
            head:Some(head)
        }
    }

    pub fn collapse(node: &Box<BTNode<T>>)->T
    where T : Copy          // .cloned()
            + Into<i32>   // Conversions between T and usize
            + From<i32>
    {
        let mut r:Option<T>=None;
        let mut l:Option<T>=None;

        //if left node exist, transverse the left
        if let Some(left)=&node.left{
            l=Some(BinaryTree::collapse(left));
        }

        //if right node exist, transverse the right
        if let Some(right)=&node.right{
            r=Some(BinaryTree::collapse(right));
        }

        //handle the leaf nodes
        let r=if let Some(x)=r{x.into()}else{0};
        let l=if let Some(x)=l{x.into()}else{0};

        match node.op{
            Op::Add=>{(l+r).into()}
            Op::Sub=>{(l-r).into()}
            Op::Mul=>{(l*r).into()}
            Op::Div=>{
                if r==0{
                    panic!("attempting to divide by zero. Invalid!")
                };
                (l/r).into()
            }
            Op::Id(x)=>x
        }
    }
}

