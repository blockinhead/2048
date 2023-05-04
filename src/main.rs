use std::cmp::Ordering;
use bevy::prelude::*;
use itertools::Itertools;
use rand::prelude::*;

const BOARD_COLOR: Color = Color::rgb(0.7, 0.7, 0.8);
const TILE_PLACEHOLDER_COLOR: Color = Color::rgb(0.75, 0.75, 0.9);
const TILE_COLOR: Color = Color::WHITE;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Materials>()
        .init_resource::<FontSpec>()
        .add_startup_system(setup)
        .add_startup_system(spawn_board)
        .add_startup_system(spawn_tiles.in_base_set(StartupSet::PostStartup))
        .add_system(render_tile_points)
        .add_system(board_shift)
        .add_system(render_tiles)
        .run();
}

// part 2

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

const TILE_SIZE: f32 = 40.0;
const TILE_SPACER: f32 = 10.0;

#[derive(Component)]
struct Board {
    size: u8,
    physical_size: f32,
}


impl Board {
    fn new(size: u8) -> Self {
        let physical_size = f32::from(size) * TILE_SIZE + f32::from(size + 1) * TILE_SPACER;
        Board {size, physical_size}
    }

    fn cell_position_to_physical(&self, pos: u8) -> f32 {
        let offset = -self.physical_size / 2.0 + 0.5 * TILE_SIZE;
        offset + f32::from(pos) * TILE_SIZE + f32::from(pos + 1) * TILE_SPACER
    }

}

fn spawn_board(mut commands: Commands) {
    let board = Board::new(4);

    commands.spawn(
        SpriteBundle{
            sprite: Sprite {
                color: BOARD_COLOR,
                custom_size: Some(Vec2::new(board.physical_size, board.physical_size)),
                ..default()
            },
            ..default()
        },
    ).with_children(|builder| {
        for tile in (0..board.size).cartesian_product(0..board.size) {
            // dbg!(tile);
            builder.spawn(SpriteBundle {
                sprite: Sprite {
                    color: TILE_PLACEHOLDER_COLOR,
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    board.cell_position_to_physical(tile.0),
                    board.cell_position_to_physical(tile.1),
                    1.0),
                ..default()
            });
        }
    })
        .insert(board);
}

// part 3

// not used, but presents in the reference
#[allow(dead_code)]
#[derive(Resource)]
struct Materials {
    board: Handle<ColorMaterial>,
    tile_placeholder: Handle<ColorMaterial>,
}

impl FromWorld for Materials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Materials {
            board: materials.add(BOARD_COLOR.into()),
            tile_placeholder: materials.add(TILE_PLACEHOLDER_COLOR.into()),
        }
    }
}

// part 6

#[derive(Component, Debug)]
struct Points {
    value: u32,
}

#[derive(Component, Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Component)]
struct TileText;


fn spawn_tiles(
    mut commands: Commands,
    query_board: Query<&Board>,
    font_spec: Res<FontSpec>,
) {
    let board = query_board.get_single().expect("only one board expected");

    let mut rng = rand::thread_rng();
    let starting_tiles = (0..board.size).cartesian_product(0..board.size).choose_multiple(&mut rng, 2);
    for (x, y) in starting_tiles.iter() {
        let pos = Position {x: *x, y: *y};
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: TILE_COLOR,
                custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(
                board.cell_position_to_physical(pos.x),
                board.cell_position_to_physical(pos.y),
                2.0
            ),
            ..default()
        })
            .with_children(|child_builder| {
                child_builder.spawn(Text2dBundle{
                    text: Text::from_section(
                        "2",
                        TextStyle {
                            font: font_spec.family.clone(),
                            font_size: 40.0,
                            color: Color::BLACK,
                            })
                        .with_alignment(TextAlignment::Center),
                    transform: Transform::from_xyz(0.0, 0.0, 2.0),
                    ..default()
                }).insert(TileText);
            })
            .insert(Points {value: 2})
            .insert(pos);
    }
}

// part 7

#[derive(Resource)]
struct FontSpec {
    family: Handle<Font>,
}

impl FromWorld for FontSpec {
    fn from_world(world: &mut World) -> Self {
        let asset_sever = world.get_resource_mut::<AssetServer>().unwrap();
        FontSpec {
            family: asset_sever.load("fonts/FiraSans-Bold.ttf"),
        }
    }
}

// part 8

fn render_tile_points(
    mut texts: Query<&mut Text, With<TileText>>,
    tiles: Query<(&Points, &Children)>,
) {
    for (point, children) in tiles.iter() {
        if let Some(entry) = children.first() {
            let mut text = texts.get_mut(*entry).expect("Text expected to exist");
            let mut text_section = text.sections.first_mut().expect("first sections as mut expected");
            text_section.value = point.value.to_string()
        }
    }
}

// part 9

enum BoardShift {
    Left,
    Right,
    Up,
    Down,
}

impl TryFrom<&KeyCode> for BoardShift {
    type Error = &'static str;

    fn try_from(value: &KeyCode) -> Result<Self, Self::Error> {
        match value {
            KeyCode::Left => Ok(BoardShift::Left),
            KeyCode::Up => Ok(BoardShift::Up),
            KeyCode::Right => Ok(BoardShift::Right),
            KeyCode::Down => Ok(BoardShift::Down),
            _ => Err("not a valid key for board shift"),
        }
    }
}

fn board_shift(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut tiles: Query<(Entity, &mut Position, &mut Points)>,
    query_board: Query<&Board>,
) {

    let shift_direction = input.get_just_pressed().find_map(
        |key_kode| BoardShift::try_from(key_kode).ok()
    );

    if shift_direction.is_none() { return; }
    let board_shift = shift_direction.expect("that cannot be none");

    let board = query_board.get_single().expect("board is expected");

    let mut it = tiles
        .iter_mut()
        .sorted_by(|a, b| board_shift.sort(&a.1, &b.1))
        .peekable();

    let mut column: u8 = 0;  // when sliding left, the column of the first sorted tile in any case will be 0

    while let Some(mut tile) = it.next() {
        board_shift.set_column_position(board.size, &mut tile.1, column);

        let tile_next = it.peek();
        if tile_next.is_none() { continue; }
        let tile_next = tile_next.expect("tile_next is not none");

        if board_shift.get_row_position(&tile.1) != board_shift.get_row_position(&tile_next.1) { column = 0; }  // different rows, don't merge
        else if tile.2.value != tile_next.2.value { column = column + 1; } // different values don't merge
        else {
            let real_next_tile = it.next().expect("definitely there is one more"); // one was peeked, so we can take it with next
            tile.2.value = tile.2.value + real_next_tile.2.value;
            commands.entity(real_next_tile.0).despawn_recursive();

            if let Some(future) = it.peek() {
                if board_shift.get_row_position(&tile.1) != board_shift.get_row_position(&future.1) {
                    column = 0; // next tile on a next row
                }
                else { column = column + 1; }
            }
        }
    }
}


// part 12

fn render_tiles(
    mut tiles: Query<(&mut Transform, &Position, Changed<Position>)>,
    query_board: Query<&Board>,
) {
    let board = query_board.get_single().expect("board is expected");
    for (mut transform, pos, pos_changed) in tiles.iter_mut() {
        if pos_changed {
            transform.translation.x = board.cell_position_to_physical(pos.x);
            transform.translation.y = board.cell_position_to_physical(pos.y);
        }
    }
}

// part 13

impl BoardShift {
    fn sort(&self, a: &Position, b: &Position) -> Ordering {
        match self {
            BoardShift::Left => {
                match Ord::cmp(&a.y, &b.y) {
                    Ordering::Equal => { Ord::cmp(&a.x, &b.x) }
                    o => o,
                }
            }
            BoardShift::Right => {
                match Ord::cmp(&b.y, &a.y) {
                    Ordering::Equal => { Ord::cmp(&b.x, &a.x) }
                    o => o,
                }
            }
            BoardShift::Up => {
                match Ord::cmp(&b.x, &a.x) {
                    Ordering::Equal => { Ord::cmp(&b.y, &a.y) }
                    o => o,
                }
            }
            BoardShift::Down => {
                match Ord::cmp(&a.x, &b.x) {
                    Ordering::Equal => { Ord::cmp(&a.y, &b.y) }
                    o => o,
                }
            }
        }
    }

    fn set_column_position(&self, board_size: u8, position: &mut Mut<Position>, index: u8) {
        match self {
            BoardShift::Left => { position.x = index; }
            BoardShift::Right => { position.x = board_size - 1 -index; }
            BoardShift::Up => { position.y = board_size - 1 - index; }
            BoardShift::Down => { position.y = index; }
        }
    }

    fn get_row_position(&self, position: &Position) -> u8 {
        match self {
            BoardShift::Left | BoardShift::Right => position.y,
            BoardShift::Up | BoardShift::Down => position.x,
        }
    }
}
