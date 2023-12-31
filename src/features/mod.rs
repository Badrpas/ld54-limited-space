pub mod brains;
pub mod damage;
pub mod end;
pub mod fall;
pub mod follow;
#[cfg(not(target_arch = "wasm32"))]
pub mod fps_limit;
pub mod hp;
pub mod hp_ui;
pub mod player;
pub mod road;
pub mod setup;
pub mod team;
pub mod third_party;
pub mod upgrade;
use bevy::prelude::*;

pub struct FeaturesPlugin;

impl Plugin for FeaturesPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(third_party::ThirdPartyPlugin);
        #[cfg(not(target_arch = "wasm32"))]
        app.add_plugins(fps_limit::FpsLimitPlugin);

        app.add_plugins(setup::SetupPlugin);
        app.add_plugins(player::PlayerPlugin);
        app.add_plugins(follow::FollowPlugin);
        app.add_plugins(road::RoadPlugin);
        app.add_plugins(fall::FallPlugin);
        app.add_plugins(hp::HpPlugin);
        app.add_plugins(team::TeamPlugin);
        app.add_plugins(brains::BrainsPlugin);
        app.add_plugins(end::EndPlugin);
        app.add_plugins(hp_ui::HpUiPlugin);
        app.add_plugins(damage::DamagePlugin);
        app.add_plugins(upgrade::UpgradePlugin);
    }
}
