use futures::executor::block_on;

async fn hello_world() {
    println!("hello world!");
}

#[derive(Debug)]
struct Song {}

async fn learn_song() -> Song {
    println!("learn_song");
    Song {}
}

async fn sing_song(song: Song) {
    println!("sing: {:?}", song);
}

async fn dance() {
    println!("dance");
}

async fn learn_and_sing() {
    // 阻塞
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // 阻塞
    futures::join!(f1, f2);
}

fn main() {
    // let future = hello_world();
    // block_on(future);

    // 逐步阻塞
    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());

    // 等待阻塞
    block_on(async_main());
}
