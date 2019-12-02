use std::collections::BTreeMap;

const UPPER: u32 = 0xFFFFFFFFu32;
//const UPPER: u32 = 0xFFFFFFFFu32;

fn main() {
    let mut seen = BTreeMap::<u32, u32>::new();
    for a in 0u32..=UPPER {
        if 0 == a & 0x003FFFFF {
            eprintln!("--- iter 0x{:08x} ({} elems)", a, seen.len());
        }
        let b = f32::from_bits(a).ln().to_bits();
        *seen.entry(b).or_default() += 1;
    }
    eprintln!("Ran for {0} (0x{0:08x}) iterations, saw only {1} (0x{1:08x}) different values", UPPER, seen.len());
    for (k, v) in seen {
        println!("{:08x}: {} times", k, v);
    }
}
