use super::*;

#[tokio::test]
async fn generate() {
    use httptest::{mappers::*, responders::*, Expectation, Server};

    let generated = types::responses::Generated {
        name: "foo".into(),
        data: "hello world".into(),
    };

    let server = Server::run();
    let url = format!("http://{}", server.addr());

    for (k, v) in vec![
        (
            vec![
                KV::new("context", "testing this"),
                KV::new("min", "5"),
                KV::new("max", "30"),
            ],
            Client::new(&url)
                .generate("foo")
                .min(5_usize)
                .max(30_usize)
                .context("testing this"),
        ),
        (
            vec![KV::new("context", "testing this")],
            Client::new(&url).generate("foo").context("testing this"),
        ),
        (
            vec![KV::new("min", "5"), KV::new("max", "30")],
            Client::new(&url).generate("foo").min(5_usize).max(30_usize),
        ),
    ] {
        server.expect(
            Expectation::matching(all_of![
                request::method("GET"), //
                request::path("/generate/foo"),
                request::query(url_decoded(eq(k)))
            ])
            .respond_with(json_encoded(&generated)),
        );

        let resp = v.send().await.unwrap();
        assert_eq!(resp, generated);
    }
}

#[tokio::test]
async fn train() {
    use httptest::{mappers::*, responders::*, Expectation, Server};

    let data = "some test message".to_string();
    let trained = types::responses::Trained {
        data: data.clone(),
        time: std::time::Duration::from_millis(1),
    };

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"), //
            request::path("/train/foo"),
            request::body(
                serde_json::to_string(&types::input::TrainData { data: data.clone() }).unwrap()
            )
        ])
        .respond_with(json_encoded(&trained)),
    );

    let addr = server.addr();
    let resp = Client::new(format!("http://{}", addr))
        .train("foo", data)
        .send()
        .await
        .unwrap();

    assert_eq!(resp, trained);
}

#[tokio::test]
async fn new_brain() {
    use httptest::{mappers::*, responders::*, Expectation, Server};

    let created = types::responses::Created {
        name: "foo".into(),
        brain_file: "foo_brain".into(),
    };

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("POST"), //
            request::path("/new/foo"),
            request::body(
                serde_json::to_string(&types::input::NewBrain {
                    brain_file: "foo_brain".into(),
                    depth: 5,
                })
                .unwrap()
            )
        ])
        .respond_with(json_encoded(&created)),
    );

    let addr = server.addr();
    let resp = Client::new(format!("http://{}", addr))
        .new_brain("foo", "foo_brain")
        .send()
        .await
        .unwrap();

    assert_eq!(resp, created);
}

#[tokio::test]
async fn save() {
    use httptest::{mappers::*, responders::*, Expectation, Server};

    let save_response = types::responses::Saved {
        name: "foo".into(),
        time: std::time::Duration::from_millis(42),
    };

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("PUT"), //
            request::path("/save/foo"),
        ])
        .respond_with(json_encoded(&save_response)),
    );

    let addr = server.addr();
    let resp = Client::new(format!("http://{}", addr))
        .save("foo")
        .send()
        .await
        .unwrap();

    assert_eq!(resp, save_response);
}

#[tokio::test]
async fn save_error() {
    use httptest::{mappers::*, responders::*, Expectation, Server};

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("PUT"), //
            request::path("/save/foo"),
        ])
        .respond_with(
            status_code(400).body(
                serde_json::to_string(&types::Error::CannotSave {
                    file: "foo".into(),
                    reason: "".into(),
                })
                .unwrap(),
            ),
        ),
    );

    let addr = server.addr();
    let resp = Client::new(format!("http://{}", addr))
        .save("foo")
        .send()
        .await
        .unwrap_err();

    matches::assert_matches!(
       resp, Error::Server {
           err: types::Error::CannotSave { ref file, ..}
       } if file == "foo"
    )
}

#[tokio::test]
async fn list_ok() {
    use httptest::{mappers::*, responders::*, Expectation, Server};

    let list_response = types::responses::List {
        brains: {
            let mut map = hashbrown::HashMap::new();
            map.insert(
                "foo".into(),
                types::responses::ListItem {
                    name: "foo".into(),
                    brain_file: "foo_brain.db".into(),
                    read_only: true,
                },
            );
            map.insert(
                "bar".into(),
                types::responses::ListItem {
                    name: "bar_brain".into(),
                    brain_file: "bar.db".into(),
                    read_only: false,
                },
            );
            map
        },
        config_path: "local_config.toml".into(),
    };

    let server = Server::run();
    server.expect(
        Expectation::matching(all_of![
            request::method("GET"), //
            request::path("/list")
        ])
        .respond_with(json_encoded(&list_response)),
    );

    let addr = server.addr();
    let resp = Client::new(format!("http://{}", addr))
        .list()
        .send()
        .await
        .unwrap();

    assert_eq!(resp, list_response);
}
