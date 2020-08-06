mod graphql;

use http_client::HttpClient;

#[cfg(feature = "native-client")]
use http_client::native::NativeClient;

#[cfg(feature = "h1-client")]
use http_client::h1::H1Client;

pub struct Client<T: HttpClient> {
    client: surf::Client<T>,
}

#[cfg(feature = "h1-client")]
impl Client<H1Client> {
    pub fn new() -> Self {
        Self::with_client(H1Client::new())
    }
}

#[cfg(feature = "native-client")]
impl Client<NativeClient> {
    pub fn new() -> Self {
        Self::with_client(NativeClient::new())
    }
}

impl<C: HttpClient> Client<C> {
    pub fn with_client(client: C) -> Self {
        let client = surf::Client::with_client(client);
        Self { client }
    }
}
