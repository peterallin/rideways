mod systems {
    pub mod alien_shooting_system;
    pub mod collision_checker_system;
    pub mod force_inside_system;
    pub mod lifetime_system;
    pub mod non_player_control_system;
    pub mod player_control_system;
    pub mod player_shooting_system;
    pub mod reap_outsiders_system;
    pub mod render_all_system;
    pub mod spawning_system;
    pub mod update_pos_system;
}

pub mod components {
    pub mod harms_aliens;
    pub mod harms_player;
    pub mod is_alien;
    pub mod is_player;
    pub mod keep_inside;
    pub mod lifetime;
    pub mod movement_kind;
    pub mod position;
    pub mod reap_when_outside;
    pub mod render_kind;
    pub mod velocity;

    pub use harms_aliens::HarmsAliens;
    pub use harms_player::HarmsPlayer;
    pub use is_alien::IsAlien;
    pub use is_player::IsPlayer;
    pub use keep_inside::KeepInside;
    pub use lifetime::Lifetime;
    pub use movement_kind::MovementKind;
    pub use position::Position;
    pub use reap_when_outside::ReapWhenOutside;
    pub use render_kind::RenderKind;
    pub use velocity::Velocity;
}

mod setup;
pub use setup::setup;
