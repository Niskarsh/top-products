use top_products::LinkedList;
fn main() {
    let rank1 = vec![50, 23, 9];
    let rank2 = vec![4, 2];
    let rank3 = vec![3];
    let mut rank1_ll = LinkedList::init();
    let mut rank2_ll = LinkedList::init();
    let mut rank3_ll = LinkedList::init();
    for rank in &rank1 {
        rank1_ll.add(rank);
    }
    for rank in &rank2 {
        rank2_ll.add(rank);
    }
    for rank in &rank3 {
        rank3_ll.add(rank);
    }
    
    println!("{:?}", rank1_ll);
}
