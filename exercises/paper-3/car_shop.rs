struct fuel {
    Diesel,
    Gasoline,
    LPG,
    Methane,
    Electric
}

enum IP_version {
    IPv4([u8; 4]),
    IPv6([u16; 8]),
}

struct PointUnnamedFields(f64, f64, f64);
