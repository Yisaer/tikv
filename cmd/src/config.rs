// Copyright 2017 TiKV Project Authors. Licensed under Apache-2.0.

use std::error::Error;
use std::fs;
use std::path::Path;

use backup::Config as BackupConfig;
use tikv::config::TiKvConfig;

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    #[serde(flatten)]
    pub tikv_cfg: TiKvConfig,
    #[serde(skip_serializing_if = "BackupConfig::is_default")]
    pub backup: BackupConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            tikv_cfg: TiKvConfig::default(),
            backup: BackupConfig::default(),
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        (|| -> Result<Self, Box<dyn Error>> {
            let s = fs::read_to_string(&path)?;
            Ok(toml::from_str(&s)?)
        })()
        .unwrap_or_else(|e| {
            panic!(
                "invalid auto generated configuration file {}, err {}",
                path.as_ref().display(),
                e
            );
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml;

    #[test]
    fn test_flatten() {
        let tikv_str = toml::to_string(&TiKvConfig::default()).unwrap();
        let cfg_str = toml::to_string(&Config::default()).unwrap();
        assert_eq!(tikv_str, cfg_str);
    }

    #[test]
    fn test_backup_config() {
        let mut backup_cfg = BackupConfig::default();
        backup_cfg.concurrency = 7;
        let mut cfg = Config::default();
        cfg.backup = backup_cfg;

        let cfg_str = toml::to_string_pretty(&cfg).unwrap();
        let cfg1: Config = toml::from_str(&cfg_str).unwrap();
        assert_eq!(cfg.backup, cfg1.backup, "{}", cfg_str);
    }
}
