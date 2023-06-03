pub fn add(left: usize, right: usize) -> usize {
    left + right 
}

pub fn bubble_sort<T: PartialOrd>(v: &mut [T]){
    for _ in 0..v.len(){ // if the array is a million, this will check a million checks
        for i in 0..v.len()-1{
            if v[i] > v[i+1] {
                v.swap(i, i+1)
            }
        }
    }
}


fn main(){
    let mut v = vec![4,6,5,62,2,3,11];
    bubble_sort(&mut v);
    for i in 0..v.len() {
        println!("{}", v[i]);
    }
}
