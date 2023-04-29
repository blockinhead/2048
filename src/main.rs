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

#[derive(Component)]
struct Points {
    value: u32,
}

#[derive(Component)]
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

fn board_shift(input: Res<Input<KeyCode>>) {
    let shift_direction = input.get_just_pressed().find_map(
        |key_kode| BoardShift::try_from(key_kode).ok()
    );

    match shift_direction {
        None => {}
        Some(BoardShift::Down) => { dbg!("down"); }
        Some(BoardShift::Right) => { dbg!("right"); }
        Some(BoardShift::Up) => { dbg!("up"); }
        Some(BoardShift::Left) => {dbg!("left"); }
    }
}
