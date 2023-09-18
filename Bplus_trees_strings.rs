use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{Read, Write};
const ORDER: usize = 2; // order of the tree

// structure of a node in b+ tree

#[derive(Serialize, Deserialize, Debug, Clone)]
struct BPlusTree {
    key_value_pairs: Vec<(String, String)>, // Store key-value pairs as strings
    children: Vec<Box<BPlusTree>>,
    is_leaf: bool,
    num_keys: usize,
}

impl BPlusTree {
    fn new_leaf() -> Self {
        BPlusTree {
            key_value_pairs: Vec::new(),
            children: Vec::new(),
            is_leaf: true,
            num_keys: 0,
        }
    }

    fn new_node() -> Self {
        BPlusTree {
            key_value_pairs: Vec::new(),
            children: Vec::new(),
            is_leaf: false,
            num_keys: 0,
        }
    }

    fn traverse(&self) {
        for (i, (key, value)) in self.key_value_pairs.iter().enumerate() {
            if !self.is_leaf {
                self.children[i].traverse();
            }
            print!("({}: {}) ", key, value);
        }
    
        if !self.is_leaf {
            self.children[self.num_keys].traverse();
        }
    }
    
    fn search(&self, key: &String) -> Option<(String, String)> {
        let mut i = 0;
    
        while i < self.num_keys && key > &self.key_value_pairs[i].0 {
            i += 1;
        }
    
        if i < self.num_keys && key == &self.key_value_pairs[i].0 {
            return Some((self.key_value_pairs[i].0.clone(), self.key_value_pairs[i].1.clone()));
        }
    
        if self.is_leaf {
            return None;
        }
    
        self.children[i].search(key)
    }
    
    fn insert(&mut self, key: String, value: String) {
        if self.is_leaf {
            self.insert_into_leaf(key, value);
        } else {
            let i = self.find_child_index(&key);
            self.children[i].insert(key.clone(), value.clone());
    
            if self.children[i].num_keys == ORDER {
                self.split_child(i);
            }
        }
    }
  
    fn insert_into_leaf(&mut self, key: String, value: String) {
        let i = self.find_insert_position(&key);
        self.key_value_pairs.insert(i, (key, value));
        self.num_keys += 1;
    }

    fn split_child(&mut self, child_index: usize) {
        let mut child = self.children[child_index].clone();
        let mut new_node = BPlusTree::new_node();
    
        for i in ORDER / 2..ORDER - 1 {
            new_node.key_value_pairs.push(child.key_value_pairs[i].clone());
            new_node.children.push(child.children[i].clone());
        }
        new_node.children.push(child.children[ORDER - 1].clone());
    
        child.key_value_pairs.truncate(ORDER / 2);
        child.children.truncate(ORDER / 2 + 1);
    
        self.children.insert(child_index + 1, Box::new(new_node));
        self.key_value_pairs.insert(child_index, child.key_value_pairs.pop().unwrap());
        self.num_keys += 1;
    }
    
    fn find_child_index(&self, key: &String) -> usize {
        let mut i = 0;
    
        while i < self.num_keys && key > &self.key_value_pairs[i].0 {
            i += 1;
        }
    
        i
    }
    
    fn find_insert_position(&self, key: &String) -> usize {
        let mut i = 0;
    
        while i < self.num_keys && key > &self.key_value_pairs[i].0 {
            i += 1;
        }
    
        i
    }
    
    fn delete(&mut self, key: &String) {
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
    
    fn delete_from_leaf(&mut self, key: &String) {
        if let Some(index) = self.key_value_pairs.iter().position(|(k, _)| k == key) {
            self.key_value_pairs.remove(index);
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
            child.key_value_pairs.insert(0, self.key_value_pairs[child_index - 1].clone());
            self.key_value_pairs[child_index - 1] = prev_sibling.key_value_pairs.pop().unwrap();
            child.num_keys += 1;
            prev_sibling.num_keys -= 1;
    
            if !child.is_leaf {
                child.children.insert(0, prev_sibling.children.pop().unwrap());
            }
        } else if next_sibling.num_keys > ORDER / 2 {
            child.key_value_pairs.push(self.key_value_pairs[child_index].clone());
            self.key_value_pairs[child_index] = next_sibling.key_value_pairs.remove(0);
            child.num_keys += 1;
            next_sibling.num_keys -= 1;
    
            if !child.is_leaf {
                child.children.push(next_sibling.children.remove(0));
            }
        } else {
            if prev_sibling.num_keys > 0 {
                prev_sibling.key_value_pairs.push(self.key_value_pairs.remove(child_index - 1));
                prev_sibling.num_keys += 1;
                prev_sibling.key_value_pairs.append(&mut child.key_value_pairs);
                prev_sibling.children.append(&mut child.children);
            } else {
                child.key_value_pairs.append(&mut next_sibling.key_value_pairs);
                child.children.append(&mut next_sibling.children);
                self.key_value_pairs.remove(child_index);
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

    bplus_tree.insert("apple".to_string(), "10".to_string());
    bplus_tree.insert("banana".to_string(), "20".to_string());
    bplus_tree.insert("cherry".to_string(), "30".to_string());
    bplus_tree.insert("date".to_string(), "5".to_string());
    bplus_tree.insert("fig".to_string(), "25".to_string());

    println!("B+ Tree traversal:");
    bplus_tree.traverse();

    println!("\n\nSearching for key 'banana':");
    if let Some((key, value)) = bplus_tree.search(&"banana".to_string()) {
        println!("Key 'banana' found with value: '{}'", value);
    } else {
        println!("Key 'banana' not found");
    }

    println!("\n\nSearching for key 'grape':");
    if let Some((key, value)) = bplus_tree.search(&"grape".to_string()) {
        println!("Key 'grape' found with value: '{}'", value);
    } else {
        println!("Key 'grape' not found");
    }

    let key_to_delete = "banana".to_string();
    bplus_tree.delete(&key_to_delete.clone());

    println!("\nB+ Tree after deleting key '{}':", key_to_delete);
    bplus_tree.traverse();

    let filename = "bplus_tree.json";
    bplus_tree.serialize_to_file(filename).unwrap();

    let deserialized_tree = BPlusTree::deserialize_from_file(filename).unwrap();

    println!("\n\nDeserialized B+ Tree:\n{:?}", deserialized_tree);
}
