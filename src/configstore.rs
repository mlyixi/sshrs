use anyhow::{Ok, Result};
use ssh_cfg::{SshConfig, SshConfigParser, SshOptionKey};
use std::path::Path;
use whoami::username;

pub struct SshItem {
    pub host: String,
    pub user: String,
    pub target: String,
    pub port: String,
    pub jump: String,
}
pub struct ConfigStore {
    pub config: SshConfig,
    pub items: Vec<SshItem>,
}
impl ConfigStore {
    pub fn new(config_path: &Path) -> Result<ConfigStore> {
        let config = SshConfigParser::parse(config_path)?;
        let items = config
            .iter()
            .map(|(host, config)| {
                let host = host.to_owned();
                let user = config
                    .get(&SshOptionKey::User)
                    .unwrap_or(&username())
                    .to_owned();
                let target = config
                    .get(&SshOptionKey::Hostname)
                    .unwrap_or(&host)
                    .to_owned();
                let port = config
                    .get(&SshOptionKey::Port)
                    .unwrap_or(&22.to_string())
                    .to_owned();
                let jump = config
                    .get(&SshOptionKey::ProxyJump)
                    .unwrap_or(&String::new())
                    .to_owned();
                SshItem {
                    host,
                    user,
                    target,
                    port,
                    jump,
                }
            })
            .collect();
        let cs = ConfigStore { config, items };
        Ok(cs)
    }
    pub fn get_all_hosts(&self) -> Vec<&SshItem> {
        self.items.iter().map(|item| item).collect()
    }
}
