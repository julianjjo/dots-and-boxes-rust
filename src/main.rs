
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use bevy_mod_picking::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

const SIZE_LINE: f32 = 60.0;
const GRID_SIZE: f32 = 6.0;
const WIDTH: f32 = 6.0;
const NOT_LINE: &i8 = &4;

// Resources
#[derive(Resource)]
struct ActualPlayer {
    player: i8,
}

#[derive(Resource)]
struct Board {
    grid: Vec<Vec<Vec<i8>>>,
}

#[derive(Resource)]
struct Scores {
    player1: usize,
    player2: usize,
}

// Components
#[derive(Component)]
struct Line {
    clicked: bool,
}

#[derive(Component)]
struct Position {
    row: usize,
    column: usize,
    index_line: usize,
}

#[derive(Component)]
struct Square {
    player: i8,
    row: usize,
    column: usize,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
        ))
        .add_plugins(WorldInspectorPlugin::new())
        .insert_resource(ActualPlayer { player: 1 })
        .insert_resource(Board {
            grid: vec![
                vec![
                    vec![0, 0, 0, 0],
                    vec![4, 0, 0, 0],
                    vec![4, 0, 0, 0],
                    vec![4, 0, 0, 0],
                    vec![4, 0, 0, 0],
                    vec![4, 0, 0, 0],
                ],
                vec![
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 0, 4],
                ],
                vec![
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 0, 4],
                ],
                vec![
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 0, 4],
                ],
                vec![
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 0, 4],
                ],
                vec![
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 4, 4],
                    vec![0, 0, 0, 4],
                ],
            ],
        })
        .insert_resource(Scores {
            player1: 0,
            player2: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (check_click, check_squares))
        .run();
}

// Systems
fn check_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut events: EventReader<Pointer<Click>>,
    query: Query<&Handle<ColorMaterial>>,
    mut lines: Query<(&mut Line, &Position)>,
    mut actual_player: ResMut<ActualPlayer>,
    mut board: ResMut<Board>,
) {
    for event in events.read() {
        if let Ok((mut line, position)) = lines.get_mut(event.target) {
            if !line.clicked {
                line.clicked = true;
                board.grid[position.row][position.column][position.index_line] =
                    actual_player.player;

                let color = if actual_player.player == 1 {
                    Color::RED
                } else {
                    Color::BLUE
                };

                if let Ok(material_handle) = query.get(event.target) {
                    if let Some(material) = materials.get_mut(material_handle) {
                        material.color = color;
                    }
                }

                // Check if a square is completed and spawn a colored square entity
                if check_square_completed(&board.grid, position.row, position.column) {
                    commands.spawn((
                        MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                                SIZE_LINE - WIDTH,
                                SIZE_LINE - WIDTH,
                            ))),
                            material: materials.add(color),
                            transform: Transform::from_translation(Vec3::new(
                                (position.column as f32) * SIZE_LINE - (GRID_SIZE * SIZE_LINE) / 2.0
                                    + SIZE_LINE / 2.0
                                    - WIDTH / 2.0,
                                (position.row as f32) * SIZE_LINE - (GRID_SIZE * SIZE_LINE) / 2.0
                                    + SIZE_LINE / 2.0
                                    - WIDTH / 2.0,
                                -0.1, // Place slightly below lines
                            )),
                            ..default()
                        },
                        Square {
                            player: actual_player.player,
                            row: position.row, // Set row and column for the square
                            column: position.column,
                        },
                    ));
                }

                actual_player.player = if actual_player.player == 1 { 2 } else { 1 };
            }
        }
    }
}

fn check_squares(
    mut commands: Commands,
    mut scores: ResMut<Scores>,
    square_query: Query<(Entity, &Square)>,
    line_query: Query<&Line>,
) {
    let mut game_over = true;
    for (entity, square) in square_query.iter() {
        // Check if any line around the square is not clicked
        let mut square_complete = true;
        for i in 0..4 {
            let row = square.row + (i / 2);
            let column = square.column + (i % 2);
            let index_line = if i < 2 { 0 } else { 1 };

            if let Ok(line) = line_query.get(Entity::from_raw(
                ((row * GRID_SIZE as usize + column) * 4 + index_line) as u32,
            )) {
                if !line.clicked {
                    square_complete = false;
                    break;
                }
            } else {
                square_complete = false;
                break;
            }
        }

        if square_complete {
            if square.player == 1 {
                scores.player1 += 1;
            } else {
                scores.player2 += 1;
            }
            commands.entity(entity).despawn();
        } else {
            game_over = false;
        }
    }

    if game_over {
        // Display winner or declare a draw
        let winner_text = if scores.player1 > scores.player2 {
            "Player 1 Wins!"
        } else if scores.player2 > scores.player1 {
            "Player 2 Wins!"
        } else {
            "It's a Draw!"
        };
        println!("{}", winner_text);
    }
}

fn check_square_completed(grid: &Vec<Vec<Vec<i8>>>, row: usize, column: usize) -> bool {
    // Check if all four lines around the square are claimed by the same player
    let player = grid[row][column][0]; // Assuming lines are consistent
    if player == 0 || player == *NOT_LINE {
        return false;
    }

    for i in 0..4 {
        let r = row + (i / 2);
        let c = column + (i % 2);
        let index_line = if i < 2 { 0 } else { 1 };

        if grid[r][c][index_line] != player {
            return false;
        }
    }

    true
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: ResMut<Board>,
) {
    commands.spawn(Camera2dBundle::default());

    let offset_x = (GRID_SIZE * SIZE_LINE) / 2.0;
    let offset_y = (GRID_SIZE * SIZE_LINE) / 2.0;

    // Spawn grid lines
    for (row_index, row) in board.grid.iter().enumerate() {
        for (column_index, lines) in row.iter().enumerate() {
            for (line_index, &line_type) in lines.iter().enumerate() {
                // Only spawn lines if line_type is not NOT_LINE
                if line_type == *NOT_LINE {
                    continue;
                }

                let line_length = if line_index == 0 || line_index == 2 { WIDTH } else { SIZE_LINE };
                let line_width = if line_index == 0 || line_index == 2 { SIZE_LINE } else { WIDTH };
                let x = (column_index as f32) * SIZE_LINE - offset_x;
                let y = (row_index as f32) * SIZE_LINE - offset_y;

                let translation = match line_index {
                    0 => Vec3::new(x - (SIZE_LINE / 2.0), y, 0.0), // Left vertical line
                    1 => Vec3::new(x, y + (SIZE_LINE / 2.0), 0.0), // Top horizontal line
                    2 => Vec3::new(x + (SIZE_LINE / 2.0), y, 0.0), // Right vertical line
                    3 => Vec3::new(x, y - (SIZE_LINE / 2.0), 0.0), // Bottom horizontal line
                    _ => unreachable!(),
                };

                spawn_line_entity(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    translation,
                    line_length,
                    line_width
                );
            }
        }
    }
}

// Helper function to spawn a line entity with the specified properties
fn spawn_line_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
    line_length: f32,
    line_width: f32
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(
                line_length,
                line_width,
            ))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        },
        Line { clicked: false },
        Position {
            row: 0, // Set dummy values for border lines
            column: 0,
            index_line: 0,
        },
        PickableBundle::default(),
    ));
}