use crate::{
    http_client::{HttpClient, SendRequest},
    Package, Registry, Result, Version,
};
use serde::Deserialize;

#[cfg(not(test))]
const REGISTRY_URL: &str = "https://registry.npmjs.org";

#[derive(Deserialize)]
struct Response {
    version: String,
}

/// The NPM package registry.
pub struct Npm;

#[cfg(not(test))]
fn get_base_url() -> String {
    REGISTRY_URL.to_string()
}

#[cfg(test)]
fn get_base_url() -> String {
    mockito::server_url()
}

impl Registry for Npm {
    const NAME: &'static str = "npm";

    fn get_latest_version<T: SendRequest>(
        http_client: HttpClient<T>,
        pkg: &Package,
        _current_version: &Version,
    ) -> Result<Option<String>> {
        let url = format!("{}/{}/latest", get_base_url(), pkg);
        let resp = http_client.get::<Response>(&url)?;

        Ok(Some(resp.version))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http_client;
    use crate::test_helper::{mock_npm, mock_pypi};
    use std::time::Duration;

    const PKG_NAME: &str = "turbo";
    const FIXTURES_PATH: &str = "tests/fixtures/registry/npm";
    const TIMEOUT: Duration = Duration::from_secs(5);

    #[test]
    fn failure_test() {
        let pkg = Package::new(PKG_NAME);
        let client = http_client::new(http_client::UreqHttpClient, TIMEOUT);
        let data_path = format!("{}/not_found.html", FIXTURES_PATH);
        let _mock = mock_pypi(&pkg, 404, &data_path);
        let current_version = Version::parse("0.1.0").expect("parse version");

        let result = Npm::get_latest_version(client, &pkg, &current_version);
        assert!(result.is_err());
    }

    #[test]
    fn success_test() {
        let pkg = Package::new(PKG_NAME);
        let client = http_client::new(http_client::UreqHttpClient, TIMEOUT);
        let data_path = format!("{}/latest.json", FIXTURES_PATH);
        let (_mock, _data) = mock_npm(&pkg, 200, &data_path);
        let current_version = Version::parse("1.6.2").expect("parse version");

        let latest_version = "1.6.3".to_string();
        let result = Npm::get_latest_version(client, &pkg, &current_version);

        dbg!(&result);

        assert!(result.is_ok());
        assert_eq!(result.expect("get result"), Some(latest_version));
    }
}