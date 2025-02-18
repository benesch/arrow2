use arrow2::array::MutableArray;
use arrow2::array::MutableUtf8ValuesArray;
use arrow2::datatypes::DataType;

#[test]
fn capacity() {
    let mut b = MutableUtf8ValuesArray::<i32>::with_capacity(100);

    assert_eq!(b.values().capacity(), 0);
    assert!(b.offsets().capacity() >= 101);
    b.shrink_to_fit();
    assert!(b.offsets().capacity() < 101);
}

#[test]
fn offsets_must_be_monotonic_increasing() {
    let offsets = vec![0, 5, 4];
    let values = b"abbbbb".to_vec();
    assert!(MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values).is_err());
}

#[test]
fn data_type_must_be_consistent() {
    let offsets = vec![0, 4];
    let values = b"abbb".to_vec();
    assert!(MutableUtf8ValuesArray::<i32>::try_new(DataType::Int32, offsets, values).is_err());
}

#[test]
fn must_be_utf8() {
    let offsets = vec![0, 2];
    let values = vec![207, 128];
    assert!(MutableUtf8ValuesArray::<i32>::try_new(DataType::Int32, offsets, values).is_err());
}

#[test]
fn as_box() {
    let offsets = vec![0, 2];
    let values = b"ab".to_vec();
    let mut b = MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values).unwrap();
    let _ = b.as_box();
}

#[test]
fn as_arc() {
    let offsets = vec![0, 2];
    let values = b"ab".to_vec();
    let mut b = MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values).unwrap();
    let _ = b.as_arc();
}

#[test]
fn extend_trusted_len() {
    let offsets = vec![0, 2];
    let values = b"ab".to_vec();
    let mut b = MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values).unwrap();
    b.extend_trusted_len(vec!["a", "b"].into_iter());

    let offsets = vec![0, 2, 3, 4];
    let values = b"abab".to_vec();
    assert_eq!(
        b.as_box(),
        MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values)
            .unwrap()
            .as_box()
    )
}

#[test]
fn from_trusted_len() {
    let mut b = MutableUtf8ValuesArray::<i32>::from_trusted_len_iter(vec!["a", "b"].into_iter());

    let offsets = vec![0, 1, 2];
    let values = b"ab".to_vec();
    assert_eq!(
        b.as_box(),
        MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values)
            .unwrap()
            .as_box()
    )
}

#[test]
fn extend_from_iter() {
    let offsets = vec![0, 2];
    let values = b"ab".to_vec();
    let mut b = MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values).unwrap();
    b.extend_trusted_len(vec!["a", "b"].into_iter());

    let a = b.clone();
    b.extend_trusted_len(a.iter());

    let offsets = vec![0, 2, 3, 4, 6, 7, 8];
    let values = b"abababab".to_vec();
    assert_eq!(
        b.as_box(),
        MutableUtf8ValuesArray::<i32>::try_new(DataType::Utf8, offsets, values)
            .unwrap()
            .as_box()
    )
}
