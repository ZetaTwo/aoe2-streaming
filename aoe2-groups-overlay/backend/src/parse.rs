use serde::{Deserialize, Serialize};

use crate::config::Tournament;

#[derive(Debug, Deserialize)]
pub struct BatchGetResponse {
    #[serde(rename = "valueRanges", default)]
    pub value_ranges: Vec<ValueRange>,
}

#[derive(Debug, Deserialize, Default)]
pub struct ValueRange {
    #[serde(default)]
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStanding {
    pub rank: u32,
    pub name: String,
    pub sets_won: u32,
    pub sets_lost: u32,
    pub maps_won: u32,
    pub maps_lost: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct GroupStanding {
    pub name: String,
    pub players: Vec<PlayerStanding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct BracketStanding {
    pub name: String,
    pub groups: Vec<GroupStanding>,
}

pub fn parse_tournament(
    tournament: &Tournament,
    value_ranges: &[ValueRange],
) -> Vec<BracketStanding> {
    let mut brackets = Vec::with_capacity(tournament.brackets.len());
    let mut range_idx = 0usize;
    for bracket in &tournament.brackets {
        let mut groups = Vec::new();
        let mut group_letter = 0u8;
        for _ in &bracket.group_ranges {
            if range_idx >= value_ranges.len() {
                break;
            }
            let vr = &value_ranges[range_idx];
            range_idx += 1;
            // Empty ranges are skipped without advancing the group letter,
            // matching the existing frontend behaviour.
            if vr.values.is_empty() {
                continue;
            }
            let name = format!("Group {}", (b'A' + group_letter) as char);
            group_letter += 1;
            groups.push(GroupStanding {
                name,
                players: parse_group(&vr.values),
            });
        }
        brackets.push(BracketStanding {
            name: bracket.name.clone(),
            groups,
        });
    }
    brackets
}

fn parse_group(rows: &[Vec<serde_json::Value>]) -> Vec<PlayerStanding> {
    let mut players = Vec::new();
    for row in rows {
        let name = match row.get(1) {
            Some(serde_json::Value::String(s)) => s.trim().to_string(),
            _ => continue,
        };
        let lower = name.to_ascii_lowercase();
        if name.is_empty() || lower == "name" || lower == "player" {
            continue;
        }
        let rank = cell_as_u32(row.first()).unwrap_or_else(|| players.len() as u32 + 1);
        let sets_won = cell_as_u32(row.get(4)).unwrap_or(0);
        let sets_lost = cell_as_u32(row.get(6)).unwrap_or(0);
        let maps_won = cell_as_u32(row.get(7)).unwrap_or(0);
        let maps_lost = cell_as_u32(row.get(9)).unwrap_or(0);
        players.push(PlayerStanding {
            rank,
            name,
            sets_won,
            sets_lost,
            maps_won,
            maps_lost,
        });
    }
    players
}

fn cell_as_u32(cell: Option<&serde_json::Value>) -> Option<u32> {
    match cell? {
        serde_json::Value::String(s) => s.trim().parse().ok(),
        serde_json::Value::Number(n) => n.as_u64().and_then(|x| u32::try_from(x).ok()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Bracket, Tournament};
    use pretty_assertions::assert_eq;

    fn make_tournament(brackets: Vec<Bracket>) -> Tournament {
        Tournament {
            slug: "test".into(),
            sheet_id: "sid".into(),
            brackets,
        }
    }

    fn bracket(name: &str, group_count: usize) -> Bracket {
        Bracket {
            name: name.into(),
            group_ranges: (0..group_count).map(|i| format!("R{i}")).collect(),
        }
    }

    fn vr_from_json(rows: serde_json::Value) -> ValueRange {
        ValueRange {
            values: serde_json::from_value(rows).expect("rows -> values"),
        }
    }

    #[test]
    fn parses_strings_and_numbers() {
        let vr = vr_from_json(serde_json::json!([
            ["1", "Alice", "", "", "3", "-", "1", "9", "-", "4"],
            [2, "Bob", "", "", 2, "-", 2, 7, "-", 6],
        ]));
        let t = make_tournament(vec![bracket("Champions", 1)]);
        let out = parse_tournament(&t, &[vr]);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].name, "Champions");
        assert_eq!(out[0].groups.len(), 1);
        assert_eq!(out[0].groups[0].name, "Group A");
        assert_eq!(
            out[0].groups[0].players,
            vec![
                PlayerStanding {
                    rank: 1,
                    name: "Alice".into(),
                    sets_won: 3,
                    sets_lost: 1,
                    maps_won: 9,
                    maps_lost: 4,
                },
                PlayerStanding {
                    rank: 2,
                    name: "Bob".into(),
                    sets_won: 2,
                    sets_lost: 2,
                    maps_won: 7,
                    maps_lost: 6,
                },
            ]
        );
    }

    #[test]
    fn skips_header_and_empty_rows() {
        let vr = vr_from_json(serde_json::json!([
            ["#", "Player"],
            ["", "  "],
            ["1", "  Alice  ", "", "", "1", "-", "0", "2", "-", "0"],
        ]));
        let t = make_tournament(vec![bracket("X", 1)]);
        let out = parse_tournament(&t, &[vr]);
        assert_eq!(out[0].groups[0].players.len(), 1);
        assert_eq!(out[0].groups[0].players[0].name, "Alice");
    }

    #[test]
    fn falls_back_rank_to_position_when_missing() {
        let vr = vr_from_json(serde_json::json!([
            ["", "Alice", "", "", "1"],
            ["", "Bob", "", "", "0"],
        ]));
        let t = make_tournament(vec![bracket("X", 1)]);
        let out = parse_tournament(&t, &[vr]);
        let players = &out[0].groups[0].players;
        assert_eq!(players[0].rank, 1);
        assert_eq!(players[1].rank, 2);
    }

    #[test]
    fn labels_groups_a_b_c_and_skips_empty_ranges_without_advancing() {
        let vr_a = vr_from_json(serde_json::json!([[
            "1", "A1", "", "", "1", "-", "0", "1", "-", "0"
        ]]));
        let vr_empty = ValueRange::default();
        let vr_b = vr_from_json(serde_json::json!([[
            "1", "B1", "", "", "1", "-", "0", "1", "-", "0"
        ]]));
        let t = make_tournament(vec![bracket("Tin", 3)]);
        let out = parse_tournament(&t, &[vr_a, vr_empty, vr_b]);
        assert_eq!(out[0].groups.len(), 2);
        assert_eq!(out[0].groups[0].name, "Group A");
        assert_eq!(out[0].groups[1].name, "Group B");
        assert_eq!(out[0].groups[0].players[0].name, "A1");
        assert_eq!(out[0].groups[1].players[0].name, "B1");
    }

    #[test]
    fn json_uses_camel_case() {
        let player = PlayerStanding {
            rank: 1,
            name: "Alice".into(),
            sets_won: 3,
            sets_lost: 1,
            maps_won: 9,
            maps_lost: 4,
        };
        let json = serde_json::to_value(&player).unwrap();
        assert_eq!(json["setsWon"], 3);
        assert_eq!(json["setsLost"], 1);
        assert_eq!(json["mapsWon"], 9);
        assert_eq!(json["mapsLost"], 4);
    }

    #[test]
    fn multiple_brackets_consume_ranges_in_order() {
        let vr1 = vr_from_json(serde_json::json!([[
            "1", "A", "", "", "1", "-", "0", "1", "-", "0"
        ]]));
        let vr2 = vr_from_json(serde_json::json!([[
            "1", "B", "", "", "1", "-", "0", "1", "-", "0"
        ]]));
        let t = make_tournament(vec![bracket("First", 1), bracket("Second", 1)]);
        let out = parse_tournament(&t, &[vr1, vr2]);
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].groups[0].players[0].name, "A");
        assert_eq!(out[1].groups[0].players[0].name, "B");
    }
}
