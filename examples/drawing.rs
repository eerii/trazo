use macro_rules_attribute::derive;
use trazo::prelude::*;

const CANVAS_COLOR: Srgba = css::IVORY;
const CANVAS_LAYER: f32 = -100.;

const COLORS: [Srgba; 4] = [
    css::ROYAL_BLUE,
    css::MEDIUM_SEA_GREEN,
    css::GOLD,
    css::TOMATO,
];

fn main() {
    App::new()
        .add_plugins((GamePlugin, MeshPickingPlugin))
        .add_systems(OnEnter(GameState::Play), init)
        .add_systems(Update, (on_resize, draw_curve, on_keyboard))
        .run();
}

// Resources
// ---

#[derive(Resource, Default)]
struct DrawData {
    current: Option<Entity>,
    selected_color: usize,
    line_start: Vec2,
}

// Components
// ---

#[derive(Component)]
struct Canvas;

// TODO: Create custom curve type that contains the points and can grow
#[derive(Component, Default)]
struct Drawing {
    points: Vec<Vec2>,
    length: usize,
    curve: Option<SampleAutoCurve<Vec2>>,
    color: Color,
}

impl Drawing {
    fn new(start: Vec2, color: Color) -> Self {
        Self {
            points: vec![start],
            color,
            ..default()
        }
    }
}

//#[derive(Component, Default)]
// struct ColorButton {
//    selected: usize,
//}

// Systems
// ---

fn init(
    mut cmd: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config: ResMut<GizmoConfigStore>,
) -> Result {
    let data = DrawData::default();
    cmd.insert_resource(data);

    // Canvas
    let size = window.single()?.size();
    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(CANVAS_COLOR))),
        Transform::from_xyz(0., 0., CANVAS_LAYER).with_scale(size.extend(1.)),
        Canvas,
    ))
    .observe(on_pointer_down)
    .observe(on_pointer_draw);

    // Color button
    // cmd.spawn((ColorButton::default(), Button, Text::new("Color"), Node {
    //    position_type: PositionType::Absolute,
    //    bottom: Val::Px(5.0),
    //    right: Val::Px(5.0),
    //    ..default()
    //}));

    // Gizmos
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line.width = 10.;
    config.line.joints = GizmoLineJoint::Round(4);

    Ok(())
}

fn on_pointer_down(
    click: Trigger<Pointer<Pressed>>,
    mut cmd: Commands,
    drawings: Query<Entity, With<Drawing>>,
    mut data: ResMut<DrawData>,
) {
    match click.button {
        // Start a new drawing
        PointerButton::Primary => {
            data.line_start = click.hit.position.unwrap_or_default().xy();
            data.current = Some(
                cmd.spawn(Drawing::new(
                    data.line_start,
                    COLORS[data.selected_color].into(),
                ))
                .id(),
            );
        },
        // Clear the screen
        PointerButton::Secondary => {
            for drawing in &drawings {
                cmd.entity(drawing).despawn();
            }
        },
        PointerButton::Middle => {},
    }
}

// Temporary, change the color
fn on_keyboard(keys: Res<ButtonInput<KeyCode>>, mut data: ResMut<DrawData>) {
    if keys.just_pressed(KeyCode::Space) {
        data.selected_color = (data.selected_color + 1) % COLORS.len();
    }
}

// Continue the drawing
fn on_pointer_draw(
    pointer: Trigger<Pointer<Drag>>,
    mut drawings: Query<&mut Drawing>,
    data: Res<DrawData>,
) {
    let Some(current) = data.current else { return };
    let mut drawing = drawings.get_mut(current).unwrap();

    let pos = data.line_start + pointer.distance * Vec2::new(1., -1.);
    let last = drawing.points.last().unwrap();

    if (pos - last).length() < 10. {
        return;
    }

    drawing.points.push(pos);
    drawing.length += 1;

    if drawing.length > 1 {
        let range = interval(0., 1.).unwrap();
        drawing.curve = Some(SampleAutoCurve::new(range, drawing.points.clone()).unwrap());
    }
}

/// Quick drawing of the curves to the screen.
fn draw_curve(drawings: Query<&Drawing>, mut gizmos: Gizmos) {
    for drawing in &drawings {
        let Some(curve) = &drawing.curve else {
            continue;
        };
        gizmos.curve_2d(
            curve,
            (0..=drawing.length).map(|n| n as f32 / drawing.length as f32),
            drawing.color,
        );
    }
}

/// Fit the canvas to the screen.
fn on_resize(
    mut readedr: EventReader<WindowResized>,
    mut canvas: Query<&mut Transform, With<Canvas>>,
) -> Result {
    for e in readedr.read() {
        let mut canvas = canvas.single_mut()?;
        canvas.scale = Vec3::new(e.width, e.height, 1.);
    }
    Ok(())
}
