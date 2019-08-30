mod movement_kind;
mod non_player_control_system;
mod player_control_system;
mod position;
mod render_all_system;
mod render_kind;
mod setup;
mod update_pos_system;
mod velocity;

use movement_kind::MovementKind;
pub use position::Position;
pub use render_kind::RenderKind;
pub use setup::setup;
use velocity::Velocity;
