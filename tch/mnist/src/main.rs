use anyhow::Result;

mod mnist_conv;

fn main() -> Result<()> {
    mnist_conv::run()
}