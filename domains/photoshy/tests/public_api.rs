use photoshy::{PhotoShyDomain, PhotoHandle, PhotoError};

#[test]
fn photoshy_public_api_compiles() {
    let domain = PhotoShyDomain::new();

    // Do NOT call ingest_photo in PASS 2A.
    let _handle = PhotoHandle { id: "x".into() };
    let _err = PhotoError::InvalidInput;

    let _ = domain;
}
