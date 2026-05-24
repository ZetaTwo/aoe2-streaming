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
    tournaments: Vec<RawTournament>,
}

#[derive(Debug, Deserialize, Default)]
struct RawServer {
    bind_addr: Option<String>,
    port: Option<u16>,
    allowed_origins: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct RawTournament {
    slug: String,
    #[serde(default)]
    brackets: Vec<Bracket>,
}

/// Wire format of the sheet-ids secret file:
/// `[sheet_ids]\n<slug> = "<sheet_id>"\n…`
#[derive(Debug, Deserialize, Default)]
struct RawSheetIds {
    #[serde(default)]
    sheet_ids: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Bracket {
    pub name: String,
    pub group_ranges: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Tournament {
    pub slug: String,
    pub sheet_id: String,
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
    pub fn load(
        config_path: &Path,
        tournaments_path: &Path,
        sheet_ids_path: &Path,
    ) -> Result<Self> {
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

        // sheet-ids.toml lives outside the image (Secret Manager mount in prod,
        // gitignored local file in dev). Missing / empty file is allowed at this
        // stage — `validate` enforces presence per-slug for tournaments that
        // have brackets configured.
        let sheet_ids = load_sheet_ids(sheet_ids_path)?;

        validate(raw, sheet_ids)
    }
}

fn load_sheet_ids(path: &Path) -> Result<HashMap<String, String>> {
    if !path.exists() {
        tracing::warn!(
            "sheet-ids file {} not found; tournaments with brackets will fail validation",
            path.display()
        );
        return Ok(HashMap::new());
    }
    let raw: RawSheetIds = Figment::new()
        .merge(Toml::file(path))
        .extract()
        .with_context(|| format!("loading sheet-ids from {}", path.display()))?;
    Ok(raw.sheet_ids)
}

fn validate(raw: RawConfig, sheet_ids: HashMap<String, String>) -> Result<Config> {
    let mut tournaments = HashMap::with_capacity(raw.tournaments.len());
    let mut missing_ids = Vec::new();

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

        // Tournaments with no brackets are placeholders — they short-circuit
        // to an empty response in the handler and don't need a sheet ID.
        let sheet_id = if t.brackets.is_empty() {
            sheet_ids.get(&t.slug).cloned().unwrap_or_default()
        } else {
            match sheet_ids.get(&t.slug) {
                Some(id) if !id.trim().is_empty() => id.clone(),
                _ => {
                    missing_ids.push(t.slug.clone());
                    String::new()
                }
            }
        };

        let slug = t.slug.clone();
        let entry = Tournament {
            slug,
            sheet_id,
            brackets: t.brackets,
        };
        if tournaments.insert(entry.slug.clone(), entry).is_some() {
            return Err(anyhow!("duplicate tournament slug '{}'", t.slug));
        }
    }

    if !missing_ids.is_empty() {
        return Err(anyhow!(
            "tournament(s) with configured brackets but no sheet_id in [sheet_ids]: {}",
            missing_ids.join(", ")
        ));
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
    fn loads_tournaments_and_merges_sheet_ids() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "ttlc2"

[[tournaments.brackets]]
name = "Obsidian"
group_ranges = ["G4:P9", "V4:AE9"]

[[tournaments.brackets]]
name = "Titanium"
group_ranges = ["G4:P9", "V4:AE9", "G23:P28"]
"#,
            )?;
            jail.create_file(
                "sheet-ids.toml",
                r#"
[sheet_ids]
ttlc2 = "sheet-123"
"#,
            )?;
            let cfg = Config::load(
                Path::new("does-not-exist-config.toml"),
                Path::new("tournaments.toml"),
                Path::new("sheet-ids.toml"),
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
    fn rejects_when_populated_tournament_missing_sheet_id() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "ttlc2"
[[tournaments.brackets]]
name = "Obsidian"
group_ranges = ["G4:P9"]
"#,
            )?;
            jail.create_file("sheet-ids.toml", "[sheet_ids]\n")?;
            let err = Config::load(
                Path::new("nope.toml"),
                Path::new("tournaments.toml"),
                Path::new("sheet-ids.toml"),
            )
            .unwrap_err()
            .to_string();
            assert!(err.contains("ttlc2"), "{err}");
            assert!(err.contains("no sheet_id"), "{err}");
            Ok(())
        });
    }

    #[test]
    fn allows_placeholder_tournament_with_no_brackets_and_no_id() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "spec2-nomad"
brackets = []
"#,
            )?;
            jail.create_file("sheet-ids.toml", "[sheet_ids]\n")?;
            let cfg = Config::load(
                Path::new("nope.toml"),
                Path::new("tournaments.toml"),
                Path::new("sheet-ids.toml"),
            )
            .map_err(|e| e.to_string())?;
            let t = cfg.tournaments.get("spec2-nomad").unwrap();
            assert_eq!(t.sheet_id, "");
            assert!(t.brackets.is_empty());
            Ok(())
        });
    }

    #[test]
    fn missing_sheet_ids_file_is_tolerated_for_placeholders() {
        Jail::expect_with(|jail| {
            jail.create_file(
                "tournaments.toml",
                r#"
[[tournaments]]
slug = "spec2-nomad"
brackets = []
"#,
            )?;
            let cfg = Config::load(
                Path::new("nope.toml"),
                Path::new("tournaments.toml"),
                Path::new("also-nope.toml"),
            )
            .map_err(|e| e.to_string())?;
            assert!(cfg.tournaments.contains_key("spec2-nomad"));
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

[[tournaments]]
slug = "x"
"#,
            )?;
            jail.create_file("sheet-ids.toml", "[sheet_ids]\n")?;
            let err = Config::load(
                Path::new("nope.toml"),
                Path::new("tournaments.toml"),
                Path::new("sheet-ids.toml"),
            )
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
brackets = []
"#,
            )?;
            jail.create_file("sheet-ids.toml", "[sheet_ids]\n")?;
            let cfg = Config::load(
                Path::new("nope.toml"),
                Path::new("tournaments.toml"),
                Path::new("sheet-ids.toml"),
            )
            .map_err(|e| e.to_string())?;
            assert_eq!(cfg.server.bind_addr, "0.0.0.0");
            assert_eq!(cfg.server.port, 8080);
            assert!(cfg.server.allowed_origins.is_empty());
            Ok(())
        });
    }
}
