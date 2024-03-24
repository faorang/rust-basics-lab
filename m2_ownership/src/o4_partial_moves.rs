use lib::delim;

#[derive(Debug)]
struct Config {
    max_bookmarks: usize,
}

#[derive(Debug)]
struct User {
    config: Config,
    bookmarks: Vec<String>,
}

#[test]
fn partial_move_test1() {
    let mut user = User {
        bookmarks: vec![],
        config: Config {
            max_bookmarks: 1000,
        },
    };

    let config = user.config; // move `config` out of `user`
                              // println!("{:?}, {:?}", user, config);
    println!("{:?}, {:?}", user.bookmarks, config);
}

/*
 * The above code moves `user.config` to a new owner, the `config` variable.
 * When we try to print `user` and `config`, the compiler tells us that `user`
 * has been "partially moved" and cannot be used. If we change the last
 * line to print `user.bookmarks` instead of `user`, the code works:
 *
 *  println!("{:?} {:?}", user.bookmarks, config);
 */

/*
 * While we're able to partially move and borrow, mutability is, by default, "inherited".
 * We cannot control the mutability of individual fields. If we don't declare our user as
 * mutable, then bookmarks isn't mutable:
 */

#[test]
fn mutability_is_inherited() {
    let user = User {
        bookmarks: vec![],
        config: Config {
            max_bookmarks: 1000,
        },
    };

    // will not work as user isn't mutable
    // user.bookmarks.push("https://www.openmymind.net".to_owned());
}

/*
 * However, when we move, we can change mutability. The following will work:
 */

fn add(mut user: User) {
    user.bookmarks.push("https://www.openmymind.net".to_owned());

    delim!();
    println!("{:?}", user);
    delim!();
}

#[test]
fn mutability_can_be_changed_when_moved() {
    let user = User {
        bookmarks: vec![],
        config: Config {
            max_bookmarks: 1000,
        },
    };

    add(user);
    /*
     * This is important: it isn't the data which is or isn't mutable, it's the binding.
     */
}
