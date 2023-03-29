mod cellembly;
mod utils;
//the AGATA computer

//modified DNA replicators that make some adjustments

//uses radix-complement

fn main() {
    let num = utils::to_u32("AGATA").unwrap();
    println!("{num:?}");
    let seq = utils::to_seq(num);
    println!("{seq}");
}
