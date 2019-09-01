mod movement_kind;
mod non_player_control_system;
mod player_control_system;
mod player_shooting_system;
mod position;
mod reap_outsiders_system;
mod reap_when_outside;
mod render_all_system;
mod render_kind;
mod setup;
mod update_pos_system;
mod velocity;

use movement_kind::MovementKind;
pub use position::Position;
use reap_when_outside::ReapWhenOutside;
pub use render_kind::RenderKind;
pub use setup::setup;
use velocity::Velocity;
