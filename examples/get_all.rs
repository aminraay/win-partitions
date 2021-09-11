use win_partiotions::windows_partition::get_partitions;

fn main() {
    let list = get_partitions();
    for i in list.unwrap() {
        print!("Drive Letter: {}", i.letter);
    }
}