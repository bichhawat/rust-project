use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
const ORDER: usize = 2; // order of the tree

// structure of a node in b+ tree

#[derive(Serialize, Deserialize,Debug, Clone)]
struct BPlusTree {
    keys: Vec<i32>,
    children: Vec<Box<BPlusTree>>,
    is_leaf: bool,
    num_keys: usize,
}
/*
We implement the following functions:
1. new_leaf
2. new_node
3. traverse
4. search
5. insert
6. insert_into_leaf
7. split_child
8. find_child_index
9. find_insert_position
10. delete
11. delete_from_leaf
12. balance_child
 */
impl BPlusTree {
    fn new_leaf() -> Self {
        BPlusTree {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf: true,
            num_keys: 0,
        }
    }

    fn new_node() -> Self {
        BPlusTree {
            keys: Vec::new(),
            children: Vec::new(),
            is_leaf: false,
            num_keys: 0,
        }
    }

    fn traverse(&self) {
        for (i, key) in self.keys.iter().enumerate() {
            if !self.is_leaf {
                self.children[i].traverse();
            }
            print!(" {} ", key);
        }

        if !self.is_leaf {
            self.children[self.num_keys].traverse();
        }
    }

    fn search(&self, key: i32) -> Option<usize> {
        let mut i = 0;

        while i < self.num_keys && key > self.keys[i] {
            i += 1;
        }

        if i < self.num_keys && key == self.keys[i] {
            return Some(i);
        }

        if self.is_leaf {
            return None;
        }

        self.children[i].search(key)
    }

    fn insert(&mut self, key: i32) {
        if self.is_leaf {
            self.insert_into_leaf(key);
        } else {
            let i = self.find_child_index(key);
            self.children[i].insert(key);

            if self.children[i].num_keys == ORDER {
                self.split_child(i);
            }
        }
    }

    fn insert_into_leaf(&mut self, key: i32) {
        let i = self.find_insert_position(key);
        self.keys.insert(i, key) ;
        self.num_keys += 1;
    }

    fn split_child(&mut self, child_index: usize) {
        let mut child = self.children[child_index].clone();
        let mut new_node = BPlusTree::new_node();

        for i in ORDER / 2..ORDER - 1 {
            new_node.keys.push(child.keys[i]);
            new_node.children.push(child.children[i].clone());
        }
        new_node.children.push(child.children[ORDER - 1].clone());

        child.keys.truncate(ORDER / 2);
        child.children.truncate(ORDER / 2 + 1);

        self.children.insert(child_index + 1, Box::new(new_node));
        self.keys.insert(child_index, child.keys.pop().unwrap());
        self.num_keys += 1;
    }

    fn find_child_index(&self, key: i32) -> usize {
        let mut i = 0;

        while i < self.num_keys && key > self.keys[i] {
            i += 1;
        }

        i
    }

    fn find_insert_position(&self, key: i32) -> usize {
        let mut i = 0;

        while i < self.num_keys && key > self.keys[i] {
            i += 1;
        }

        i
    }


    fn delete(&mut self, key: i32) {
        if self.is_leaf {
            self.delete_from_leaf(key);
        } else {
            let child_index = self.find_child_index(key);
            self.children[child_index].delete(key);

            if self.children[child_index].num_keys < ORDER / 2 {
                self.balance_child(child_index);
            }
        }
    }

    fn delete_from_leaf(&mut self, key: i32) {
        if let Some(index) = self.search(key) {
            self.keys.remove(index);
            self.num_keys -= 1;
        }
    }

    fn balance_child(&mut self, child_index: usize) {
        let prev_sibling_index = if child_index > 0 {
            child_index - 1
        } else {
            child_index
        };
    
        let next_sibling_index = if child_index < self.num_keys {
            child_index + 1
        } else {
            child_index
        };
    
        let mut child = Box::new(BPlusTree::new_node());
        let mut prev_sibling = Box::new(BPlusTree::new_node());
        let mut next_sibling = Box::new(BPlusTree::new_node());
    
        std::mem::swap(&mut child, &mut self.children[child_index]);
        std::mem::swap(&mut prev_sibling, &mut self.children[prev_sibling_index]);
        std::mem::swap(&mut next_sibling, &mut self.children[next_sibling_index]);
    
        let child = child.as_mut();
        let prev_sibling = prev_sibling.as_mut();
        let next_sibling = next_sibling.as_mut();
    
        // Rest of the code using child, prev_sibling, and next_sibling in the same scope
        if prev_sibling.num_keys > ORDER / 2 {
            child.keys.insert(0, self.keys[child_index - 1]);
            self.keys[child_index - 1] = prev_sibling.keys.pop().unwrap();
            child.num_keys += 1;
            prev_sibling.num_keys -= 1;

            if !child.is_leaf {
                child.children.insert(0, prev_sibling.children.pop().unwrap());
            }
        } else if next_sibling.num_keys > ORDER / 2 {
            child.keys.push(self.keys[child_index]);
            self.keys[child_index] = next_sibling.keys.remove(0);
            child.num_keys += 1;
            next_sibling.num_keys -= 1;

            if !child.is_leaf {
                child.children.push(next_sibling.children.remove(0));
            }
        } else {
            if prev_sibling.num_keys > 0 {
                prev_sibling.keys.push(self.keys.remove(child_index - 1));
                prev_sibling.num_keys += 1;
                prev_sibling.keys.append(&mut child.keys);
                prev_sibling.children.append(&mut child.children);
            } else {
                child.keys.append(&mut next_sibling.keys);
                child.children.append(&mut next_sibling.children);
                self.keys.remove(child_index);
                self.children.remove(next_sibling_index);
                child.num_keys += next_sibling.num_keys;
            }
            self.num_keys -= 1;
        }
    }

    fn serialize_to_file(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_data = serde_json::to_string(self)?;

        let mut file = File::create(filename)?;
        file.write_all(json_data.as_bytes())?;

        Ok(())
    }

    fn deserialize_from_file(filename: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(filename)?;
        let mut json_data = String::new();
        file.read_to_string(&mut json_data)?;

        let bplus_tree: BPlusTree = serde_json::from_str(&json_data)?;

        Ok(bplus_tree)
    }
    
}



fn main() {
    let mut bplus_tree = BPlusTree::new_leaf();

    bplus_tree.insert(10);
    bplus_tree.insert(20);
    bplus_tree.insert(30);
    bplus_tree.insert(5);
    bplus_tree.insert(25);

    println!("B+ Tree traversal:");
    bplus_tree.traverse();

    println!("\n\nSearching for key 20:");
    if let Some(index) = bplus_tree.search(20) {
        println!("Key 20 found at index: {}", index);
    } else {
        println!("Key 20 not found");
    }

    println!("\n\nSearching for key 15:");
    if let Some(index) = bplus_tree.search(15) {
        println!("Key 15 found at index: {}", index);
    } else {
        println!("Key 15 not found");
    }

    let key_to_delete = 20;
    bplus_tree.delete(key_to_delete);

    println!("\nB+ Tree after deleting key {}: ", key_to_delete);
    bplus_tree.traverse();

    let filename = "bplus_tree.json";
    bplus_tree.serialize_to_file(filename);

    // Deserialize B+ tree from the file
    let deserialized_tree = BPlusTree::deserialize_from_file(filename);

    println!("\n\nDeserialized B+ Tree:\n{:?}", deserialized_tree);

}


