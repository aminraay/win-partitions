use win_partiotions::windows_partition::get_volume_name;

fn main() {
    let list = get_volume_name();
    for i in list.unwrap() {
        print!("Drive Letter: {}", i.letter);
    }
}