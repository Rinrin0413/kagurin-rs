//! Utilities for [TETRA CHANNEL API](https://tetr.io/about/api/)

use crate::util::round_mid;
use tetr_ch::model::{prelude::*, summary::league::LeagueData, user::User};

/// Returns all the discord emoji of the given user's badges.
pub fn badge_emojis(user: &User) -> String {
    let mut result = String::new();
    let mut other_badges = 0;
    for b in &user.badges {
        #[rustfmt::skip]
        match b.id.to_string().as_ref() {
            "secretgrade" => {result.push_str("<:secretgrade:992079389611278477>");},
            "leaderboard1" => {result.push_str("<:leaderboard1:992095621018308759>");},
            "allclear" => {result.push_str("<:allclear:992096168664383622>");},
            "kod_founder" => {result.push_str("<:kod_founder:992096688653209610>");},
            "20tsd" => {result.push_str("<:20tsd:992097227260567553>");},
            "100player" => {result.push_str("<:100player:992097864081735730>");},
            "kod_by_founder" => {result.push_str("<:kod_by_founder:992099693599412274>");},
            "founder" => {result.push_str("<:founder:992100252641407076>");},
            "indev" => {result.push_str("<:indev:992100717726810214>");},
            // "infdev" => {result.push_str("<:infdev:992101142832107521>");},
            "poop" => {result.push_str("<:poop:992102034218156133>");},
            "heart" => {result.push_str("<:heart:992102426171686912>");},
            // "early-supporter" => {result.push_str("<:earlysupporter:992103936276303982>");},
            "bugbounty" => {result.push_str("<:bugbounty:992104885531197511>");},
            _ => {other_badges += 1;},
        };
    }
    if 0 < other_badges {
        result = format!("{} | and more {} badges", result, other_badges);
    }
    result
}

/// Returns the discord emoji of the given rank.
pub fn rank_emoji(rank: &Rank) -> String {
    format!(
        "<:{}>",
        match rank {
            Rank::D => "d_:993082933898391562",
            Rank::DPlus => "dp:993083934147616839",
            Rank::CMinus => "cm:993085497247289356",
            Rank::C => "c_:993085982909923450",
            Rank::CPlus => "cp:993086352893677639",
            Rank::BMinus => "bm:993086643957416006",
            Rank::B => "b_:993088403535036426",
            Rank::BPlus => "bp:993088785996857425",
            Rank::AMinus => "am:993089083830185985",
            Rank::A => "a_:993089364928245821",
            Rank::APlus => "ap:993089619845451848",
            Rank::SMinus => "sm:993089892915609601",
            Rank::S => "s_:993090162835865660",
            Rank::SPlus => "sp:993090398618656768",
            Rank::SS => "ss:993090656820002866",
            Rank::U => "u_:993091250154635335",
            Rank::X => "x_:993091489376776232",
            Rank::XPlus => "xp:1274835025321398384",
            Rank::Z => "z_:993091724547194931",
        }
    )
}

// Generates a progress bar of the given league data.
pub fn generate_progress_bar(league: &LeagueData) -> Option<String> {
    let current_standing = league.standing? as f64;
    let prev_at = league.prev_at? as f64;
    // let next_at = user.league.next_at as f64;
    let mut prev_rank = String::new();
    let mut next_rank = String::new();

    if let Some(r) = &league.prev_rank {
        prev_rank = rank_emoji(r);
    } else if current_standing == prev_at {
        return Some(format!(
            "👑`─────────────────|`{}\n　 ┗ Reached 0.00%",
            rank_emoji(&Rank::DPlus)
        ));
    }
    if let Some(r) = &league.next_rank {
        next_rank = rank_emoji(r);
    } else if current_standing == 1. {
        return Some(format!(
            "{}`|━━━━━━━━━━━━━━━━━`👑\n**𝐓𝐄𝐓𝐑𝐀 𝐋𝐄𝐀𝐆𝐔𝐄 𝐂𝐇𝐀𝐌𝐏𝐈𝐎𝐍**",
            rank_emoji(&Rank::X),
        ));
    }

    let progress = league.rank_progress()? / 100.;
    let bar_step = 17.;
    let mut progress_step = (progress * (bar_step - 1.)).ceil() as i8 + 1;
    if progress_step <= 0 {
        progress_step = 1;
    }

    let mut bar = String::new();
    for _ in 1..progress_step {
        bar.push('━');
    }
    if progress_step <= 1 {
        bar.push('┠');
    } else {
        bar.push('╋');
    }
    for _ in progress_step..bar_step as i8 {
        bar.push('─');
    }

    Some(format!(
        "{}`|{}|`{}\n　 ┗ Reached {:.2}%",
        prev_rank,
        bar,
        next_rank,
        league.rank_progress()?
    ))
}

/// Formats the given `gametime` in seconds to a human-readable string.
pub fn fmt_gametime(gametime: f64) -> String {
    let h = gametime / 3600.;
    let m = h / 60.;
    let s = m / 60.;
    let (value, unit) = if 1. <= h {
        (h, "hours")
    } else if 1.0 <= m {
        (m, "minutes")
    } else {
        (s, "seconds")
    };
    format!("{:.2} {}", value, unit)
}

pub fn fmt_forty_lines_time(final_time: f64) -> String {
    // `final_time` is in milliseconds,
    // so returns the quotient of one thousand.
    let time = final_time / 1000.;
    let m = (time / 60.).floor();
    let s = round_mid(time - m * 60., 3);
    let fmted_s = format!("{}{}", if s < 10. { "0" } else { "" }, s);
    format!("{}:{}", m, fmted_s)
}

// pub fn fmt_finesse(endcontext: &SinglePlayEndCtx) -> String {
//     let finesse = match &endcontext.finesse {
//         Some(f) => f,
//         None => {
//             return "unavailable".to_string();
//         }
//     };
//     format!(
//         "{}F ({:.2}%)",
//         finesse.faults.unwrap(),
//         round_mid(
//             finesse.perfect_pieces.unwrap() as f64 / endcontext.pieces_placed.unwrap() as f64,
//             4
//         ) * 100.
//     )
// }

/// Returns the symbol of the given level.
pub fn level_symbol(level: u32) -> char {
    if level < 100 {
        return '/';
    }
    if 5000 <= level {
        return '❯';
    }
    let digits = level.to_string().len();
    if let Some(hundreds_place) = level.to_string().chars().nth(digits - 3) {
        match hundreds_place {
            '0' | '5' => '/',
            '1' | '6' => '▲',
            '2' | '7' => '◆',
            '3' | '8' => '⭓',
            '4' | '9' => '⬢',
            _ => '/',
        }
    } else {
        '/'
    }
}

pub fn rank_col(rank: &Rank, percentile_rank: &Option<Rank>) -> u32 {
    if rank.is_unranked() {
        if let Some(r) = percentile_rank {
            r.color()
        } else {
            Rank::Z_COL
        }
    } else {
        rank.color()
    }
}
