use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use warp::{path::FullPath, Filter};

pub fn cache<F>(func: F) -> impl Filter<Extract = (String,)>
where
    F: Send + Sync + Clone + Fn() -> String,
{
    let store = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    warp::path::full().map(move |path: FullPath| {
        let mut cache_store = store.lock().unwrap();

        let path_string = path.as_str().to_string();

        if cache_store.contains_key(&path_string) {
            if let Some(val) = cache_store.get(&path_string) {
                return val.clone();
            }
        }

        let resp = func();

        cache_store.insert(path_string, resp.clone());

        resp
    })
}

#[cfg(test)]
mod tests {
    use super::cache;

    #[test]
    fn it_works() {
        let filter = cache(|| "test".to_string());

        let value = warp::test::request().path("/").filter(&filter).unwrap();
        assert_eq!(value, "test");

        let value = warp::test::request().path("/").filter(&filter).unwrap();
        assert_eq!(value, "test");
    }
}
