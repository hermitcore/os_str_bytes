#![allow(dead_code)]

use std::borrow::Cow;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::ops::Deref;
use std::path::Path;
use std::path::PathBuf;

use os_str_bytes::EncodingError;
use os_str_bytes::OsStrBytes;
use os_str_bytes::OsStringBytes;

pub(crate) const WTF8_STRING: &[u8] = b"foo\xED\xA0\xBD\xF0\x9F\x92\xA9bar";

pub(crate) fn from_bytes(
    string: &[u8],
) -> Result<Cow<'_, OsStr>, EncodingError> {
    let os_string = OsStr::from_bytes(string);

    assert_eq!(
        os_string.as_ref().map(Deref::deref),
        Path::from_bytes(string).as_ref().map(|x| x.as_os_str()),
    );

    os_string
}

pub(crate) fn from_vec(string: Vec<u8>) -> Result<OsString, EncodingError> {
    let os_string = OsString::from_vec(string.clone());

    let path = PathBuf::from_vec(string);
    assert_eq!(os_string, path.map(PathBuf::into_os_string));

    os_string
}

pub(crate) fn test_bytes(string: &[u8]) -> Result<(), EncodingError> {
    let os_string = from_bytes(string)?;
    assert_eq!(string.len(), os_string.len());
    assert_eq!(string, &*os_string.to_bytes());
    Ok(())
}

pub(crate) fn test_vec(string: &[u8]) -> Result<(), EncodingError> {
    let os_string = from_vec(string.to_vec())?;
    assert_eq!(string.len(), os_string.len());
    assert_eq!(string, &*os_string.into_vec());
    Ok(())
}

pub(crate) fn test_utf8_bytes(string: &str) {
    let os_string = OsStr::new(string);
    let string = string.as_bytes();
    assert_eq!(Ok(Cow::Borrowed(os_string)), from_bytes(string));
    assert_eq!(string, &*os_string.to_bytes());
}

pub(crate) fn test_utf8_vec(string: &str) {
    let os_string = string.to_owned().into();
    let string = string.as_bytes();
    assert_eq!(Ok(&os_string), from_vec(string.to_vec()).as_ref());
    assert_eq!(string, &*os_string.into_vec());
}
