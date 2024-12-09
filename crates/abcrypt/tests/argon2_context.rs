// SPDX-FileCopyrightText: 2024 Shun Sakai
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

use abcrypt::{
    argon2::{Algorithm, Version},
    Argon2,
};

// Generated using `abcrypt` crate version 0.4.0.
const TEST_DATA_ENC: &[u8] = include_bytes!("data/v1/data.txt.abcrypt");

#[test]
fn success() {
    let argon2 = Argon2::new(TEST_DATA_ENC);
    assert!(argon2.is_ok());
}

#[test]
fn variant() {
    let argon2 = Argon2::new(TEST_DATA_ENC).unwrap();
    assert_eq!(argon2.variant(), Algorithm::Argon2id);
}

#[test]
fn version() {
    let argon2 = Argon2::new(TEST_DATA_ENC).unwrap();
    assert_eq!(argon2.version(), Version::V0x13);
}