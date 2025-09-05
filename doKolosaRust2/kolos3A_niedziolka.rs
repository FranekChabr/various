// lib.rs (albo main.rs) — kompletne rozwiązanie

use std::convert::TryFrom;

pub const MIN_COACH_STR: i32 = 50; // wartość przykładowa (stała wymagana przez zadanie)

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Position {
    Goalkeeper,
    Field,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub position: Position,
    pub strength: i32,
}

#[derive(Debug, Clone)]
pub struct Coach {
    pub name: String,
    pub strength: i32,
}

// Zgodnie z treścią zadania: enum Result: Winner(String) lub Draw.
// Nazwa "Result" jest dozwolona, ale by nie mylić ze std::result::Result,
// używamy jej bezpośrednio (można też przemianować).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Result {
    Winner(String),
    Draw,
}

pub mod team {
    use super::*;
    // Struktura Team:
    #[derive(Debug, Clone)]
    pub struct Team {
        pub name: String,
        pub players: Vec<Player>,
        pub coach: Option<Coach>,
    }

    // Implementacja From<Vec<Player>> dla Team — nazwa pusta.
    impl From<Vec<Player>> for Team {
        fn from(v: Vec<Player>) -> Self {
            Team {
                name: String::new(),
                players: v,
                coach: None,
            }
        }
    }

    // Implementacja TryFrom<Player> dla Coach.
    // Jeżeli siła gracza >= MIN_COACH_STR -> konwersja się uda.
    impl TryFrom<Player> for Coach {
        type Error = String;

        fn try_from(p: Player) -> std::result::Result<Self, Self::Error> {
            if p.strength >= MIN_COACH_STR {
                Ok(Coach {
                    name: p.name,
                    strength: p.strength,
                })
            } else {
                Err(format!(
                    "player '{}' too weak to be coach (strength {})",
                    p.name, p.strength
                ))
            }
        }
    }

    // Funkcja oceniająca mecz. Nazwana `match_` ponieważ `match` to keyword w Rust.
    // Zwraca super::Result (Winner(team_name) lub Draw).
    //
    // Zasady (wg PDF):
    // - Jeżeli jedna z drużyn ma mniej niż 11 zawodników -> ta drużyna przegrywa.
    // - Sumuje się: siła 10 najsilniejszych zawodników (pozycja Field),
    //   siła dokładnie jednego (najsilniejszego) bramkarza oraz siła trenera (jeżeli jest).
    // - Drużyna z większą sumaryczną siłą wygrywa; przy równej sile -> Draw.
    pub fn match_(team1: &Team, team2: &Team) -> super::Result {
        // Sprawdzenie liczby zawodników:
        let t1_count = team1.players.len();
        let t2_count = team2.players.len();

        let t1_incomplete = t1_count < 11;
        let t2_incomplete = t2_count < 11;

        if t1_incomplete && !t2_incomplete {
            return super::Result::Winner(team2.name.clone());
        }
        if t2_incomplete && !t1_incomplete {
            return super::Result::Winner(team1.name.clone());
        }
        if t1_incomplete && t2_incomplete {
            // obie drużyny mają mniej niż 11 -> traktujemy jako remis
            return super::Result::Draw;
        }

        // Funkcja pomocnicza: oblicza sumaryczną siłę drużyny wg zasad
        fn team_power(t: &Team) -> i32 {
            // Najsilniejszy bramkarz (jeżeli jest)
            let mut gk_strength = t
                .players
                .iter()
                .filter(|p| matches!(p.position, Position::Goalkeeper))
                .map(|p| p.strength)
                .max()
                .unwrap_or(0);

            // 10 najsilniejszych "field" zawodników
            let mut field_players: Vec<&Player> = t
                .players
                .iter()
                .filter(|p| matches!(p.position, Position::Field))
                .collect();

            field_players.sort_by(|a, b| b.strength.cmp(&a.strength));
            let top_10_sum: i32 = field_players
                .iter()
                .take(10)
                .map(|p| p.strength)
                .sum();

            // Trener (jeśli jest)
            let coach_strength = t.coach.as_ref().map(|c| c.strength).unwrap_or(0);

            // Suma końcowa
            top_10_sum + gk_strength + coach_strength
        }

        let p1 = team_power(team1);
        let p2 = team_power(team2);

        if p1 > p2 {
            super::Result::Winner(team1.name.clone())
        } else if p2 > p1 {
            super::Result::Winner(team2.name.clone())
        } else {
            super::Result::Draw
        }
    }

    // Testy jednostkowe
    #[cfg(test)]
    mod tests {
        use super::*;
        use std::convert::TryFrom;

        #[test]
        fn from_vec_creates_team_with_empty_name() {
            let players = vec![
                Player {
                    name: "A".into(),
                    position: Position::Field,
                    strength: 10,
                },
                Player {
                    name: "B".into(),
                    position: Position::Goalkeeper,
                    strength: 20,
                },
            ];
            let team = Team::from(players.clone());
            assert_eq!(team.name, "");
            assert_eq!(team.players.len(), 2);
            assert!(team.coach.is_none());
            assert_eq!(team.players[0].name, "A");
        }

        #[test]
        fn try_from_player_to_coach_respects_min_coach_str() {
            // gracz o sile poniżej MIN_COACH_STR -> błąd
            let weak = Player {
                name: "Weak".into(),
                position: Position::Field,
                strength: MIN_COACH_STR - 1,
            };
            let res = Coach::try_from(weak);
            assert!(res.is_err());

            // gracz o sile równej MIN_COACH_STR -> ok
            let strong = Player {
                name: "Strong".into(),
                position: Position::Field,
                strength: MIN_COACH_STR,
            };
            let c = Coach::try_from(strong).expect("should convert to coach");
            assert_eq!(c.strength, MIN_COACH_STR);
        }

        #[test]
        fn match_logic_counts_top_players_goalkeeper_and_coach() {
            // Stworzymy dwie drużyny po 11 zawodników.
            // Team1 będzie silniejszy dzięki trenerowi i mocnym field players.

            // Team1 players: 10 field players with strengths 10..19, 1 goalkeeper 15
            let mut p1: Vec<Player> = (10..20)
                .map(|s| Player {
                    name: format!("P{}", s),
                    position: Position::Field,
                    strength: s,
                })
                .collect();
            p1.push(Player {
                name: "GK1".into(),
                position: Position::Goalkeeper,
                strength: 15,
            });

            let mut team1 = Team::from(p1);
            team1.name = "Team1".into();
            team1.coach = Some(Coach {
                name: "Coach1".into(),
                strength: 20,
            });

            // Team2 players: 10 field players with strengths 5..14, 1 goalkeeper 30
            let mut p2: Vec<Player> = (5..15)
                .map(|s| Player {
                    name: format!("Q{}", s),
                    position: Position::Field,
                    strength: s,
                })
                .collect();
            p2.push(Player {
                name: "GK2".into(),
                position: Position::Goalkeeper,
                strength: 30,
            });

            let mut team2 = Team::from(p2);
            team2.name = "Team2".into();
            team2.coach = Some(Coach {
                name: "Coach2".into(),
                strength: 5,
            });

            // Obliczmy kto wygra:
            let res = match_( &team1, &team2 );
            // team1 ma mocne field players (10..19 sum = 145), gk=15, coach=20 -> total 180
            // team2 field (5..14 sum = 95), gk=30, coach=5 -> total 130
            assert_eq!(res, super::super::Result::Winner("Team1".into()));
        }

        #[test]
        fn team_with_less_than_11_players_loses() {
            let mut full_players = vec![];
            for i in 0..11 {
                full_players.push(Player {
                    name: format!("A{}", i),
                    position: Position::Field,
                    strength: 10,
                });
            }
            let mut team_full = Team::from(full_players);
            team_full.name = "Full".into();

            let mut small = vec![];
            for i in 0..10 {
                small.push(Player {
                    name: format!("B{}", i),
                    position: Position::Field,
                    strength: 100,
                });
            }
            let mut team_small = Team::from(small);
            team_small.name = "Small".into();

            let res = match_(&team_full, &team_small);
            // small has <11 so small loses -> full wins
            assert_eq!(res, super::super::Result::Winner("Full".into()));
        }
    }
}
