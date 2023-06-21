use ndarray::{self, array, Array};

//fn _foo<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
//where
//T: Clone + Copy,
//{
//let mut n = Vec::new();
//let mut temp = 0;
//for _i in &v[0] {
//let mut t = Vec::new();
//for j in v.clone() {
//t.push(j[temp]);
//}
//n.push(t);
//temp += 1;
//}
//n
//}

fn main() {
    //let v = vec![
    //vec![1, 2, 3],
    //vec![4, 5, 6],
    //vec![7, 8, 9],
    //vec![10, 11, 12],
    //];
    //let n = foo(v);
    //println!("{:?}", n);
    //println!("{:?}", n)
    //foo(n)
    let a = array![1, 2, 3];
    foo(a);
}
