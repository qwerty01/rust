// edition:2018
#![warn(clippy::unnecessary_async)]

async fn foo() -> i32 {
    4
}

async fn bar() -> i32 {
    foo().await
}

fn main() {
    foo();
    bar();
}
