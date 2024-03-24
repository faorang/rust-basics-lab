#[derive(Clone, Copy, Debug)]
struct User {
    id: u32,
}

#[test]
fn pass_by_reference() {
    fn print_user(u: &User) {
        println!("{u:?}");
    }

    let u = User { id: 9000 };

    print_user(&u);
    println!("{u:?}");
}

#[test]
fn borrow_test1() {
    let mut a1 = 1;
    let a2 = &a1;
    let a3 = &a1; // No Problem. Can have multiple borrows
    println!("{a1:?} {a2:?} {a3:?}");

    let mut b1 = 1;
    let b2 = &mut b1;
    // let b3 = &mut b1; // Fail. Cannot mutably borrow when already mutably borrowed
    // println!("{b1:?} {b2:?} {b3:?}");

    let mut c1 = 1;
    let c2 = &c1;
    // let c3 = &mut c1; // Fail. Cannot mutably borrow when already borrowed
    // println!("{c1:?} {c2:?} {c3:?}");

    let mut d1 = 1;
    let d2 = &mut d1;
    // let d3 = &d1; // Fail. Cannot borrow when already mutably borrowed
    // println!("{d1:?} {d2:?} {d3:?}");
}

/*
 * Borrows, mutable or not, exist for the lifetime of their scope.
 * In all of the above cases, the borrows exist until the end of main,
 * which is why we violate the requirements of multiple readers OR one
 * writer. If we limit the scope of the borrows, say by borrowing for a
 * function, the code compiles:
 */

fn echo(id: &i32) {
    println!("{id}")
}

fn mut_echo(id: &mut i32) {
    *id += 1;
    println!("{id}")
}

#[test]
pub fn borrow_test2() {
    let mut a1 = 1;
    let a2 = &a1;
    let a3 = &a1;
    println!("{a1:?} {a2:?} {a3:?}");

    let mut b1 = 1;
    mut_echo(&mut b1);
    mut_echo(&mut b1);
    println!("{b1:?}");

    let mut c1 = 1;
    echo(&c1);
    mut_echo(&mut c1);
    println!("{c1:?}");

    let mut d1 = 1;
    mut_echo(&mut d1);
    echo(&d1);
    println!("{d1:?}");
}
