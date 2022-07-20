//! Utilities for [TETRA CHANNEL API](https://tetr.io/about/api/)

use crate::util::round_mid;
use chrono::DateTime;
use reqwest::Response;
use serde::Deserialize;
use thousands::Separable;

// TODO:testing

/// ### User Info
/// An struct describing the user in detail.
#[derive(Deserialize)]
pub struct TetraUser {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<Cache>,
    /// The requested data.
    pub data: Option<UserData>,
}

impl TetraUser {
    /// Creates a new `TetraUser`.
    /// Converts the given `Response` to `TetraUser` and returns it.
    pub async fn new(response: Response) -> Self {
        response
            .json::<Self>()
            .await
            .expect("USER DATA PARSE TO JSON")
    }

    /// Returns a `Result<(), &str>`.
    ///
    /// If the `success` field is `true`, returns a `Ok(())`.
    /// If the `success` field is `false`, returns `Err(String)`.
    ///
    /// # Errors
    ///
    /// An error is returned, e.g. if the user cannot be found
    pub fn get_err(&self) -> Result<(), &str> {
        if self.success {
            Ok(())
        } else {
            Err(self.error.as_ref().expect("WTF SUCCESS?"))
        }
    }

    /// Returns the user's avatar URL.
    /// If the user has no avatar, returns anon's.
    pub fn get_face_url(&self) -> String {
        let default = "https://tetr.io/res/avatar.png".to_string();
        if let Some(ar) = self
            .data
            .as_ref()
            .expect("WTF SUCCESS???")
            .user
            .avatar_revision
        {
            if ar == 0 {
                return default;
            }
            format!(
                "https://tetr.io/user-content/avatars/{}.jpg?rv={}",
                self.data.as_ref().expect("WTF SUCCESS??").user._id,
                ar
            )
        } else {
            default
        }
    }

    /// Returns a hexadecimal color code based on rank.
    /// If the user is not ranked, returns the `0x2f3136` or based on the percentile.
    pub fn get_col(&self) -> u32 {
        let mut rank = self
            .data
            .as_ref()
            .expect("WTF SUCCESS????")
            .user
            .league
            .rank
            .as_str();
        if rank == "z" {
            rank = self
                .data
                .as_ref()
                .expect("WTF SUCCESS?????")
                .user
                .league
                .percentile_rank
                .as_str();
        }
        match rank {
            "d" => 0x907591,
            "d+" => 0x8e6091,
            "c-" => 0x79558c,
            "c" => 0x733e8f,
            "c+" => 0x552883,
            "b-" => 0x5650c7,
            "b" => 0x4f64c9,
            "b+" => 0x4f99c0,
            "a-" => 0x3bb687,
            "a" => 0x46ad51,
            "a+" => 0x1fa834,
            "s-" => 0xb2972b,
            "s" => 0xe0a71b,
            "s+" => 0xd8af0e,
            "ss" => 0xdb8b1f,
            "u" => 0xff3813,
            "x" => 0xff45ff,
            _ => 0x2f3136,
        }
    }

    /// Returns the Option<String>.
    ///
    /// If user is displaying the country,
    /// returns Some() with an image of the national flag based on the user's ISO 3166-1 country code.
    /// If the user is not displaying the country, returns `None`.
    pub fn get_national_flag(&self) -> Option<String> {
        if let Some(cc) = &self.data.as_ref().expect("WTF SUCCESS??????").user.country {
            Some(format!(
                "https://tetr.io/res/flags/{}.png",
                cc.to_lowercase()
            ))
        } else {
            None
        }
    }

    /// Returns the user's role name.
    pub fn get_role_name(&self) -> String {
        String::from(
            match self
                .data
                .as_ref()
                .expect("WTF SUCCESS!?")
                .user
                .role
                .as_str()
            {
                "anon" => "Anonymous",
                "user" => "User",
                "bot" => "Bot",
                "mod" => "Moderator",
                "admin" => "Administrator",
                // ┌ If the user is banned, use another embed
                //"banned" => "Banned User", // EXCEPTION
                unknown => return format!("Unknown Role ({})", unknown),
            },
        )
    }

    /// Returns a boolean that indicates whether the user has any badge.
    pub fn has_badges(&self) -> bool {
        !self
            .data
            .as_ref()
            .expect("WTF SUCCESS!?!?")
            .user
            .badges
            .is_empty()
    }

    /// Returns some emojis of the user's badges.
    ///
    /// Badges that appear to be competition badges are
    /// only that number of it will be displayed.
    pub fn get_bages(&self) -> String {
        let mut result = String::new();
        let mut competition_badges = 0;
        for b in &self.data.as_ref().expect("WTF SUCCESS!?!?!?").user.badges {
            #[rustfmt::skip]
            match b.id.as_str() {
                "secretgrade" => {result.push_str("<:secretgrade:992079389611278477>");},
                "leaderboard1" => {result.push_str("<:leaderboard1:992095621018308759>");},
                "allclear" => {result.push_str("<:allclear:992096168664383622>");},
                "kod_founder" => {result.push_str("<:kod_founder:992096688653209610>");},
                "20tsd" => {result.push_str("<:20tsd:992097227260567553>");},
                "100player" => {result.push_str("<:100player:992097864081735730>");},
                "kod_by_founder" => {result.push_str("<:kod_by_founder:992099693599412274>");},
                "founder" => {result.push_str("<:founder:992100252641407076>");},
                "indev" => {result.push_str("<:indev:992100717726810214>");},
                "infdev" => {result.push_str("<:infdev:992101142832107521>");},
                "poop" => {result.push_str("<:poop:992102034218156133>");},
                "heart" => {result.push_str("<:heart:992102426171686912>");},
                "early-supporter" => {result.push_str("<:earlysupporter:992103936276303982>");},
                "bugbounty" => {result.push_str("<:bugbounty:992104885531197511>");},
                _ => {competition_badges += 1;},
            };
        }
        if 0 < competition_badges {
            result.push_str(&format!(" | More {} badges", competition_badges))
        }
        result
    }

    /// Returns the user's xp(separated with commas).
    pub fn get_formatted_xp(&self) -> String {
        self.data
            .as_ref()
            .expect("WTF SUCCESS!?!?!?!?")
            .user
            .xp
            .separate_with_commas()
    }

    /// Returns the level based on the user's xp.
    pub fn get_level(&self) -> u32 {
        fn max(v1: f64, v2: f64) -> f64 {
            if v1 < v2 {
                v2
            } else {
                v1
            }
        }
        let xp = self.data.as_ref().expect("WTF SUCCESS?!").user.xp;
        // (xp/500)^0.6 + (xp / (5000 + max(0, xp-4000000) / 5000)) + 1
        let level = ((xp / 500.).powf(0.6) + (xp / (5000. + max(0., xp - 4000000.) / 5000.)) + 1.)
            .floor() as u32;
        level
    }

    /// Returns the user's ID.
    pub fn get_id(&self) -> &str {
        &self.data.as_ref().expect("WTF SUCCESS").user._id
    }

    /// Returns a `char`.
    ///
    /// This `char` is the mark for the level
    /// and varies according to the hundreds place of the level.
    pub fn get_level_mark(&self) -> char {
        let level = self.get_level();
        if level < 100 {
            return '/';
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

    /// Retrun a boolean that indicates whether the is user is hiding `gameplay`.
    pub fn is_gameplay_hidden(&self) -> bool {
        self.data.as_ref().expect("WTF SUCCESS?!?").user.gametime < 0.
    }

    /// Returns the user's formatted playtime.
    pub fn get_gametime(&self) -> String {
        let gametime = self.data.as_ref().expect("WTF SUCCESS?!?!").user.gametime;
        let h = gametime / 3600.;
        let m = h / 60.;
        let s = m / 60.;
        let (value, unit) = if 1.0 <= h {
            (h, "hours")
        } else if 1.0 <= m {
            (m, "minutes")
        } else {
            (s, "seconds")
        };
        format!("{:.2} {}", value, unit)
    }

    /// Retrun a boolean that indicates whether the is user is banned.
    pub fn is_banned(&self) -> bool {
        self.data.as_ref().expect("WTF SUCCESS?!?!?").user.role == "banned"
    }

    /// Returns a boolean that indicates whether the user is BAD STANDING.
    pub fn is_bad_standing(&self) -> bool {
        if let Some(b) = self
            .data
            .as_ref()
            .expect("WTF SUCCESS?!?!?!")
            .user
            .badstanding
        {
            b
        } else {
            false
        }
    }

    /// Returns a boolean that indicates whether the user is supporter.
    pub fn is_supporter(&self) -> bool {
        if let Some(b) = self
            .data
            .as_ref()
            .expect("WTF SUCCESS?!?!?!?")
            .user
            .supporter
        {
            b
        } else {
            false
        }
    }

    /// Returns a supporter tier.
    pub fn get_supporter_tier(&self) -> usize {
        self.data
            .as_ref()
            .expect("WTF SUCCESS⁉")
            .user
            .supporter_tier as usize
    }

    /// Returns a boolean that indicates whether the user is verified.
    pub fn is_verified(&self) -> bool {
        self.data.as_ref().expect("WTF SUCCESS?⁉⁉").user.verified
    }

    /// Returns the user's banner URL.
    /// This value will only be present if user has a banner.
    pub fn get_banner_url(&self) -> Option<String> {
        if let Some(br) = self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉")
            .user
            .banner_revision
        {
            if br == 0 {
                return None;
            }
            Some(format!(
                "https://tetr.io/user-content/banners/{}.jpg?rv={}",
                self.data.as_ref().expect("WTF SUCCESS⁉⁉⁉⁉").user._id,
                br
            ))
        } else {
            None
        }
    }

    /// Retruns the user's bio.
    /// This value will only be present if user has a bio.
    pub fn get_bio(&self) -> &Option<String> {
        &self.data.as_ref().expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉").user.bio
    }

    /// Returns the user's friend count.
    pub fn get_friend_count(&self) -> u32 {
        if let Some(fc) = self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉")
            .user
            .friend_count
        {
            fc
        } else {
            0
        }
    }

    /// Returns the user's emoji of rank.
    pub fn get_rank_emoji(&self) -> String {
        let rank = self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .rank
            .as_str();
        get_rank_from_raw(rank)
    }

    /// Returns a boolean that indicates whether the user does not play the TETRA LEAGUE less than 10 times.
    pub fn is_rating(&self) -> bool {
        0. <= self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .rating
    }

    /// Returns the user's play count on TETRA LEAGUE.
    pub fn get_gamesplayed(&self) -> u32 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .gamesplayed
    }

    /// Returns the user's rating.
    pub fn get_rating(&self) -> f64 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .rating
    }

    /// Returns the amount of TETRA LEAGUE games won by this user.
    pub fn get_gameswon(&self) -> u32 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .gameswon
    }

    /// Returns the user's rank.
    pub fn get_rank(&self) -> &str {
        &self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .rank
    }

    /// Returns the user's progress bar for TETRA LEAGUE.
    pub fn get_demotion_on_next_loss(&self) -> String {
        let current_standing = self.get_standing() as f64;
        let prev_at = self
            .data
            .as_ref()
            .expect("WTF SUCCESS¿")
            .user
            .league
            .prev_at as f64;
        let next_at = self
            .data
            .as_ref()
            .expect("WTF SUCCESS¿¿")
            .user
            .league
            .next_at as f64;
        let mut prev_rank = String::from("");
        let mut next_rank = String::from("");
        if let Some(r) = &self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .prev_rank
        {
            prev_rank = get_rank_from_raw(r);
        } else if current_standing == prev_at {
            return format!(
                "👑`─────────────────|`{}\n　 ┗ Reached 0.00%",
                get_rank_from_raw("d+")
            );
        }
        if let Some(r) = &self
            .data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .next_rank
        {
            next_rank = get_rank_from_raw(r);
        } else if current_standing == 1. {
            return format!(
                "{}`|━━━━━━━━━━━━━━━━━`👑\n　　　　　**ℕ°𝟙**",
                get_rank_from_raw("u"),
            );
        }
        let progress = (current_standing - prev_at) / (next_at - prev_at);
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
        let progress_rate = progress * 100.;
        format!(
            "{}`|{}|`{}\n　 ┗ Reached {:.2}%",
            prev_rank, bar, next_rank, progress_rate
        )
    }

    /// Returns the user's position in global leaderboards.
    /// If not applicable, returns -1.
    pub fn get_standing(&self) -> i32 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉⁉")
            .user
            .league
            .standing
    }

    /// Returns the user's percentile rank.
    pub fn get_percentile_rank(&self) -> String {
        get_rank_from_raw(
            &self
                .data
                .as_ref()
                .expect("WTF SUCCESS¿¿¿")
                .user
                .league
                .percentile_rank,
        )
    }

    /// Returns the time of when this resource was cached.
    pub fn get_cached_at(&self) -> String {
        let cacherd_at = self.cache.as_ref().unwrap().cached_at / 1000;
        format!("cached at: <t:{}:R>", cacherd_at)
    }

    /// Returns the user's position in local leaderboards.
    /// If not applicable, returns -1.
    pub fn get_standing_local(&self) -> i32 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS¿¿¿¿")
            .user
            .league
            .standing_local
    }

    /// Returns the user's glicko and RD.
    pub fn get_glicko(&self) -> String {
        format!(
            "{:.3}±{:.3}",
            self.data
                .as_ref()
                .expect("WTF SUCCESS¿¿¿¿¿")
                .user
                .league
                .glicko
                .unwrap(),
            self.data
                .as_ref()
                .expect("WTF SUCCESS¿¿¿¿¿¿")
                .user
                .league
                .rd
                .unwrap()
        )
    }

    /// Retruns the amount of TETRA LEAGUE games played by this user.
    pub fn get_gamesplayed_league(&self) -> u32 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS¿¿¿¿¿¿¿")
            .user
            .league
            .gamesplayed
    }

    /// Retruns the amount of TETRA LEAGUE games won by this user.
    pub fn get_gameswon_league(&self) -> u32 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS¿¿¿¿¿¿¿¿")
            .user
            .league
            .gameswon
    }

    /// Returns the user's average APM over the last 10 games.
    pub fn get_apm(&self) -> f64 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS¿¿¿¿¿¿¿¿¿")
            .user
            .league
            .apm
            .unwrap()
    }

    /// Returns the user's average PPS over the last 10 games.
    pub fn get_pps(&self) -> f64 {
        self.data
            .as_ref()
            .expect("WTF SUCCESS¿¿¿¿¿¿¿¿¿¿")
            .user
            .league
            .pps
            .unwrap()
    }

    /// Returns the user's average VS over the last 10 games.
    pub fn get_vs(&self) -> String {
        if let Some(vs) = self
            .data
            .as_ref()
            .expect("WTF SUCCESS¿¿¿¿¿¿¿¿¿¿")
            .user
            .league
            .vs
        {
            vs.to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Returns the usert's URL of recent TETRA LEAGUE games.
    pub fn get_recent_league(&self) -> String {
        format!("https://ch.tetr.io/s/league_userrecent_{}", self.get_id())
    }
}

/// ### Cache Data Structure
/// All responses from the TETRA CHANNEL API are cached. With every response, 
/// a cache object is made available to view the status of the cache:
#[derive(Deserialize)]
pub struct Cache {
    /// Whether the cache was hit.
    /// Either `"hit"`, `"miss"`, or `"awaited"`.
    /// `"awaited"` means resource was already being requested by another client.
    pub status: String,
    /// When this resource was cached.
    pub cached_at: u64,
    /// When this resource's cache expires.
    pub cached_until: u64,
}

/// The requested data.
#[derive(Deserialize)]
pub struct UserData {
    /// The requested user.
    pub user: User,
}

/// The requested user.
#[derive(Deserialize)]
pub struct User {
    /// The user's internal ID.
    pub _id: String,
    /// The user's username.
    pub username: String,
    /// The user's role (one of "anon", "user", "bot", "mod", "admin").
    pub role: String,
    /// When the user account was created.
    /// If not set, this account was created before join dates were recorded.
    pub ts: Option<String>,
    /// If this user is a bot, the bot's operator.
    pub botmaster: Option<String>,
    /// The user's badges
    pub badges: Vec<Badges>,
    /// The user's XP in points.
    pub xp: f64,
    /// The amount of online games played by this user. If the user has chosen to hide this statistic, it will be -1.
    pub gamesplayed: i32,
    /// The amount of online games won by this user. If the user has chosen to hide this statistic, it will be -1.
    pub gameswon: i32,
    /// The amount of seconds this user spent playing, both on- and offline. If the user has chosen to hide this statistic, it will be -1.
    pub gametime: f64,
    /// The user's ISO 3166-1 country code, or null if hidden/unknown. Some vanity flags exist.
    pub country: Option<String>,
    /// Whether this user currently has a bad standing (recently banned).
    pub badstanding: Option<bool>,
    /// Whether this user is currently supporting TETR.IO <3
    pub supporter: Option<bool>, // EXCEPTION
    /// An indicator of their total amount supported, between 0 and 4 inclusive.
    pub supporter_tier: u8,
    /// Whether this user is a verified account.
    pub verified: bool,
    /// This user's current TETRA LEAGUE standing.
    pub league: League,
    /// This user's avatar ID.
    /// We can get their avatar at `https://tetr.io/user-content/avatars/{ USERID }.jpg?rv={ AVATAR_REVISION }`.
    pub avatar_revision: Option<u64>,
    /// his user's banner ID.
    /// We can get their banner at `https://tetr.io/user-content/banners/{ USERID }.jpg?rv={ BANNER_REVISION }`.
    /// Ignore this field if the user is not a supporter.
    pub banner_revision: Option<u64>,
    /// This user's "About Me" section.
    /// Ignore this field if the user is not a supporter.
    pub bio: Option<String>,
    /// The amount of players who have added this user to their friends list.
    pub friend_count: Option<u32>, // EXCEPTION
}

/// The user's badges.
#[derive(Deserialize)]
pub struct Badges {
    /// The badge's internal ID,
    /// and the filename of the badge icon (all PNGs within /res/badges/)
    pub id: String,
    /// The badge's label, shown when hovered.
    pub label: String,
    /// The badge's timestamp, if shown.
    pub ts: Option<String>,
}

/// This user's current TETRA LEAGUE standing.
#[derive(Deserialize)]
pub struct League {
    /// The amount of TETRA LEAGUE games played by this user.
    pub gamesplayed: u32,
    /// The amount of TETRA LEAGUE games won by this user.
    pub gameswon: u32,
    /// This user's TR (Tetra Rating), or -1 if less than 10 games were played.
    pub rating: f64,
    /// This user's letter rank. Z is unranked.
    pub rank: String,
    /// This user's position in global leaderboards, or -1 if not applicable.
    pub standing: i32,
    /// This user's position in local leaderboards, or -1 if not applicable.
    pub standing_local: i32,
    /// The next rank this user can achieve,
    /// if they win more games, or null if unranked (or the best rank).
    pub next_rank: Option<String>,
    /// The previous rank this user can achieve,
    /// if they lose more games, or null if unranked (or the worst rank).
    pub prev_rank: Option<String>,
    /// The position of the best player in the user's current rank, surpass them to go up a rank.
    /// -1 if unranked (or the best rank).
    pub next_at: i32,
    /// The position of the worst player in the user's current rank, dip below them to go down a rank.
    /// -1 if unranked (or the worst rank).
    pub prev_at: i32,
    /// This user's percentile position (0 is best, 1 is worst).
    pub percentile: f64,
    /// This user's percentile rank, or Z if not applicable.
    pub percentile_rank: String,
    /// This user's Glicko-2 rating.
    pub glicko: Option<f64>,
    /// This user's Glicko-2 Rating Deviation.
    /// If over 100, this user is unranked.
    pub rd: Option<f64>,
    /// This user's average APM (attack per minute) over the last 10 games.
    pub apm: Option<f64>,
    /// This user's average PPS (pieces per second) over the last 10 games.
    pub pps: Option<f64>,
    /// This user's average VS (versus score) over the last 10 games.
    pub vs: Option<f64>,
    /// Whether this user's RD is rising (has not played in the last week).
    pub decaying: bool,
}

fn get_rank_from_raw(rank: &str) -> String {
    format!(
        "<:{}>",
        match rank {
            "d" => "d_:993082933898391562",
            "d+" => "dp:993083934147616839",
            "c-" => "cm:993085497247289356",
            "c" => "c_:993085982909923450",
            "c+" => "cp:993086352893677639",
            "b-" => "bm:993086643957416006",
            "b" => "b_:993088403535036426",
            "b+" => "bp:993088785996857425",
            "a-" => "am:993089083830185985",
            "a" => "a_:993089364928245821",
            "a+" => "ap:993089619845451848",
            "s-" => "sm:993089892915609601",
            "s" => "s_:993090162835865660",
            "s+" => "sp:993090398618656768",
            "ss" => "ss:993090656820002866",
            "u" => "u_:993091250154635335",
            "x" => "x_:993091489376776232",
            _ => "z_:993091724547194931",
        }
    )
}

/// ### User Records
/// An struct describing the user's single player records.
#[derive(Deserialize)]
pub struct TetraRecords {
    /// Whether the request was successful.
    pub success: bool,
    /// The reason the request failed.
    pub error: Option<String>,
    /// Data about how this request was cached.
    pub cache: Option<Cache>,
    /// The requested data.
    pub data: Option<RecordsData>,
}

impl TetraRecords {
    /// Creates a new `TetraRecords`.
    /// Converts the given `Response` to ｀TetraRecords｀ and returns it.
    pub async fn new(response: Response) -> Self {
        response
            .json::<Self>()
            .await
            .expect("RECORDS DATA PARSE TO JSON")
    }

    /// Whether the user has 40 LINES records.
    pub fn has_40l_record(&self) -> bool {
        self.data.as_ref().unwrap().records._40l.record.is_some()
    }

    /// Returns the URL of the user's 40 LINES best record.
    pub fn get_best_40l_record(&self) -> String {
        format!("https://tetr.io/#r:{}", self.get_40l_rec().replayid)
    }

    /// Returns the `Record` from 40 LINES.
    fn get_40l_rec(&self) -> &Record {
        self.data
            .as_ref()
            .unwrap()
            .records
            ._40l
            .record
            .as_ref()
            .unwrap()
    }

    /// Returns the timestamp of the user's 40 LINES best record.
    pub fn get_40l_ts(&self) -> i64 {
        DateTime::parse_from_rfc3339(self.get_40l_rec().ts.clone().as_ref())
            .expect("PARSE FROM STRING TO DATETIME")
            .timestamp()
    }

    /// Returns the user's 40 LINES best record's time.
    pub fn get_40l_time(&self) -> String {
        // `final_time` is in milliseconds,
        // so returns the quotient of one thousand.
        let time = self.get_40l_rec().endcontext.final_time / 1000.;
        let m = (time / 60.).floor();
        let s = round_mid(time - m * 60., 3);
        format!("{}:{}", m, s)
    }

    /// Returns the user's PPS in 40 LINES best record.
    pub fn get_40l_pps(&self) -> f64 {
        round_mid(
            self.get_40l_piecesplaced() as f64 / (self.get_40l_rec().endcontext.final_time / 1000.),
            2,
        )
    }

    /// Returns the user's fromatted finesse score in 40 LINES best record.
    pub fn get_40l_finesse(&self) -> String {
        let finesse = match &self.get_40l_rec().endcontext.finesse {
            Some(f) => f,
            None => {
                return "unavailable".to_string();
            }
        };
        format!(
            "{}F ({:.2}%)",
            finesse.faults,
            round_mid(
                finesse.perfectpieces as f64 / self.get_40l_piecesplaced() as f64,
                4
            ) * 100.
        )
    }

    /// Returns the `piecesplaced` from 40 LINES.
    fn get_40l_piecesplaced(&self) -> u32 {
        self.get_40l_rec().endcontext.piecesplaced
    }

    /// Whether the user's rank is in the top 1000(40 LINES).
    pub fn is_40l_top1000(&self) -> bool {
        self.data.as_ref().unwrap().records._40l.rank.is_some()
    }

    /// Returns the user's rank in 40 LINES.
    pub fn get_40l_rank(&self) -> u32 {
        self.data.as_ref().unwrap().records._40l.rank.unwrap()
    }

    /// Whether the user has BLITZ records.
    pub fn has_blitz_record(&self) -> bool {
        self.data.as_ref().unwrap().records.blitz.record.is_some()
    }

    /// Returns the URL of the user's BLITZ best record.
    pub fn get_best_blitz_record(&self) -> String {
        format!("https://tetr.io/#r:{}", self.get_blitz_rec().replayid)
    }

    /// Returns the `Record` from BLITZ.
    fn get_blitz_rec(&self) -> &Record {
        self.data
            .as_ref()
            .unwrap()
            .records
            .blitz
            .record
            .as_ref()
            .unwrap()
    }

    /// Returns the timestamp of the user's BLITZ best record.
    pub fn get_blitz_ts(&self) -> i64 {
        DateTime::parse_from_rfc3339(self.get_blitz_rec().ts.clone().as_ref())
            .expect("PARSE FROM STRING TO DATETIME")
            .timestamp()
    }

    /// Whether the user's rank is in the top 1000(BLITZ).
    pub fn is_blitz_top1000(&self) -> bool {
        self.data.as_ref().unwrap().records.blitz.rank.is_some()
    }

    /// Returns the user's rank in BLITZ.
    pub fn get_blitz_rank(&self) -> u32 {
        self.data.as_ref().unwrap().records.blitz.rank.unwrap()
    }

    /// Returns the user's BLITZ best score(separated with commas).
    pub fn get_blitz_score(&self) -> String {
        self.get_blitz_rec().endcontext.score.separate_with_commas()
    }

    /// Returns the `piecesplaced` from BLITZ.
    fn get_blitz_piecesplaced(&self) -> u32 {
        self.get_blitz_rec().endcontext.piecesplaced
    }

    /// Returns the user's PPS in BLITZ best record.
    pub fn get_blitz_pps(&self) -> f64 {
        round_mid(
            self.get_blitz_piecesplaced() as f64
                / (self.get_blitz_rec().endcontext.final_time / 1000.),
            2,
        )
    }

    /// Returns the user's fromatted finesse score in BLITZ best record.
    pub fn get_blitz_finesse(&self) -> String {
        let finesse = match &self.get_blitz_rec().endcontext.finesse {
            Some(f) => f,
            None => {
                return "unavailable".to_string();
            }
        };
        format!(
            "{}F ({:.2}%)",
            finesse.faults,
            round_mid(
                finesse.perfectpieces as f64 / self.get_blitz_piecesplaced() as f64,
                4
            ) * 100.
        )
    }
}

/// The requested data.
#[derive(Deserialize)]
pub struct RecordsData {
    /// The requested user's ranked records.
    pub records: Records,
    /// The user's ZEN record.
    pub zen: Zen,
}

/// The requested user's ranked records.
#[derive(Deserialize)]
pub struct Records {
    /// The user's 40 LINES record.
    #[serde(rename = "40l")]
    pub _40l: FortyLines,
    /// The user's BLITZ record.
    pub blitz: Blitz,
}

/// The user's 40 LINES record
#[derive(Deserialize)]
pub struct FortyLines {
    /// The user's 40 LINES record data, or `None` if never played.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or null if not in global leaderboards.
    pub rank: Option<u32>,
}

/// The user's BLITS record
#[derive(Deserialize)]
pub struct Blitz {
    /// The user's BLITS record data, or `None` if never played.
    pub record: Option<Record>,
    /// The user's rank in global leaderboards, or null if not in global leaderboards.
    pub rank: Option<u32>,
}

/// The user's ZEN record.
#[derive(Deserialize)]
pub struct Zen {
    /// The user's level in ZEN mode.
    pub level: u32,
    /// The user's score in ZEN mode.
    pub score: u32,
}

/// ### Record Data Structure
/// The user's 40-LINES or BLITS record data, or `None` if never played.
///
/// Single player records are saved into Record objects,
/// retrievable through the Get Stream and User Records methods.
#[derive(Deserialize)]
pub struct Record {
    /// The Record's ID. This is NOT the replay ID.
    pub _id: String,
    /// The Stream this Record belongs to.
    pub stream: String,
    /// The ID of the associated replay.
    pub replayid: String,
    /// The user who set this Record.
    pub user: RecordUser,
    /// The time this record was set.
    pub ts: String,
    /// If true, this is a multiplayer replay.
    pub ismulti: Option<bool>,
    /// The state this replay finished with.
    pub endcontext: EndContext,
}

/// The user who set this Record.
#[derive(Deserialize)]
pub struct RecordUser {
    /// The user's internal ID.
    pub _id: String,
    /// The user's username.
    pub username: String,
}

/// The state this replay finished with.
///
/// **Note**: This structure is incomplete.
/// Because there are so many elements that make it up.
/// So I only provide the minimum required.
/// But I may eventually implement it perfectly:)
#[derive(Deserialize)]
pub struct EndContext {
    /// Time taken to finish(ms).
    #[serde(rename = "finalTime")]
    pub final_time: f64,
    /// The number of pieces placed.
    pub piecesplaced: u32,
    /// The finesse.
    pub finesse: Option<Finesse>,
    /// The record's score
    pub score: u32,
}

/// The finesse.
///
/// **Note**: This structure is incomplete.
/// I only provide the minimum required.
/// But I may eventually implement it perfectly:)
#[derive(Deserialize)]
pub struct Finesse {
    /// The number of finesse faults.
    pub faults: u32,
    /// The number of finesse perfects.
    pub perfectpieces: u32,
}
