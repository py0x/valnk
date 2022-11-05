use aws_config;
use aws_sdk_dynamodb::Client as AwsDdbClient;
use std::ops::Deref;

#[derive(Debug)]
enum DdbClient<'c> {
    SharedClient(&'c AwsDdbClient),
    OwnedClient(AwsDdbClient),
}

impl<'c> Deref for DdbClient<'c> {
    type Target = AwsDdbClient;

    fn deref(&self) -> &Self::Target {
        return match self {
            Self::SharedClient(cli) => *cli,
            Self::OwnedClient(cli) => cli
        };
    }
}

#[derive(Debug)]
pub struct Client<'c> {
    ddb_cli: DdbClient<'c>,
}

impl<'c> Client<'c> {
    /// Creates a new client with the default configuration from the environment.
    ///
    /// # Example:
    ///
    /// ```
    /// use tokio;
    /// use aws_config;
    /// use valnk::data::api::client::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let client = Client::new();
    /// }
    /// ```
    pub async fn new() -> Client<'c> {
        let shared_config = aws_config::load_from_env().await;
        let aws_cli = AwsDdbClient::new(&shared_config);

        return Client {
            ddb_cli: DdbClient::OwnedClient(aws_cli),
        };
    }

    /// Creates a new client from a shared config.
    ///
    /// # Example:
    ///
    /// ```
    /// use tokio;
    /// use aws_config;
    /// use valnk::data::api::client::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shared_config = aws_config::load_from_env().await;
    ///     let client = Client::from_aws_conf(&shared_config);
    /// }
    /// ```
    pub fn from_aws_conf(aws_config: &aws_config::SdkConfig) -> Client<'c> {
        let aws_cli = AwsDdbClient::new(&aws_config);

        return Client {
            ddb_cli: DdbClient::OwnedClient(aws_cli),
        };
    }

    /// Creates a new client from a shared aws client.
    ///
    /// # Example:
    ///
    /// ```
    /// use tokio;
    /// use aws_config;
    /// use aws_sdk_dynamodb;
    /// use valnk::data::api::client::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let shared_config = aws_config::load_from_env().await;
    ///     let aws_cli = aws_sdk_dynamodb::Client::new(&shared_config);
    ///     let client = Client::from_aws_cli(&aws_cli);
    /// }
    /// ```
    pub fn from_aws_cli(aws_ddb_cli: &'c AwsDdbClient) -> Client<'c> {
        return Client {
            ddb_cli: DdbClient::SharedClient(aws_ddb_cli),
        };
    }
}

