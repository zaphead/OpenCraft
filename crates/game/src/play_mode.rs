#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayMode {
    Survival,
    Spectator,
}

impl PlayMode {
    pub fn label(self) -> &'static str {
        match self {
            Self::Survival => "Survival",
            Self::Spectator => "Spectator",
        }
    }
}

/// Local client play mode. Absent on server (player systems always run).
#[derive(Debug, Clone, Copy)]
pub struct ActivePlayMode(pub PlayMode);

impl Default for ActivePlayMode {
    fn default() -> Self {
        Self(PlayMode::Survival)
    }
}

impl ActivePlayMode {
    pub fn allows_player_sim(self) -> bool {
        self.0 == PlayMode::Survival
    }
}
