use std::{
    cell::RefCell,
    collections::HashMap,
    ops::{Deref, DerefMut},
    rc::Rc,
};

#[derive(Debug)]
struct AnnotatedTree {
    children: Vec<AnnotatedTree>,
    size: usize,
}

enum Tree {
    Directory(
        HashMap<String, Rc<RefCell<Tree>>>,
        Option<Rc<RefCell<Tree>>>,
    ),
    File(usize),
}
impl Tree {
    fn is_tree(&self) -> bool {
        match self {
            Tree::Directory(_, _) => true,
            Tree::File(_) => false,
        }
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directory(arg0, _) => f.debug_tuple("Directory").field(arg0).finish(),
            Self::File(arg0) => f.debug_tuple("File").field(arg0).finish(),
        }
    }
}

fn add_file(s: &Rc<RefCell<Tree>>, id: String, val: usize) {
    let mut r = s.borrow_mut();
    match r.deref_mut() {
        Tree::Directory(v, _) => v.insert(id, Rc::new(RefCell::new(Tree::File(val)))),
        _ => unreachable!(),
    };
}

fn add_dir(s: &Rc<RefCell<Tree>>, id: String) {
    let c = s.clone();
    let mut r = s.borrow_mut();
    match r.deref_mut() {
        Tree::Directory(v, _) => v.insert(
            id,
            Rc::new(RefCell::new(Tree::Directory(HashMap::new(), Some(c)))),
        ),
        _ => unreachable!(),
    };
}
fn get_dir(s: &Rc<RefCell<Tree>>, id: String) -> Rc<RefCell<Tree>> {
    match s.borrow().deref() {
        Tree::Directory(v, _) => v.get(&id).unwrap().clone(),
        _ => unreachable!(),
    }
}

fn get_parent(s: &Rc<RefCell<Tree>>) -> Rc<RefCell<Tree>> {
    match s.borrow().deref() {
        Tree::Directory(_, Some(p)) => p.clone(),
        Tree::Directory(_, None) => s.clone(),
        _ => unreachable!(),
    }
}

fn get_sizes_recurse(s: &Rc<RefCell<Tree>>) -> usize {
    match s.borrow().deref() {
        Tree::Directory(children, _) => children.values().map(get_sizes_recurse).sum(),
        Tree::File(s) => *s,
    }
}

fn get_sizes(s: &Rc<RefCell<Tree>>) -> Vec<usize> {
    let mut vec = Vec::new();
    match s.borrow().deref() {
        Tree::Directory(children, _) => children
            .values()
            .filter(|c| c.borrow().is_tree())
            .map(|c| (get_sizes_recurse(c), get_sizes(c)))
            .for_each(|(size, mut rest)| {
                vec.push(size);
                vec.append(&mut rest)
            }),
        Tree::File(_) => {}
    };
    vec
}

pub fn part1() -> usize {
    let root = Rc::new(RefCell::new(Tree::Directory(HashMap::new(), None)));
    let mut current = root.clone();
    for s in include_str!("input/day7.input")
        .split("$ ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
    {
        for s in s.split('\n') {
            let mut splat = s.split(' ');
            let prefix = splat.next().unwrap();
            let parsed = prefix.parse::<usize>();
            match (prefix, parsed) {
                ("cd", _) => {
                    match splat.next().unwrap() {
                        "/" => {
                            current = root.clone();
                        }
                        ".." => {
                            let c = get_parent(&current);
                            current = c
                        }
                        a => {
                            let c = get_dir(&current, a.to_string());
                            current = c
                        }
                    };
                }
                ("ls", _) => {} // we can safely ignore this case
                ("dir", _) => add_dir(&current, splat.next().unwrap().to_string()),
                (_, Ok(parsed)) => {
                    add_file(&current, splat.next().unwrap().to_string(), parsed);
                }
                _ => unreachable!(),
            };
        }
    }
    get_sizes(&root).into_iter().filter(|u| *u <= 100000).sum()
}

fn annonate_tree(root: &Rc<RefCell<Tree>>) -> AnnotatedTree {
    match root.borrow().deref() {
        Tree::Directory(treeees, _) => {
            let mut size = 0;
            let mut children: Vec<AnnotatedTree> = Vec::new();
            for c in treeees.iter().map(|(_, c)| c) {
                size += get_sizes_recurse(c);
                children.push(annonate_tree(c));
            }
            AnnotatedTree { children, size }
        }
        Tree::File(f) => AnnotatedTree {
            children: Vec::new(),
            size: *f,
        },
    }
}

fn find_smoll(a: &AnnotatedTree, current: usize) -> Vec<usize> {
    let AnnotatedTree { children, size } = &a;
    if !children.is_empty() {
        let total = 70000000;
        let needed = 30000000;
        let mut v = Vec::new();
        if total - needed >= current - size {
            v.push(*size);
        }
        for c in children {
            v.append(&mut find_smoll(c, current))
        }
        v
    } else {
        Vec::new()
    }
}

pub fn part2() -> usize {
    let root = Rc::new(RefCell::new(Tree::Directory(HashMap::new(), None)));
    let mut current = root.clone();
    for s in include_str!("input/day7.input")
        .split("$ ")
        .filter(|s| !s.is_empty())
        .map(|s| s.trim())
    {
        for s in s.split('\n') {
            let mut splat = s.split(' ');
            let prefix = splat.next().unwrap();
            let parsed = prefix.parse::<usize>();
            match (prefix, parsed) {
                ("cd", _) => {
                    match splat.next().unwrap() {
                        "/" => {
                            current = root.clone();
                        }
                        ".." => {
                            let c = get_parent(&current);
                            current = c
                        }
                        a => {
                            let c = get_dir(&current, a.to_string());
                            current = c
                        }
                    };
                }
                ("ls", _) => {} // we can safely ignore this case
                ("dir", _) => add_dir(&current, splat.next().unwrap().to_string()),
                (_, Ok(parsed)) => {
                    add_file(&current, splat.next().unwrap().to_string(), parsed);
                }
                _ => unreachable!(),
            };
        }
    }

    let a = annonate_tree(&root);
    find_smoll(&a, a.size).into_iter().min().unwrap()
}
