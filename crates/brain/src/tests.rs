use crate::{
    models::Error,
    server::{Brain, Topics},
    *,
};
use hashbrown::HashMap;
use markov::Markov;
use std::path::PathBuf;
use std::sync::Arc;
use tempdir::TempDir;
use tokio::sync::Mutex;
use warp::http::StatusCode;
use warp::test::request;

const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet,\
                           consectetur adipiscing elit. \
                           Donec ornare mi vitae fermentum aliquet. \
                           Vivamus placerat lacinia ipsum, at suscipit.";

fn make_input() -> models::input::TrainData {
    models::input::TrainData {
        data: LOREM_IPSUM.into(),
    }
}

fn make_brain<'a>(
    dir: impl Into<Option<&'a TempDir>>,
    name: impl ToString,
    file: impl Into<PathBuf>,
    read_only: bool,
    state: impl Into<Option<&'static str>>,
) -> Brain {
    let mut brain_file = file.into();

    if let Some(dir) = dir.into() {
        let file_name = dir.path().join(&brain_file);
        std::fs::File::create(&file_name).unwrap();
        brain_file = file_name;
    }

    let mut markov = Markov::new(3, name.to_string());
    if let Some(data) = state.into() {
        markov.train_text(data)
    }

    Brain {
        config: config::BrainConfig {
            name: name.to_string(),
            brain_file,
            read_only,
        },
        markov: Mutex::new(markov),
    }
}

fn make_db(test_dir: &TempDir, state: impl Into<Option<&'static str>> + Copy) -> Arc<Topics> {
    let test1 = make_brain(test_dir, "test1", "test1.db", false, state);
    let test2 = make_brain(test_dir, "test2", "test2.db", true, state);
    let test_no_file = make_brain(test_dir, "test_no_file", "test_no_file.db", false, state);

    let mut map = HashMap::new();
    map.insert("test1".into(), Arc::new(test1));
    map.insert("test2".into(), Arc::new(test2));
    map.insert("test_no_file".into(), Arc::new(test_no_file));

    let brain_config_path = test_dir.path().join("brain.toml");
    std::fs::File::create(&brain_config_path).unwrap();

    Arc::new(Topics::new(brain_config_path, map))
}

fn body_as_json<'de, T>(resp: &'de warp::http::Response<bytes::Bytes>) -> T
where
    T: serde::de::Deserialize<'de>,
{
    use bytes::Buf as _;
    assert_eq!(
        resp.headers().get("content-type").unwrap(),
        "application/json"
    );
    let data = resp.body().bytes();
    serde_json::from_slice(data).unwrap()
}

#[tokio::test]
async fn generate_no_state() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, None));
    let resp = request()
        .method("GET")
        .path("/generate/test1")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn generate_context_no_state() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, None));
    let resp = request()
        .method("GET")
        .path("/generate/test1?context=foo%20bar")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn generate_readonly_no_state() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, None));
    let resp = request()
        .method("GET")
        .path("/generate/test2")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn generate() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, LOREM_IPSUM));
    let resp = request()
        .method("GET")
        .path("/generate/test1")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn generate_context() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, LOREM_IPSUM));
    let resp = request()
        .method("GET")
        .path("/generate/test1?context=foo%20bar")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn generate_readonly() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, LOREM_IPSUM));
    let resp = request()
        .method("GET")
        .path("/generate/test2")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
}

#[tokio::test]
async fn generate_unknown() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::generate(make_db(&dir, LOREM_IPSUM));
    let resp = request()
        .method("GET")
        .path("/generate/test3")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn train_readonly() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::train(make_db(&dir, None));
    // test2 is read-only
    let resp = request()
        .method("POST")
        .path("/train/test2")
        .json(&make_input())
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let err: Error = body_as_json(&resp);
    matches::assert_matches!(err, Error::ReadOnly);
}

#[tokio::test]
async fn train_success() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::train(make_db(&dir, None));
    let resp = request()
        .method("POST")
        .path("/train/test1")
        .json(&make_input())
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);
    // just to assert
    let _data: models::responses::Trained = body_as_json(&resp);
}

#[tokio::test]
async fn train_not_allowed() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::train(make_db(&dir, None));
    let resp = request()
        .method("GET")
        .path("/train/test2?garbage=here")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn train_no_body() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::train(make_db(&dir, None));
    let resp = request()
        .method("POST")
        .path("/train/test2")
        .reply(&api)
        .await;
    // TODO use or_else + recover to turn this into a 400
    assert_eq!(resp.status(), 411);
}

#[tokio::test]
async fn save_cannot_rotate() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::save(make_db(&dir, None));

    {
        let temp = dir.path().join("test_no_file.bak");
        tokio::fs::File::create(&temp).await.unwrap();
        let mut perms = tokio::fs::metadata(&temp).await.unwrap().permissions();
        perms.set_readonly(true);
        tokio::fs::set_permissions(&temp, perms).await.unwrap();
    }

    let resp = request()
        .method("PUT")
        .path("/save/test_no_file")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let err: Error = body_as_json(&resp);
    matches::assert_matches!(err, Error::CannotRotate{..});
    if let Error::CannotRotate { file, .. } = err {
        assert_eq!(file, dir.path().join("test_no_file.db").to_str().unwrap())
    }
}

#[tokio::test]
async fn save_rotate() {
    let name = {
        let dir = TempDir::new("brain_tests").unwrap();
        let api = routes::save(make_db(&dir, None));
        let resp = request()
            .method("PUT")
            .path("/save/test1")
            .reply(&api)
            .await;
        assert_eq!(resp.status(), StatusCode::OK);
        let models::responses::Saved { name, .. } = body_as_json(&resp);

        // assert that they are there
        tokio::fs::metadata(&name).await.unwrap();

        let mut right = std::path::PathBuf::from(&name);
        right.set_extension("bak");
        tokio::fs::metadata(right).await.unwrap();

        name
    };

    // assert that they aren't there
    tokio::fs::metadata(&name).await.unwrap_err();

    let mut right = std::path::PathBuf::from(name);
    right.set_extension("bak");
    tokio::fs::metadata(right).await.unwrap_err();
}

// this is incase the 'input' is read-only
#[tokio::test]
async fn save_cannnot_save() {
    let dir = TempDir::new("brain_tests").unwrap();

    let api = routes::save(make_db(&dir, None));
    let resp = request()
        .method("PUT")
        .path("/save/test1")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);

    for test in &["test1.db", "test1.bak"] {
        let test1 = dir.path().join(test);
        let mut perms = tokio::fs::metadata(&test1).await.unwrap().permissions();
        perms.set_readonly(true);
        tokio::fs::set_permissions(&test1, perms).await.unwrap();
    }

    let resp = request()
        .method("PUT")
        .path("/save/test1")
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let err: Error = body_as_json(&resp);
    matches::assert_matches!(err, Error::CannotRotate{..});
    if let Error::CannotSave { file, .. } = err {
        assert_eq!(file, "test1.db")
    }
}

#[tokio::test]
async fn save_invalid_method() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::save(make_db(&dir, None));

    let resp = request()
        .method("POST")
        .path("/save/test2")
        .json(&make_input())
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

    let resp = request()
        .method("GET")
        .path("/save/test3")
        .reply(&api)
        .await;
    // TODO
    assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);
}

#[tokio::test]
async fn new_success() {
    let dir = TempDir::new("brain_tests").unwrap();
    let db = make_db(&dir, None);
    let api = routes::new(Arc::clone(&db));

    let brain_file = dir
        .path()
        .join("test3_different")
        .to_string_lossy()
        .to_string();

    let resp = request()
        .method("POST")
        .path("/new/test3")
        .json(&models::input::NewBrain {
            brain_file: brain_file.clone(),
            depth: 5,
        })
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let lock = db.brains.lock().await;
    let brain = lock.get("test3").unwrap();
    assert_eq!(brain.config.name, "test3");
    assert_eq!(brain.config.brain_file, PathBuf::from(brain_file));
    assert_eq!(brain.config.read_only, false);
}

#[tokio::test]
async fn new_append() {
    let dir = TempDir::new("brain_tests").unwrap();
    let db = make_db(&dir, None);
    let api = routes::new(Arc::clone(&db));

    let brain_file = dir
        .path()
        .join("test3_different")
        .to_string_lossy()
        .to_string();

    let resp = request()
        .method("POST")
        .path("/new/test3")
        .json(&models::input::NewBrain {
            brain_file: brain_file.clone(),
            depth: 5,
        })
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let data = tokio::fs::read_to_string(&db.config_path).await.unwrap();
    let config: config::Config = toml::from_str(&data).unwrap();
    assert_eq!(config.brains.len(), 1);

    let brain = config.brains.get("test3").unwrap();
    assert_eq!(brain.name, ""); // a name isn't set here
    assert_eq!(brain.brain_file, PathBuf::from(brain_file));
    assert_eq!(brain.read_only, false);

    let brain_file = dir
        .path()
        .join("another_file")
        .to_string_lossy()
        .to_string();

    let resp = request()
        .method("POST")
        .path("/new/test4")
        .json(&models::input::NewBrain {
            brain_file: brain_file.clone(),
            depth: 5,
        })
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::OK);

    let data = tokio::fs::read_to_string(&db.config_path).await.unwrap();
    let config: config::Config = toml::from_str(&data).unwrap();
    assert_eq!(config.brains.len(), 2);

    let brain = config.brains.get("test4").unwrap();
    assert_eq!(brain.name, ""); // a name isn't set here
    assert_eq!(brain.brain_file, PathBuf::from(brain_file));
    assert_eq!(brain.read_only, false);
}

#[tokio::test]
async fn new_conflict() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::new(make_db(&dir, None));
    let resp = request()
        .method("POST")
        .path("/new/test1")
        .json(&make_input())
        .reply(&api)
        .await;
    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);

    let err: Error = body_as_json(&resp);
    matches::assert_matches!(err, Error::AlreadyExists{..});
    if let Error::AlreadyExists { name } = err {
        assert_eq!(name, "test1")
    }
}

#[tokio::test]
async fn list_some() {
    let dir = TempDir::new("brain_tests").unwrap();
    let api = routes::list(make_db(&dir, None));
    let resp = request().method("GET").path("/list").reply(&api).await;
    let list = body_as_json::<models::responses::List>(&resp);
    // test1 and test2 and test_no_file
    assert_eq!(list.brains.len(), 3);
}

#[tokio::test]
async fn list_none() {
    let dir = TempDir::new("brain_tests").unwrap();
    let brain = make_db(&dir, None);
    brain.brains.lock().await.clear();
    let api = routes::list(brain);

    let resp = request().method("GET").path("/list").reply(&api).await;
    let list = body_as_json::<models::responses::List>(&resp);
    assert_eq!(list.brains.len(), 0);
}
