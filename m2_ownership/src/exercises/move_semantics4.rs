// Make me compile only by reordering the lines in `test()`, but without
// adding, changing or removing any of them.

#[cfg(feature = "skip")]
#[test]
fn test1() {
    let mut x = 100;
    let y = &mut x;
    let z = &mut x;
    *y += 100;
    *z += 1000;
    assert_eq!(x, 1200);
}
