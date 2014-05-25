static METRIC_PREFIXES: &'static [&'static str] = &[
    "", "K", "M", "G", "T", "P", "E", "Z", "Y"
];

static IEC_PREFIXES: &'static [&'static str] = &[
    "", "Ki", "Mi", "Gi", "Ti", "Pi", "Ei", "Zi", "Yi"
];

fn formatBytes(mut amount: u64, kilo: u64, prefixes: &[&str]) -> StrBuf {
    let mut prefix = 0;
    while amount > kilo {
        amount /= kilo;
        prefix += 1;
    }
    format!("{}{}", amount, prefixes[prefix])
}

pub fn formatBinaryBytes(amount: u64) -> StrBuf {
    formatBytes(amount, 1024, IEC_PREFIXES)
}

pub fn formatDecimalBytes(amount: u64) -> StrBuf {
    formatBytes(amount, 1000, METRIC_PREFIXES)
}
