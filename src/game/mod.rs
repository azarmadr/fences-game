use bevy::{
    app::Plugin,
    ecs::{
        schedule::OnEnter,
        system::{Commands, Resource},
    },
    prelude::*,
};
use fences::{Board, FencesSolver};

use crate::GameState;

use self::components::Hint;

mod components;
#[derive(Resource, Deref, DerefMut, Clone)]
pub struct Game(Board);

fn spawn_board(
    mut commands: Commands,
    window: Query<&Window>,
    // assets: Res<BoardAssets>,
    // mut cfg: ResMut<OwareCfg>,
    // mut board: ResMut<Game>,
    // balls: Query<Entity, (With<Bowl>, Without<PC>)>,
) {
    let game = Game("2#3  3".parse().unwrap());
    commands.insert_resource(game.clone());
    commands
        .spawn(SpatialBundle::default())
        .with_children(|parent| {
            game.tasks_iter().for_each(|(idx, task)| {
                parent.spawn((
                    Hint(idx),
                    Text2dBundle {
                        text: Text::from_section(
                            task.map_or_else(|| ' ', |v| v.into()).to_string(),
                            default(),
                        ),
                        ..Default::default()
                    },
                ));
            });
        });
}
pub struct FencesGamePlugin;
impl Plugin for FencesGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_board);
    }
}
