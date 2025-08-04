use wav1d::utils::bits::BitsReader;



#[test]
fn onebyte1() {
    let a = [0b10100111, 0b11001100];
    let r = BitsReader::from(a.as_slice());
    let br = 0b00111110u8;

    let b = r.get_1byte(3, 8);
    assert_eq!(b, br, "{b:08b} {br:08b}");
}


#[test]
fn onebyte2() {
    let a = [0b10100111, 0b11001100];
    let r = BitsReader::from(a.as_slice());
    let br = 0b00001111u8;

    let b = r.get_1byte(3, 6);
    assert_eq!(b, br, "{b:08b} {br:08b}");
}


#[test]
fn onebyte3() {
    let a = [0b10100111, 0b11001100];
    let r = BitsReader::from(a.as_slice());
    let br = 0b00000001u8;

    let b = r.get_1byte(10, 3);
    assert_eq!(b, br, "{b:08b} {br:08b}");
}


#[test]
fn nbyte1() {
    let a = [0b10100111, 0b11001100];
    let r = BitsReader::from(a.as_slice());
    let mut b = [0u8; 2];
    let br = [0b00111110u8, 0b00000000];

    r.get_nbyte(3, 8, &mut b);
    assert_eq!(b, br, "{:08b} {:08b}", b[0], br[0]);

    let res = u16::from_le_bytes(b);
    assert_eq!(res, 62, "res");
}




#[test]
fn bit1() {
    let a = [0b10100111, 0b11001100];
    let mut r = BitsReader::from(a.as_slice());
    let t = [true, false, true, false, false, true, true, true, true, true, false, false, true, true, false, false];
    for (ii, i) in t.into_iter().enumerate() {
        assert_eq!(i, r.read_bit(), "{ii}");
    }
}