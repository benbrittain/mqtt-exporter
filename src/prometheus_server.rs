use hyper::{header::CONTENT_TYPE, Body, Request, Response};

pub async fn serve(_conn: Request<Body>) -> Result<hyper::Response<hyper::Body>, hyper::Error> {
    use prometheus::Encoder;

    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, encoder.format_type())
        .body(Body::from(buffer))
        .unwrap();
    Ok(resp)
}
