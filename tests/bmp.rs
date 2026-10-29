#[cfg(test)]
use imagesize::{size, ImageSize};

#[test]
#[cfg(feature = "bmp")]
fn bmp_test() {
    let dim = size("tests/images/bmp/test.bmp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 512,
            height: 512
        }
    );
}

#[test]
#[cfg(feature = "bmp")]
fn bmp_top_down_test() {
    // biHeight is -1, which marks a top-down bitmap
    let dim = size("tests/images/bmp/top_down.bmp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1,
            height: 1
        }
    );
}

#[test]
#[cfg(feature = "bmp")]
fn bmp_negative_width_test() {
    let mut data = std::fs::read("tests/images/bmp/top_down.bmp").unwrap();
    data[0x12..0x16].copy_from_slice(&(-1i32).to_le_bytes());
    assert!(imagesize::blob_size(&data).is_err());
}

#[test]
#[cfg(feature = "bmp")]
fn bmp_core_header_test() {
    // 12-byte BITMAPCOREHEADER with 16-bit width and height
    let dim = size("tests/images/bmp/core_header.bmp").unwrap();
    assert_eq!(
        dim,
        ImageSize {
            width: 1,
            height: 1
        }
    );
}
