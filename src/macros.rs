// Send a request
macro_rules! send_request {
    ($req:expr) => {
        $req
        .send()
        .await
    };
}
// Get text from a response
macro_rules! response_text {
    ($res:expr) => {
        $res
        .ok()?
        .text()
        .await
        .ok()?
    };
    (request $req:expr) => {
        response_text!(send_request!($req))
    }
}
// Create an HTTP client
macro_rules! hclient {
    () => {
        reqwest::Client::new()
    };
}