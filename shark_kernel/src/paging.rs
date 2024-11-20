#[repr(C, align(4096))]
struct PageTable {
    entries: [u64; 512],
}
