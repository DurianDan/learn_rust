// https://www.youtube.com/watch?v=ygL_xcavzQ4&t=177s
// from 2:05:47 to 2:15:00

fn main(){
    // BOX pointers, stores data on heap
    let box_integer = Box::new(10);
    println!("{}", box_integer);
    
    // use binary trie:
    let node1 = TrieNode::new(1)
                    .add_left(TrieNode::new(3))
                    .add_right(TrieNode::new(4));
    
    // add .unwrap() to go straight to Some() value
    println!("{}",node1.left.unwrap().key);

}

// practical example: create a binary tree struct
struct TrieNode<T>{
    // the pub keyword allows other code to call these objects
    // a struct cant have itself as a parameter (infinitely contains itself, => infinite data size),
    // Box<> pointer has to be ultilized so that the TrieNode stored pointers, instead of actual data 
    // Option<> enum handle the null Nodes, which are the end of a TrieNode
        // Option will return "None", or "Some(T)"
        // Option enum can handle Null scenerios 
    left: Option<Box<TrieNode<T>>>,
    right: Option<Box<TrieNode<T>>>,
    key: T
}

impl<T> TrieNode<T>{
    fn new(key: T) -> Self{
       TrieNode { left: None, right: None, key} 
    }
    fn add_left(mut self, node: TrieNode<T>) -> Self{
        self.left = Some(Box::new(node));
        self
    }
    fn add_right(mut self, node: TrieNode<T>) -> Self{
        self.right = Some(Box::new(node));
        self
    }
}
