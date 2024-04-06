use tools_programmer_homework::run;

static PORT: u16 = 9999;

#[tokio::main]
async fn main() {
    run(PORT).await;
}
