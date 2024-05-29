enum DisplayOrder {
    Ascend,
    Descend,
}

enum CommandType {
    Insert(i32),
    Display(DisplayOrder),
}

struct SortTree {
    value: i32,
    left_node: Option<SortTree>,
    right_node: Option<SortTree>,
}

pub fn run_cli(command: String) {}

fn insert_tree(value: i32) {}

fn display_tree(order: DisplayOrder) {}
