mod baboon;

#[tokio::main]
async fn main() {
    println!("Baboons crossing a canyon");
    baboon::baboons_crossing_canyon(20, 20).await;
}
