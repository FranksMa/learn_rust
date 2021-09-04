fn main() {
    let a = 42;
    let ref b = a;
    let c = &a;
    assert_eq!(b, c);

    let mut a = [1,2,3];
    let ref mut b = a;
    b[0] = 0;
    assert_eq!(a, [0,2,3]);
}