use getaver;

fn main() {
    let mut a = getaver::AverCollect::new();
    a.add(2);
    a.add(4);
    a.add(6);
    a.remove(4);
    println!("{}", a);
}
