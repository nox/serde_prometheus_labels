use serde::Serialize;
use serde_prometheus_labels::to_string;

#[test]
fn ok() {
    #[derive(Serialize)]
    struct Labels {
        method: Method,
        path: String,
    }

    #[derive(Serialize)]
    enum Method {
        #[serde(rename = "GET")]
        Get,
    }

    let labels = Labels {
        method: Method::Get,
        path: "/metrics".to_string(),
    };

    let serialized = to_string(&labels).unwrap();

    assert_eq!(serialized, r#"method="GET",path="/metrics""#);
}

#[test]
fn invalid_key() {
    #[derive(Serialize)]
    struct InvalidKey {
        #[serde(rename = "_🦾")]
        flex: &'static str,
    }

    let err = to_string(InvalidKey {
        flex: "prosthetics",
    })
    .unwrap_err();

    assert_eq!(err.to_string(), r#"invalid key ("_🦾")"#);
}

#[test]
fn escapes() {
    #[derive(Serialize)]
    struct StrField {
        field: &'static str,
    }

    let serialized = to_string(StrField {
        field: "slash: \\, newline: \n, quote: \"",
    })
    .unwrap();

    assert_eq!(serialized, r#"field="slash: \\, newline: \n, quote: \"""#)
}
