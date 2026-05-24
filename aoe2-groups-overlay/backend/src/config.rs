use std::{collections::HashMap, path::Path};

use anyhow::{anyhow, Context, Result};
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RawConfig {
    #[serde(default)]
    server: RawServer,
    #[serde(default)]
    tournaments: Vec<Tournament>,
}

#[derive(Debug, Deserialize, Default)]
struct RawServer {
    bind_addr: Option<String>,
    port: Option<u16>,
    allowed_origins: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bracket {
    pub name: String,
    pub group_ranges: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Tournament {
    pub slug: String,
    pub sheet_id: String,
    #[serde(default)]
    pub brackets: Vec<Bracket>,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub bind_addr: String,
    pub port: u16,
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub tournaments: HashMap<String, Tournament>,
}

impl Config {
    pub fn load(config_path: &Path, tournaments_path: &Path) -> Result<Self> {
        // tournaments.toml is baked into the image and required; config.toml is
        // optional (used for local dev or future runtime overrides).
        let mut fig = Figment::new().merge(Toml::file(tournaments_path));
        if config_path.exists() {
            fig = fig.merge(Toml::file(config_path));
        }
        let raw: RawConfig = fig
            .merge(Env::prefixed("AOE2_PROXY_").split("__"))
            .extract()
            .with_context(|| {
                format!(
                    "loading config from {} (+ optional {})",
                    tournaments_path.display(),
                    config_path.display(),
                )
            })?;
        validate(raw)
    }
}

fn validate(raw: RawConfig) -> Result<Config> {
    let mut tournaments = HashMap::with_capacity(raw.tournaments.len());
    for t in raw.tournaments {
        if t.slug.is_empty() {
            return Err(anyhow!("tournament has empty slug"));
        }
        for b in &t.brackets {
            if b.name.is_empty() {
                return Err(anyhow!(
                    "tournament '{}' has bracket with empty name",
                    t.slug
                ));
            }
        }
        if tournaments.insert(t.slug.clone(), t.clone()).is_some() {
            return Err(anyhow!("duplicate tournament slug '{}'", t.slug));
        }
    }

    let bind_addr = raw
        .server
        .bind_addr
        .unwrap_or_else(|| "0.0.0.0".to_string());
    // Cloud Run injects PORT into the container env.
    let port = raw
        .server
        .port
        .or_else(|| std::env::var("PORT").ok().and_then(|s| s.parse().ok()))
        .unwrap_or(8080);
    let allowed_origins = raw.server.allowed_origins.unwrap_or_else(|| {
        std::env::var("ALLOWED_ORIGINS")
            .ok()
            .map(|s| {
                s.split(',')
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty())
                    .collect()
            })
            .unwrap_or_default()
    });

    Ok(Config {
        server: ServerConfig {
            bind_addr,
            port,
            allowed_origins,
        },
        tournaments,
    })
}

#[cfg(test)]
#[allow(clippy::result_large_err)] // figment::Error is large; Jail's closure can't avoid it.
mod tests {
    use super::*;
    use figment::Jail;

    #[test]
    fn loads_minimal_tournaments_toml() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "ttlc2"
sheet_id = "sheet-123"

[[tournaments.brackets]]
name = "Obsidian"
group_ranges = ["G4:P9", "V4:AE9"]

[[tournaments.brackets]]
name = "Titanium"
group_ranges = ["G4:P9", "V4:AE9", "G23:P28"]
"#,
            )?;
            let cfg = Config::load(
                Path::new("does-not-exist-config.toml"),
                Path::new("tournaments.toml"),
            )
            .map_err(|e| e.to_string())?;
            assert_eq!(cfg.tournaments.len(), 1);
            let t = cfg.tournaments.get("ttlc2").unwrap();
            assert_eq!(t.sheet_id, "sheet-123");
            assert_eq!(t.brackets.len(), 2);
            assert_eq!(t.brackets[0].name, "Obsidian");
            assert_eq!(t.brackets[1].group_ranges.len(), 3);
            Ok(())
        });
    }

    #[test]
    fn rejects_duplicate_slugs() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "x"
sheet_id = "a"

[[tournaments]]
slug = "x"
sheet_id = "b"
"#,
            )?;
            let err = Config::load(Path::new("nope.toml"), Path::new("tournaments.toml"))
                .unwrap_err()
                .to_string();
            assert!(err.contains("duplicate"), "{err}");
            Ok(())
        });
    }

    #[test]
    fn defaults_when_no_server_section() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "a"
sheet_id = "s"
"#,
            )?;
            let cfg = Config::load(Path::new("nope.toml"), Path::new("tournaments.toml"))
                .map_err(|e| e.to_string())?;
            assert_eq!(cfg.server.bind_addr, "0.0.0.0");
            assert_eq!(cfg.server.port, 8080);
            assert!(cfg.server.allowed_origins.is_empty());
            Ok(())
        });
    }
}
