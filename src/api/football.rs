use serde::{Deserialize, Serialize};

pub type FootballRes = Vec<Match>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Match {
    pub matchID: u64,
    pub matchDateTime: String,
    pub timeZoneID: String,
    pub leagueId: u64,
    pub leagueName: String,
    pub leagueSeason: u64,
    pub leagueShortcut: String,
    pub matchDateTimeUTC: String,
    pub group: Group,
    pub team1: Team,
    pub team2: Team,
    pub lastUpdateDateTime: String,
    pub matchIsFinished: bool,
    pub matchResults: Vec<MatchResult>,
    pub goals: Vec<Goal>,
    pub location: Location,
    pub numberOfViewers: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Group {
    pub groupName: String,
    pub groupOrderID: u64,
    pub groupID: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Team {
    pub teamId: u64,
    pub teamName: String,
    pub shortName: String,
    pub teamIconUrl: String,
    pub teamGroupName: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct MatchResult {
    pub resultID: u64,
    pub resultName: String,
    pub pointsTeam1: u64,
    pub pointsTeam2: u64,
    pub resultOrderID: u64,
    pub resultTypeID: u64,
    pub resultDescription: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Goal {
    pub goalID: u64,
    pub scoreTeam1: u64,
    pub scoreTeam2: u64,
    pub matchMinute: u64,
    pub goalGetterID: u64,
    pub goalGetterName: String,
    pub isPenalty: bool,
    pub isOwnGoal: bool,
    pub isOvertime: bool,
    pub comment: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Location {
    pub locationID: u64,
    pub locationCity: String,
    pub locationStadium: String,
}
