use crate::core;
use crate::core::game;
use crate::core::summoner;

pub fn tier(tier: summoner::Tier) -> String {
    match tier {
        summoner::Tier::Iron(_) => "Iron",
        summoner::Tier::Bronze(_) => "Bronze",
        summoner::Tier::Silver(_) => "Silver",
        summoner::Tier::Gold(_) => "Gold",
        summoner::Tier::Platinum(_) => "Platinum",
        summoner::Tier::Emerald(_) => "Emerald",
        summoner::Tier::Diamond(_) => "Diamond",
        summoner::Tier::Master(_) => "Master",
        summoner::Tier::Grandmaster(_) => "Grandmaster",
        summoner::Tier::Challenger(_) => "Challenger",
    }
    .to_string()
}

pub fn division_or_points(tier: summoner::Tier) -> String {
    match tier {
        summoner::Tier::Iron(division)
        | summoner::Tier::Bronze(division)
        | summoner::Tier::Silver(division)
        | summoner::Tier::Gold(division)
        | summoner::Tier::Platinum(division)
        | summoner::Tier::Emerald(division)
        | summoner::Tier::Diamond(division) => self::division(division),
        summoner::Tier::Master(points)
        | summoner::Tier::Grandmaster(points)
        | summoner::Tier::Challenger(points) => points.to_string(),
    }
}

pub fn division(division: summoner::Division) -> String {
    match division {
        summoner::Division::One(_) => "1",
        summoner::Division::Two(_) => "2",
        summoner::Division::Three(_) => "3",
        summoner::Division::Four(_) => "4",
    }
    .to_string()
}

pub fn duration(duration: time::Duration) -> String {
    let minutes = duration.whole_minutes().to_string();
    let seconds = duration.whole_seconds().to_string();

    format!("{minutes:.2}m {seconds:.2}s")
}

pub fn time_since(now: time::OffsetDateTime, since: time::OffsetDateTime) -> String {
    // let now = time::OffsetDateTime::now_utc();
    let duration = now - since;
    let seconds = duration.whole_seconds();
    let minutes = seconds / 60;
    let hours = minutes / 60;
    let days = hours / 24;
    let weeks = days / 7;
    let months = days / 30;
    let years = days / 365;

    if seconds < 60 {
        String::from("few seconds ago")
    } else if minutes < 60 {
        format!(
            "{} minute{} ago",
            minutes,
            if minutes == 1 { "" } else { "s" }
        )
    } else if hours < 24 {
        format!("{} hour{} ago", hours, if hours == 1 { "" } else { "s" })
    } else if days < 7 {
        if days == 1 {
            String::from("yesterday")
        } else {
            format!("{} days ago", days)
        }
    } else if weeks < 4 {
        if weeks == 1 {
            String::from("last week")
        } else {
            format!("{} weeks ago", weeks)
        }
    } else if months < 12 {
        if months <= 1 {
            String::from("last month")
        } else {
            format!("{} months ago", months)
        }
    } else {
        if years <= 1 {
            return String::from("last year");
        } else {
            format!("{} years ago", years)
        }
    }
}

pub fn role(role: core::game::Role) -> String {
    match role {
        core::game::Role::Bottom => "Bottom",
        core::game::Role::Jungle => "Jungle",
        core::game::Role::Mid => "Mid",
        core::game::Role::Support => "Support",
        core::game::Role::Top => "Top",
    }
    .to_string()
}

pub fn team(team: core::Team) -> String {
    match team {
        core::Team::BLUE => "Blue team",
        core::Team::RED => "Red team",
        _ => unimplemented!(),
    }
    .to_string()
}

pub fn win(result: game::Result) -> String {
    match result {
        game::Result::Remake => "Remake",
        game::Result::Defeat | game::Result::Surrender => "Defeat",
        game::Result::Victory => "Victory",
    }
    .to_string()
}

pub fn kda(kills: u32, deaths: u32, assists: u32) -> String {
    let mut kda = (kills as f32 + assists as f32) / deaths as f32;
    if !kda.is_normal() {
        kda = 0.0;
    }
    format!("{kda:.2} KDA")
}

pub fn creep_score(creep_score: u32, minutes: u32) -> String {
    let mut cs_per_minute = creep_score as f32 / minutes as f32;
    if !cs_per_minute.is_normal() {
        cs_per_minute = 0.0;
    }
    format!("{creep_score} CS ({cs_per_minute:.1})")
}

pub fn vision_score(vision_score: u32) -> String {
    format!("{vision_score} vision")
}
