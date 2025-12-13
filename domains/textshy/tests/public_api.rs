use textshy::{TextDomain, TextHandle, TextError};

#[test]
fn textshy_public_api_compiles() {
    let domain = TextDomain::new();

    let _handle = TextHandle { id: "x".into() };
    let _err = TextError::InvalidInput;

    let _ = domain;
}
