mod baboon;
#[tokio::main]
async fn main() {
    println!("Baboons crossing a canyon");
    baboon::baboons_crossing(4, 6).await;

}
