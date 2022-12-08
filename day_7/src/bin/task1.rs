use aoc_lib::read_arg_file;
use std::cell::RefCell;
use std::io::{self, prelude::*, BufReader};
use std::rc::Rc;

#[derive(PartialEq, Debug)]
enum FileType {
    Dir,
    File,
}

#[derive(Debug)]
struct FsNode {
    name: Option<String>,
    file_type: FileType,
    size: Option<u32>,
    children: Vec<Rc<RefCell<FsNode>>>,
    parent: Option<Rc<RefCell<FsNode>>>,
}

impl FsNode {
    pub fn new(
        name: String,
        file_type: FileType,
        size: u32,
        parent: Option<Rc<RefCell<FsNode>>>,
    ) -> FsNode {
        return FsNode {
            name: Some(name),
            file_type: file_type,
            size: Some(size),
            children: Vec::new(),
            parent: parent,
        };
    }

    pub fn add_node(&mut self, new_node: Rc<RefCell<FsNode>>) {
        self.children.push(new_node);
    }

    pub fn print(&self, deepth: usize) {
        let size: u32 = if let Some(size) = self.size { size } else { 0 };
        let name = if let Some(name) = &self.name {
            name.clone()
        } else {
            "".to_string()
        };
        let file_type = if self.file_type == FileType::Dir {
            "dir".to_string()
        } else {
            // self.file_type == FileType::file
            "file".to_string()
        };
        let deepth_spaces = std::iter::repeat(" ").take(deepth).collect::<String>();
        if self.file_type == FileType::File {
            println!("{}- {} ({}, size={})", deepth_spaces, name, file_type, size);
        } else {
            println!("{}- {} ({})", deepth_spaces, name, file_type);
            for child in self.children.iter() {
                //.map(|tn| {
                child.borrow().print(deepth + 2);
            }
        }
    }
}

fn cd_dispatcher(
    cmd: &str,
    node: Rc<RefCell<FsNode>>,
    deepth: usize,
) -> (Rc<RefCell<FsNode>>, usize) {
    let cmd_vec: Vec<&str> = cmd.split(" ").collect();
    assert_eq!(cmd_vec[1].eq("cd"), true);
    let dir_name: &str = cmd_vec[2];
    if dir_name.eq("..") {
        match node.borrow().parent {
            None => {
                return (node.clone(), deepth);
            }
            Some(ref parent) => {
                return (parent.clone(), deepth - 1);
            }
        };
    } else {
        for child in &node.borrow().children {
            if let Some(name) = &child.borrow().name {
                if dir_name.eq(name) {
                    if child.borrow().file_type == FileType::Dir {
                        return (child.clone(), deepth + 1);
                    }
                }
            }
        }
    }
    return (node, deepth);
}

fn main() {
    let filesystem = FsNode::new("/".to_string(), FileType::Dir, 0, None);
    let reader = read_arg_file().unwrap();
    let cmd_handler = |(acc, deepth), x: Result<String, _>| {
        if let Ok(x) = x {
            let cmd_vec: Vec<&str> = x.split(" ").collect::<Vec<&str>>();
            if cmd_vec[0].eq("$") {
                if cmd_vec[1].eq("cd") {
                    return cd_dispatcher(&x, acc, deepth);
                } else if cmd_vec[1].eq("ls") {
                    return (acc, deepth);
                }
                return (acc, deepth);
            } else {
                let ls_result: Vec<&str> = x.split(" ").collect::<Vec<&str>>();
                if ls_result[0].eq("dir") {
                    let new_node = Rc::new(RefCell::new(FsNode::new(
                        ls_result[1].to_string(),
                        FileType::Dir,
                        0,
                        Some(acc.clone()),
                    )));
                    acc.borrow_mut().add_node(Rc::clone(&new_node));
                    return (acc, deepth);
                } else {
                    let new_node = Rc::new(RefCell::new(FsNode::new(
                        ls_result[1].to_string(),
                        FileType::File,
                        ls_result[0].parse::<u32>().unwrap(),
                        None,
                    )));
                    acc.borrow_mut().add_node(Rc::clone(&new_node));
                    return (acc, deepth);
                }
                return (acc, deepth);
            }
        }
        (acc, deepth)
    };
    let (filesystem, deepth) = reader
        .lines()
        .fold((Rc::new(RefCell::new(filesystem)), 0), cmd_handler);

    let (filesystem, deepth) = cd_dispatcher("$ cd ..", filesystem, deepth);
    let (filesystem, deepth) = cd_dispatcher("$ cd ..", filesystem, deepth);

    filesystem.borrow().print(0);
}
