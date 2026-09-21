//! Day clock from the server tick. 20 TPS, 120 seconds, unchanged.

pub const DAY_TICKS: u32 = 2400;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Clone, Copy, Debug)]
pub struct Phase {
    pub day: u32,
    /// 0 at dawn, 0.25 noon, 0.5 dusk, 0.75 midnight.
    pub t: f32,
    pub night: bool,
    pub blood: bool,
    pub syzygy: bool,
    pub season: Season,
}

pub fn at_tick(tick: u32) -> Phase {
    at_secs(tick as f32 / 20.0)
}

pub fn at_secs(secs: f32) -> Phase {
    let day = (secs / 120.0).floor().max(0.0) as u32;
    let t = (secs / 120.0).rem_euclid(1.0);
    let night = t >= 0.5;
    let blood = night && day % 4 == 3;
    let syzygy = !night && day % 4 == 0;
    let season = match (day / 4) % 4 {
        0 => Season::Spring,
        1 => Season::Summer,
        2 => Season::Autumn,
        _ => Season::Winter,
    };
    Phase {
        day,
        t,
        night,
        blood,
        syzygy,
        season,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_stays_two_minutes() {
        assert_eq!(DAY_TICKS, 2400);
        let dawn = at_tick(0);
        assert!(!dawn.night);
        assert!(at_tick(1200).night);
        assert!(!at_tick(2400).night);
    }

    #[test]
    fn blood_moon_is_the_fourth_night_only() {
        assert!(!at_tick(1200).blood);
        assert!(at_tick(2400 * 3 + 1200).blood);
        assert!(!at_tick(2400 * 4).blood);
    }

    #[test]
    fn syzygy_is_dawn_of_every_fourth_day() {
        assert!(at_tick(0).syzygy);
        assert!(!at_tick(1200).syzygy);
        assert!(at_tick(2400 * 4 + 10).syzygy);
    }
}
