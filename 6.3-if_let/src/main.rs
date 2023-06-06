fn main() {
    if_let();
}

fn if_let() {
    let config_max = Some(3u8);
    // let config_max = None;
    if let Some(max) = config_max {
        println!("Max is {}",max);
    }
}
