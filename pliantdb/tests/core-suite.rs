use std::time::Duration;

use once_cell::sync::Lazy;
use pliantdb::{
    client::{Client, RemoteDatabase},
    core::{
        connection::ServerConnection,
        fabruic::Certificate,
        test_util::{BasicSchema, HarnessTest, TestDirectory},
    },
    server::test_util::{initialize_basic_server, BASIC_SERVER_NAME},
};
use tokio::sync::Mutex;
use url::Url;

async fn initialize_shared_server() -> Certificate {
    static CERTIFICATE: Lazy<Mutex<Option<Certificate>>> = Lazy::new(|| Mutex::new(None));
    let mut certificate = CERTIFICATE.lock().await;
    if certificate.is_none() {
        let (sender, receiver) = flume::bounded(1);
        std::thread::spawn(|| run_shared_server(sender));

        *certificate = Some(receiver.recv_async().await.unwrap());
        // Give the server time to start listening
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }

    certificate.clone().unwrap()
}

fn run_shared_server(certificate_sender: flume::Sender<Certificate>) -> anyhow::Result<()> {
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async move {
        let directory = TestDirectory::new("shared-server");
        let server = initialize_basic_server(directory.as_ref()).await.unwrap();
        certificate_sender
            .send(server.certificate().await.unwrap())
            .unwrap();

        #[cfg(feature = "websockets")]
        {
            let task_server = server.clone();
            tokio::spawn(async move {
                task_server
                    .listen_for_websockets_on("localhost:6001")
                    .await
                    .unwrap();
            });
        }

        server.listen_on(6000).await.unwrap();
    });

    Ok(())
}

#[cfg(feature = "websockets")]
mod websockets {
    use super::*;

    struct WebsocketTestHarness {
        client: Client,
        db: RemoteDatabase<BasicSchema>,
    }

    impl WebsocketTestHarness {
        pub async fn new(test: HarnessTest) -> anyhow::Result<Self> {
            initialize_shared_server().await;
            let url = Url::parse("ws://localhost:6001")?;
            let client = Client::new(url, None).await?;

            let dbname = format!("websockets-{}", test);
            client.create_database::<BasicSchema>(&dbname).await?;
            let db = client.database::<BasicSchema>(&dbname).await?;

            Ok(Self { client, db })
        }

        pub const fn server_name() -> &'static str {
            "websocket"
        }

        pub fn server(&self) -> &'_ Client {
            &self.client
        }

        pub async fn connect<'a, 'b>(&'a self) -> anyhow::Result<RemoteDatabase<BasicSchema>> {
            Ok(self.db.clone())
        }

        pub async fn shutdown(&self) -> anyhow::Result<()> {
            Ok(())
        }
    }

    pliantdb_core::define_connection_test_suite!(WebsocketTestHarness);

    #[cfg(feature = "pubsub")]
    pliantdb_core::define_pubsub_test_suite!(WebsocketTestHarness);
    #[cfg(feature = "keyvalue")]
    pliantdb_core::define_kv_test_suite!(WebsocketTestHarness);
}

mod pliant {
    use super::*;
    struct PliantTestHarness {
        client: Client,
        db: RemoteDatabase<BasicSchema>,
    }

    impl PliantTestHarness {
        pub async fn new(test: HarnessTest) -> anyhow::Result<Self> {
            let certificate = initialize_shared_server().await;

            let url = Url::parse(&format!(
                "pliantdb://localhost:6000?server={}",
                BASIC_SERVER_NAME
            ))?;
            let client = Client::new(url, Some(certificate)).await?;

            let dbname = format!("pliant-{}", test);
            client.create_database::<BasicSchema>(&dbname).await?;
            let db = client.database::<BasicSchema>(&dbname).await?;

            Ok(Self { client, db })
        }

        pub fn server_name() -> &'static str {
            "pliant"
        }

        pub fn server(&self) -> &'_ Client {
            &self.client
        }

        pub async fn connect<'a, 'b>(&'a self) -> anyhow::Result<RemoteDatabase<BasicSchema>> {
            Ok(self.db.clone())
        }

        pub async fn shutdown(&self) -> anyhow::Result<()> {
            Ok(())
        }
    }

    pliantdb_core::define_connection_test_suite!(PliantTestHarness);
    #[cfg(feature = "pubsub")]
    pliantdb_core::define_pubsub_test_suite!(PliantTestHarness);

    #[cfg(feature = "keyvalue")]
    pliantdb_core::define_kv_test_suite!(PliantTestHarness);
}
