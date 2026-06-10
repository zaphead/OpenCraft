use std::collections::HashMap;

/// Execution stages run in declaration order each tick.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Stage {
    PreUpdate,
    Update,
    Physics,
    PostUpdate,
    Extract,
    Render,
}

impl Stage {
    pub const ORDER: &'static [Stage] = &[
        Stage::PreUpdate,
        Stage::Update,
        Stage::Physics,
        Stage::PostUpdate,
        Stage::Extract,
        Stage::Render,
    ];
}

pub type SystemFn = fn(&mut crate::app::SystemContext<'_>);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SystemId(u32);

#[derive(Clone)]
pub struct SystemEntry {
    pub id: SystemId,
    pub function: SystemFn,
    pub after: Vec<SystemId>,
}

pub struct Schedule {
    next_id: u32,
    systems: HashMap<Stage, Vec<SystemEntry>>,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            next_id: 0,
            systems: HashMap::new(),
        }
    }
}

impl Schedule {
    pub fn add_system(&mut self, stage: Stage, function: SystemFn) -> SystemId {
        let id = SystemId(self.next_id);
        self.next_id += 1;
        self.systems
            .entry(stage)
            .or_default()
            .push(SystemEntry {
                id,
                function,
                after: Vec::new(),
            });
        id
    }

    pub fn add_system_after(
        &mut self,
        stage: Stage,
        function: SystemFn,
        after: SystemId,
    ) -> SystemId {
        let id = SystemId(self.next_id);
        self.next_id += 1;
        self.systems
            .entry(stage)
            .or_default()
            .push(SystemEntry {
                id,
                function,
                after: vec![after],
            });
        id
    }

    pub fn systems_for(&self, stage: Stage) -> &[SystemEntry] {
        self.systems.get(&stage).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn sort_stage(&self, stage: Stage) -> Vec<SystemEntry> {
        let systems = self.systems_for(stage).to_vec();
        if systems.is_empty() {
            return systems;
        }

        let mut sorted = Vec::with_capacity(systems.len());
        let mut remaining = systems;

        while !remaining.is_empty() {
            let before = sorted.len();
            remaining.retain(|entry| {
                let ready = entry
                    .after
                    .iter()
                    .all(|dep| sorted.iter().any(|done: &SystemEntry| done.id == *dep));
                if ready {
                    sorted.push(entry.clone());
                    false
                } else {
                    true
                }
            });
            if sorted.len() == before {
                panic!("cyclic system dependency in stage {stage:?}");
            }
        }

        sorted
    }
}
