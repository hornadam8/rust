use rand::Rng;
use std::time::SystemTime;

fn main() {
    let start = SystemTime::now();
    let nav_items = gen_rand_tree(1000);
    let elapsed = start.elapsed().unwrap();
    println!("set up in {elapsed:?}");
    let start = SystemTime::now();
    print_tree(nav_items);
    let elapsed = start.elapsed().unwrap();
    println!("Built tree in {elapsed:?}")
}

#[derive(Clone)]
struct NavItem {
    id: i32,
    content: String,
    parent_id: Option<i32>,
}

fn print_tree(items: Vec<NavItem>) {
    let mut roots = vec![];
    let mut children = vec![];
    for item in items.into_iter() {
        if item.parent_id.is_some() {
            children.push(item);
        } else {
            roots.push(item);
        }
    }
    for root in roots.iter() {
        print_with_children(root, &children, 0);
    }
}

fn print_with_children(item: &NavItem, items: &Vec<NavItem>, level: i32) {
    let mut space = "".to_string();
    for _ in 0..level {
        space += "   ";
    }
    println!("{}{}", space, item.content);
    for it in items.iter() {
        if it.parent_id.unwrap_or(-1) == item.id {
            print_with_children(it, items, level + 1);
        }
    }
}

fn gen_rand_tree(size: i32) -> Vec<NavItem> {
    let mut nav_items = vec![];
    nav_items.push(NavItem {
        id: 1,
        content: "item 1 root".to_string(),
        parent_id: None,
    });
    for i in 2..=size {
        let rn = rand::thread_rng().gen_range(1..=100);
        if rn > 90 {
            nav_items.push(NavItem {
                id: i,
                content: format!("item {i} root"),
                parent_id: None,
            });
        } else {
            let p = rand::thread_rng().gen_range(1..=size);
            nav_items.push(NavItem {
                id: i,
                content: format!("item {i} sub {p}"),
                parent_id: Some(p),
            });
        }
    }
    nav_items
}
