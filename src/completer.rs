use crate::configstore::ConfigStore;

pub struct Completer {
    pub jumpers_string: String,
    pub display_string: String,
    pub match_jumpers: Vec<String>,
    pub search_string: String,
    pub has_popup_input: bool,
    pub idx: usize,
}
impl Completer {
    pub fn new() -> Completer {
        Completer {
            jumpers_string: String::new(),
            display_string: String::new(),
            match_jumpers: Vec::new(),
            search_string: String::new(),
            has_popup_input: true,
            idx: 0,
        }
    }
    pub fn get_filtered_jumpers<'a>(&self, store: &'a ConfigStore, s: &str) -> Vec<String> {
        store
            .get_all_hosts()
            .iter()
            .filter(|h| h.host.starts_with(s))
            .map(|h| h.host.to_string())
            .collect()
    }
    pub fn complete<'a>(&mut self, store: &'a ConfigStore) {
        match self.has_popup_input {
            true => {
                let (ready, last) = self
                    .display_string
                    .rsplit_once(',')
                    .unwrap_or(("", &self.display_string));
                self.idx = 0;
                self.jumpers_string = ready.to_owned();
                self.search_string = last.to_owned();
                self.match_jumpers = self.get_filtered_jumpers(store, last);
                self.display_string = match self.jumpers_string.is_empty() {
                    true => format!("{},", self.match_jumpers[self.idx]),
                    false => format!("{},{},", self.jumpers_string, self.match_jumpers[self.idx]),
                };
                self.idx += 1;
            }
            false => {
                let host = &self.match_jumpers[self.idx % self.match_jumpers.len()];
                self.display_string = match self.jumpers_string.is_empty() {
                    true => format!("{},", host),
                    false => format!("{},{},", self.jumpers_string, host),
                };
                self.idx += 1;
            }
        }
        self.has_popup_input = false;
    }
    pub fn add_char(&mut self, c: char) {
        self.has_popup_input = true;
        self.display_string.push(c);
    }
    pub fn del_char(&mut self) {
        self.has_popup_input = true;
        self.display_string.pop();
    }
    pub fn clear(&mut self) {
        self.has_popup_input = true;
        self.display_string.clear();
        self.jumpers_string.clear();
    }
}
