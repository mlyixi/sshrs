use crate::{
    completer::Completer,
    configstore::{ConfigStore, SshItem},
    searcher::Searcher,
    ui,
};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode};
use std::path::Path;
use tui::{backend::Backend, widgets::TableState, Terminal};

pub struct App {
    pub state: TableState,
    pub configstore: ConfigStore,
    pub searcher: Searcher,
    pub completer: Completer,
    pub should_spawn_ssh: bool,
    pub should_show_popup: bool,
}

impl App {
    pub fn new(config_path: &Path, search_str: &str) -> Result<App> {
        match ConfigStore::new(config_path) {
            Ok(configstore) => Ok(App {
                state: TableState::default(),
                configstore,
                searcher: Searcher::new(search_str),
                completer: Completer::new(),
                should_spawn_ssh: false,
                should_show_popup: false,
            }),
            Err(e) => return Err(e),
        }
    }
    pub fn change_selected_item(&mut self, rot_right: bool) {
        let items_len = self.get_filtered_items().len();

        if items_len == 0 {
            return;
        }
        let i = match self.state.selected() {
            Some(i) => {
                if rot_right {
                    (i + 1) % items_len
                } else {
                    (i + items_len - 1) % items_len
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
    pub fn add_char(&mut self, c: char) {
        match self.should_show_popup {
            true => self.completer.add_char(c),
            false => self.searcher.add_char(c),
        }
    }

    pub fn del_char(&mut self) {
        match self.should_show_popup {
            true => self.completer.del_char(),
            false => self.searcher.del_char(),
        }
    }

    pub fn get_filtered_items(&self) -> Vec<&SshItem> {
        self.searcher.get_filtered_hosts(&self.configstore)
    }
    pub fn get_selected_item(&self) -> Option<&SshItem> {
        if let Some(selected) = self.state.selected() {
            let items_len = self.get_filtered_items().len();
            if selected < items_len {
                Some(self.get_filtered_items()[selected])
            } else {
                None
            }
        } else {
            None
        }
    }
}
pub fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => match app.should_show_popup {
                    true => {
                        app.should_show_popup = false;
                        app.completer.clear();
                    }
                    false => return Ok(()),
                },
                KeyCode::Char(c) => {
                    app.add_char(c);
                }
                KeyCode::Backspace => {
                    app.del_char();
                }
                KeyCode::Down => app.change_selected_item(true),
                KeyCode::Up => app.change_selected_item(false),
                KeyCode::Enter => {
                    if app.get_selected_item().is_some() {
                        match app.should_show_popup {
                            false => app.should_show_popup = true,
                            true => {
                                if !app.completer.has_popup_input
                                    || app.completer.display_string.ends_with(",")
                                    || app.completer.display_string.is_empty()
                                {
                                    app.should_spawn_ssh = true;
                                }
                            }
                        }
                    }
                }
                KeyCode::Tab => app.completer.complete(&app.configstore),
                _ => {}
            }
        }
        if app.should_spawn_ssh {
            break;
        }
    }

    Ok(())
}
