use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_mod_picking::prelude::*;
use bevy::app::AppExit;
use bevy::input::ButtonInput;

// Definir un componente marcador para entidades propias del juego (estado Playing)
#[derive(Component)]
struct PlayingElement;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Playing,
}

const SIZE_LINE: f32 = 100.0;
const GRID_SIZE: f32 = 6.0;
const WIDTH: f32 = 10.0;
const NOT_LINE: &i8 = &4;

// Recursos
#[derive(Resource)]
struct ActualPlayer {
    player: i8,
}

#[derive(Resource)]
struct Board {
    grid: Vec<Vec<Vec<Vec<i8>>>>,
}

// Componentes
#[derive(Component)]
struct Line {
    clicked: bool,
}

#[derive(Component)]
struct ScoreText;

#[derive(Component, Debug)]
struct Position {
    row: usize,
    column: usize,
    index_line: usize,
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct MainMenuButton;

#[derive(Component)]
struct ExitButton;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(low_latency_window_plugin()),
            DefaultPickingPlugins
                .build()
                .disable::<DefaultHighlightingPlugin>(),
        ))
        .init_state::<GameState>()
        .insert_resource(ActualPlayer { player: 1 })
        .insert_resource(Board {
            grid: vec![
                vec![
                    vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
            ],
        })
        // Al entrar al menú, primero eliminamos las entidades del juego
        .add_systems(OnEnter(GameState::MainMenu), cleanup_playing)
        // Luego configuramos el menú
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(Startup, setup_camera)
        .add_systems(
            Update,
            (
                main_menu_button_system.run_if(in_state(GameState::MainMenu)),
                exit_button_system.run_if(in_state(GameState::MainMenu)),
                check_click.run_if(in_state(GameState::Playing)),
                score_draw.run_if(in_state(GameState::Playing)),
                some_condition_to_go_back_to_menu.run_if(in_state(GameState::Playing)),
            ),
        )
        .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(GameState::Playing), setup)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Sistema para limpiar todas las entidades propias del juego (estado Playing)
fn cleanup_playing(mut commands: Commands, query: Query<Entity, With<PlayingElement>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

// Sistema para manejar los clics en las líneas durante el juego
fn check_click(
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
                board.grid[position.row][position.column][1][position.index_line] =
                    actual_player.player;

                if position.column > 0
                    && util::is_valid_position(
                        position.column - 1,
                        board.grid[position.row].clone(),
                    )
                    && position.index_line == 0
                {
                    board.grid[position.row][position.column - 1][1][2] = actual_player.player;
                }

                if util::is_valid_position(position.column + 1, board.grid[position.row].clone())
                    && position.index_line == 2
                {
                    board.grid[position.row][position.column + 1][1][0] = actual_player.player;
                }

                if util::is_valid_position(position.row + 1, board.grid.clone())
                    && position.index_line == 1
                {
                    board.grid[position.row + 1][position.column][1][3] = actual_player.player;
                }

                if position.row > 0
                    && util::is_valid_position(position.row - 1, board.grid.clone())
                    && position.index_line == 3
                {
                    board.grid[position.row - 1][position.column][1][1] = actual_player.player;
                }

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

                actual_player.player = if actual_player.player == 1 { 2 } else { 1 };
            }
        }
    }
}

// Sistema para actualizar la puntuación en pantalla
fn score_draw(mut query: Query<&mut Text, With<ScoreText>>, board: Res<Board>) {
    let mut player_1 = 0;
    let mut player_2 = 0;
    util::calulate_score(board.grid.clone(), &mut player_1, &mut player_2);
    if (player_1 > 0) || (player_2 > 0) {
        for mut text in &mut query {
            let score = format!("Score: Player 1: {} - Player 2: {}", player_1, player_2);
            text.sections[0].value = score;
        }
    }
}

// Sistema para configurar el menú principal
fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Nodo raíz de la interfaz del menú
    commands.spawn((
        NodeBundle {
            style: Style {
                height: Val::Percent(100.0),
                width: Val::Percent(100.0),
                // Centrar los elementos
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        },
        MainMenu,
    ))
    .with_children(|parent| {
        // Título del juego
        parent.spawn(TextBundle {
            text: Text::from_section(
                "Dots and Boxes",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 60.0,
                    color: Color::WHITE,
                },
            ),
            ..default()
        });

        // Espaciador
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(50.0),
                ..default()
            },
            ..default()
        });

        // Botón "Nueva partida"
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                MainMenuButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Nueva partida",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });
            });

        // Espaciador entre botones
        parent.spawn(NodeBundle {
            style: Style {
                width: Val::Auto,
                height: Val::Px(20.0),
                ..default()
            },
            ..default()
        });

        // Botón "Salir"
        parent
            .spawn((
                ButtonBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(65.0),
                        margin: UiRect::all(Val::Auto),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::DARK_GRAY),
                    ..default()
                },
                ExitButton,
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle {
                    text: Text::from_section(
                        "Salir",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    ),
                    ..default()
                });
            });
    });
}

// Sistema para gestionar la interacción con el botón "Nueva partida"
fn main_menu_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<MainMenuButton>),
    >,
    mut state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::GRAY);
                state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
        }
    }
}

// Sistema para gestionar la interacción con el botón "Salir"
fn exit_button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>, With<ExitButton>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::GRAY);
                // Envía el evento para salir de la aplicación
                app_exit_events.send(AppExit);
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::GRAY);
            }
            Interaction::None => {
                *color = BackgroundColor(Color::DARK_GRAY);
            }
        }
    }
}

// Sistema para limpiar el menú principal al salir de él
fn cleanup_main_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in &query {
        commands.entity(entity).despawn_recursive();
    }
}

// Sistema para volver al menú principal (por ejemplo, al presionar Escape durante el juego)
fn some_condition_to_go_back_to_menu(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        state.set(GameState::MainMenu);
    }
}

// Configuración del juego (spawn de líneas y UI de puntuación)
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    board: ResMut<Board>,
) {
    let offset_x = (GRID_SIZE * SIZE_LINE) / 2.0;
    let offset_y = (GRID_SIZE * SIZE_LINE) / 2.0;

    // Creación de las líneas de la cuadrícula
    for (row_index, row) in board.grid.iter().enumerate() {
        for (column_index, lines) in row.iter().enumerate() {
            for (line_index, &line_type) in lines[0].iter().enumerate() {
                // Solo se crean líneas si no es NOT_LINE
                if line_type == *NOT_LINE {
                    continue;
                }

                let line_length = if line_index == 0 || line_index == 2 {
                    WIDTH
                } else {
                    SIZE_LINE
                };
                let line_width = if line_index == 0 || line_index == 2 {
                    SIZE_LINE
                } else {
                    WIDTH
                };
                let x = (column_index as f32) * SIZE_LINE - offset_x;
                let y = (row_index as f32) * SIZE_LINE - offset_y;

                let translation = match line_index {
                    0 => Vec3::new(x - (SIZE_LINE / 2.0), y, 0.0), // Línea vertical izquierda
                    1 => Vec3::new(x, y + (SIZE_LINE / 2.0), 0.0), // Línea horizontal superior
                    2 => Vec3::new(x + (SIZE_LINE / 2.0), y, 0.0), // Línea vertical derecha
                    3 => Vec3::new(x, y - (SIZE_LINE / 2.0), 0.0), // Línea horizontal inferior
                    _ => unreachable!(),
                };

                spawn_line_entity(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    translation,
                    line_length,
                    line_width,
                    Position {
                        row: row_index,
                        column: column_index,
                        index_line: line_index,
                    },
                );
            }
        }
    }

    let score = format!("Score: Player 1: {} - Player 2: {}", 0, 0);

    commands.spawn((
        TextBundle::from_section(
            score,
            TextStyle {
                color: Color::WHITE,
                font_size: 30.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(30.0),
            left: Val::Px(15.0),
            ..default()
        }),
        ScoreText,
        PlayingElement, // Marca como entidad del juego
    ));
}

// Función auxiliar para crear una entidad de línea
fn spawn_line_entity(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    translation: Vec3,
    line_length: f32,
    line_width: f32,
    position: Position,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(line_length, line_width))),
            material: materials.add(Color::WHITE),
            transform: Transform {
                translation,
                ..default()
            },
            ..default()
        },
        Line { clicked: false },
        position,
        PickableBundle::default(),
        PlayingElement, // Marca como entidad del juego
    ));
}

mod util {
    use std::collections::HashSet;

    // Calcula la puntuación a partir de la cuadrícula
    pub fn calulate_score(grid: Vec<Vec<Vec<Vec<i8>>>>, player_1: &mut i32, player_2: &mut i32) {
        for row in grid.iter() {
            for column in row.iter() {
                let amount_player_lines = count_unique_numbers(column[1].clone());
                if amount_player_lines == 1 && column[1][0] != 0 {
                    if column[1][0] == 1 {
                        *player_1 += 1;
                    } else {
                        *player_2 += 1;
                    }
                }
            }
        }
    }

    // Verifica que el índice sea válido para el vector
    pub fn is_valid_position<T>(index: usize, vector: Vec<T>) -> bool {
        index < vector.len()
    }

    // Cuenta la cantidad de números únicos en un vector
    pub fn count_unique_numbers(vector: Vec<i8>) -> usize {
        let mut unique_set = HashSet::new();
        for num in vector {
            unique_set.insert(num);
        }
        unique_set.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_test() {
        let mut world = World::new();
        let entity = world
            .spawn(Position {
                row: 1,
                column: 1,
                index_line: 1,
            })
            .id();
        let entity_ref = world.get_entity(entity).unwrap();
        let position = entity_ref.get::<Position>().unwrap();
        assert_eq!(position.row, 1);
        assert_eq!(position.column, 1);
        assert_eq!(position.index_line, 1);
    }

    #[test]
    fn is_valid_position_board_test() {
        let mut world = World::new();
        world.insert_resource(Board {
            grid: vec![
                vec![
                    vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
            ],
        });
        assert_eq!(
            util::is_valid_position(1, world.get_resource::<Board>().unwrap().grid[0].clone()),
            true
        );
    }

    #[test]
    fn calulate_score_test() {
        let mut world = World::new();
        world.insert_resource(Board {
            grid: vec![
                vec![
                    vec![vec![0, 0, 0, 0], vec![1, 1, 1, 1]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                    vec![vec![4, 0, 0, 0], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
                vec![
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 4, 4], vec![0, 0, 0, 0]],
                    vec![vec![0, 0, 0, 4], vec![0, 0, 0, 0]],
                ],
            ],
        });
        let mut player_1 = 0;
        let mut player_2 = 0;
        util::calulate_score(
            world.get_resource_mut::<Board>().unwrap().grid.clone(),
            &mut player_1,
            &mut player_2,
        );
        assert_eq!(player_1, 1);
        assert_eq!(player_2, 0);
    }
}
