use rand::Rng;

// Groups contains its and id and nodes
// For example here we taking node as a string, node can be anything.
#[derive(Debug)]
struct Group {
    id: usize,
    nodes: Vec<String>,
}

#[derive(Debug, Default)]
struct Header {
    laegest_node_count: usize,
    group_iteration: usize,
}

#[derive(Debug)]
struct System {
    // header contain 1.largest count of nodes present in groups and 
    // 2. second one is for stores the iteration over the group.
    header: Header,
    groups: Vec<Group>,
}

impl System {
    fn new(group_count: usize) -> System {
        let mut groups = Vec::new();
        for i in 1..=group_count {
            groups.push(Group {
                id: i,
                nodes: vec![],
            });
        }
        System {
            header: Header::default(),
            groups,
        }
    }

    // Adding the nodes
    fn add_node(&mut self, node: String) {
        // If the iteration value is equal to the length of the group array then 
        // the node will be added to any of the groups.
        if self.header.group_iteration == self.groups.len() {
            let group_index = rand::thread_rng().gen_range(0..self.groups.len());
            self.groups[group_index].nodes.push(node);
            self.header.laegest_node_count += 1;
            self.header.group_iteration = 1;
        } 
        // else the if iteration value is less than to the length of the group array then
        // we find a group that has have less nodes.
        else if self.header.group_iteration < self.groups.len() {
            let mut group_index;
            loop {
                group_index = rand::thread_rng().gen_range(0..self.groups.len());
                if self.groups[group_index].nodes.len() <= self.header.laegest_node_count {
                    break;
                }
            }
            self.groups[group_index].nodes.push(node);
            self.header.group_iteration += 1;
        }
    }
}

fn main() {
    // Create a new system with 10 groups
    let mut system = System::new(10);

    // Adding nodes 
    for i in 1..=30 {
        system.add_node(i.to_string());
    }

    //Printing the groups and their nodes.
    for group in system.groups.iter() {
        println!("{:?}", group);
    }
}
