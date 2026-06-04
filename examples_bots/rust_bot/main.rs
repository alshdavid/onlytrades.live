use onlytrades_bot::Context;

#[onlytrades_bot::main]
async fn main(ctx: Context) -> Result<(), Box<dyn std::error::Error>> {
  ctx.subscribe();
  println!("Hello world");
  Ok(())
}
