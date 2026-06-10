use engine_core::SystemContext;
use engine_input::InputState;
use game::{ActivePlayMode, GameplayInput, LocalPlayerId, PlayMode, PlayerInputs};

pub struct PendingWinitInput(pub InputState);

pub fn sync_local_input_system(ctx: &mut SystemContext<'_>) {
    let pending = ctx.resources.get::<PendingWinitInput>().expect("pending input");
    let player_id = ctx
        .resources
        .get::<LocalPlayerId>()
        .and_then(|local| local.id)
        .unwrap_or(0);

    let survival = ctx
        .resources
        .get::<ActivePlayMode>()
        .is_none_or(|mode| mode.0 == PlayMode::Survival);

    let gameplay = GameplayInput {
        move_axis: pending.0.move_axis,
        look_delta: pending.0.look_delta,
        vertical_axis: pending.0.vertical_axis(),
        sprint: pending.0.sprint,
        jump: pending.0.jump || (survival && pending.0.ascend),
        interact: pending.0.interact,
        break_block: pending.0.break_block,
        place_block: pending.0.place_block,
    };

    if let Some(inputs) = ctx.resources.get_mut::<PlayerInputs>() {
        inputs.set(player_id, gameplay);
    }
}
