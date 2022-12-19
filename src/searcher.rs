use crate::configstore::{ConfigStore, SshItem};

pub struct Searcher {
    pub search_string: String,
}
impl Searcher {
    pub fn new(search_str: &str) -> Searcher {
        Searcher {
            search_string: search_str.to_owned(),
        }
    }

    pub fn get_filtered_hosts<'a>(&self, store: &'a ConfigStore) -> Vec<&'a SshItem> {
        if self.search_string.is_empty() {
            return store.get_all_hosts();
        }

        store
            .get_all_hosts()
            .into_iter()
            .filter(|item| item.host.contains(&self.search_string))
            .collect()
    }
    pub fn add_char(&mut self, c: char) {
        self.search_string.push(c);
    }

    pub fn del_char(&mut self) {
        self.search_string.pop();
    }
}
