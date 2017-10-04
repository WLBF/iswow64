extern crate iswow64;

fn main() {
    let result = iswow64::iswow64();
    println!("{:?}", result);

    #[cfg(target_arch = "x86")]
    assert_eq!(result.unwrap(), true);

    #[cfg(target_arch = "x86_64")]
    assert_eq!(result.unwrap(), false);
}