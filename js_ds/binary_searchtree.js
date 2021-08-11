class Node {
  constructor(data) {
    this.data = data;
    this.left = null;
    this.right = null;
  }
}

class BinarySearchTree {
  constructor() {
    this.root = null;
  }
  //insert a new node to the tree
  insert(data) {
    //create a node and init the value
    let new_node = new Node(data);
    if (this.root == null) {
      this.root = new_node;
    } else {
      this.insertNode(this.root, new_node);
    }
  }
  // Method to insert a node in a tree
  // it moves over the tree to find the location
  // to insert a node with a given data
  insertNode(node, new_node) {
    // if the data is less than the node
    // data move left of the tree
    if (new_node.data < node.data) {
      if (node.left === null) {
        node.left = new_node;
      } else {
        //if left id not null
        //recurse until null is found
        this.insertNode(node.left, new_node);
      }
    } else {
      // if right is null insert node here
      if (node.right === null) {
        node.right = new_node;
      } else {
        //if right is not null
        //recurse until null is found
        this.insertNode(node.right, new_node);
      }
    }
  }
  remove(data) {
    //root is re-initialized with root of a modified tree
    this.root = this.removeNode(this.root, data);
  }
  removeNode(node, key) {
    //return null if root null is empty
    if (node === null) {
      return null;
    }
    //if data to be deleted is less than root node
    //remove the left sub tree
    else if (key < node.data) {
      node.left = this.removeNode(node.left, key);
      return node;
    }
    //if data to be deleted is greater than root node
    //remove the right sub tree
    else if (key > node.data) {
      node.right = this.removeNode(node.right, key);
      return node;
    }
    // if data is similar to the root's data
    // then delete this node
    else {
      //remove node with no children
      if (node.left === null && node.right === null) {
        node = null;
        return null;
      }
      //remove node with one child
      if (node.left === null) {
        node = node.right;
        return node;
      } else if (node.right === null) {
        node = node.left;
        return node;
      }
      // Deleting node with two children
      // minumum node of the right subtree
      // is stored in aux
      var aux = this.findMinNode(node.right);
      node.data = aux.data;

      node.right = this.removeNode(node.right, aux.data);
      return node;
    }
  }
  // Performs inorder traversal of a tree
  inorder(node) {
    if (node !== null) {
      this.inorder(node.left);
      console.log(node.data);
      this.inorder(node.right);
    }
  }
  // Performs preorder traversal of a tree
  preorder(node) {
    if (node !== null) {
      console.log(node.data);
      this.preorder(node.left);
      this.preorder(node.right);
    }
  }
  //  finds the minimum node in tree
  // searching starts from given node
  findMinNode(node) {
    //if left node is null, then node must be min node
    if (node.left === null) {
      return node;
    } else {
      return this.findMinNode(node.left);
    }
  }
  //get root node of tree
  getRootNode() {
    return this.root;
  }

  // search for a node with given data
  search(node, data) {
    // if trees is empty return null
    if (node === null) {
      return null;
    }

    // if data is less than node's data
    // move left
    else if (data < node.data) {
      return this.search(node.left, data);
    }
    // if data is less than node's data
    // move left
    else if (data > node.data) {
      return this.search(node.right, data);
    }
    // if data is equal to the node data
    // return node
    else {
      return node;
    }
  }
}

var BST = new BinarySearchTree();

// Inserting nodes to the BinarySearchTree
BST.insert(15);
BST.insert(25);
BST.insert(10);
BST.insert(7);
BST.insert(22);
BST.insert(17);
BST.insert(13);
BST.insert(5);
BST.insert(9);
BST.insert(27);

var root = BST.getRootNode();

BST.inorder(root);
