use std::convert::TryFrom;

const MIN_COACH_STR: i32 = 50;

#[derive(Debug, Clone, PartialEq)]
pub enum Result{
    Winner(String),
    Draw,
}

pub enum Position{
    Goalkeeper,
    Field,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player{
    name : String,
    position : Position,
    strength : i32,
}

pub struct Coach{
    name : String,
    strength : i32,
}
impl TryFrom<Player> for Coach {
    type Error = String;

    fn try_from(player: Player) -> Result<Self, Self::Error> {
        if player.strength >= MIN_COACH_STR {
            Ok(Coach {
                name: player.name,
                strength: player.strength,
            })
        } else {
            Err(format!(
                "Coach {} has too low strength: {} < {}",
                player.name, player.strength, MIN_COACH_STR
            ))
        }
    }
}
pub mod team {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    pub struct Team {
        pub name: String,
        pub players: Vec<Player>,
        pub coach: Option<Coach>,
    }

    impl From<Vec<Player>> for Team {
        fn from(players: Vec<Player>) -> Self {
            Team {
                name: String::new(),
                players,
                coach: None,
            }
        }
    }

    pub fn r#match(team1: &Team, team2: &Team) -> Result {
        // jeśli któraś drużyna ma mniej niż 11 zawodników -> przegrywa
        if team1.players.len() < 11 && team2.players.len() < 11 {
            return Result::Draw;
        }
        if team1.players.len() < 11 {
            return Result::Winner(team2.name.clone());
        }
        if team2.players.len() < 11 {
            return Result::Winner(team1.name.clone());
        }

        fn team_strength(team: &Team) -> i32 {
            // najpierw bramkarz
            let mut goalkeepers: Vec<&Player> = team
                .players
                .iter()
                .filter(|p| matches!(p.position, Position::Goalkeeper))
                .collect();
            goalkeepers.sort_by_key(|p| -p.strength);

            let gk_strength = goalkeepers.first().map(|p| p.strength).unwrap_or(0);

            // 10 najsilniejszych z pola
            let mut field_players: Vec<&Player> = team
                .players
                .iter()
                .filter(|p| matches!(p.position, Position::Field))
                .collect();
            field_players.sort_by_key(|p| -p.strength);
            let field_strength: i32 = field_players.iter().take(10).map(|p| p.strength).sum();

            let coach_strength = team.coach.as_ref().map(|c| c.strength).unwrap_or(0);

            gk_strength + field_strength + coach_strength
        }

        let s1 = team_strength(team1);
        let s2 = team_strength(team2);

        if s1 > s2 {
            Result::Winner(team1.name.clone())
        } else if s2 > s1 {
            Result::Winner(team2.name.clone())
        } else {
            Result::Draw
        }
    }

    #[cfg(test)]
    pub mod tests {
        use super::*;

        fn sample_players(n: usize, pos: Position, base: i32) -> Vec<Player> {
            (0..n)
                .map(|i| Player {
                    name: format!("P{}", i),
                    position: pos.clone(),
                    strength: base + i as i32,
                })
                .collect()
        }

        #[test]
        fn create_team_from_vec() {
            let players = sample_players(5, Position::Field, 10);
            let team: Team = players.clone().into();
            assert_eq!(team.players.len(), 5);
            assert!(team.coach.is_none());
        }

        #[test]
        fn create_coach_from_player() {
            let p = Player {
                name: "Coach1".to_string(),
                position: Position::Field,
                strength: MIN_COACH_STR,
            };
            let coach = Coach::try_from(p).unwrap();
            assert_eq!(coach.strength, MIN_COACH_STR);

            let weak = Player {
                name: "Weak".to_string(),
                position: Position::Field,
                strength: MIN_COACH_STR - 1,
            };
            assert!(Coach::try_from(weak).is_err());
        }

        #[test]
        fn match_correctness() {
            let mut players1 = sample_players(10, Position::Field, 50);
            players1.push(Player {
                name: "GK1".to_string(),
                position: Position::Goalkeeper,
                strength: 60,
            });
            let mut team1: Team = players1.into();
            team1.name = "Team1".to_string();
            team1.coach = Some(Coach {
                name: "Coach1".to_string(),
                strength: 70,
            });

            let mut players2 = sample_players(10, Position::Field, 30);
            players2.push(Player {
                name: "GK2".to_string(),
                position: Position::Goalkeeper,
                strength: 40,
            });
            let mut team2: Team = players2.into();
            team2.name = "Team2".to_string();
            team2.coach = Some(Coach {
                name: "Coach2".to_string(),
                strength: 55,
            });

            let res = r#match(&team1, &team2);
            assert_eq!(res, Result::Winner("Team1".to_string()));
        }
    }
}