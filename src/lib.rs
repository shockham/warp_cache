use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use warp::{filters::BoxedFilter, path::FullPath, Filter};

pub fn cache(store: Arc<Mutex<HashMap<String, String>>>) -> BoxedFilter<(String,)> {
    warp::path::full()
        .map(move |path: FullPath| {
            let mut cache_store = store.lock().unwrap();

	    let path_string = path.as_str().to_string();

            if cache_store.contains_key(&path_string) {
                if let Some(val) = cache_store.get(&path_string) {
                    return val.clone();
		}
            } else {
                cache_store.insert(path_string, "from cache".to_string());
            }

            String::from("")
        })
        .boxed()
}

#[cfg(test)]
mod tests {
    use super::cache;
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
    };

    #[test]
    fn it_works() {
        let store = Arc::new(Mutex::new(HashMap::new()));
        let filter = cache(store);

        let value = warp::test::request().path("/").filter(&filter).unwrap();
        assert_eq!(value, "");

        let value = warp::test::request().path("/").filter(&filter).unwrap();
        assert_eq!(value, "from cache");
    }
}
