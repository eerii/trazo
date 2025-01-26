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
        .add_systems(Update, (on_resize, draw_curve))
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

// Systems
// ---

fn init(
    mut cmd: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut config: ResMut<GizmoConfigStore>,
) {
    let data = DrawData::default();
    cmd.insert_resource(data);

    // Canvas
    let size = single!(window).size();
    cmd.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(CANVAS_COLOR))),
        Transform::from_xyz(0., 0., CANVAS_LAYER).with_scale(size.extend(1.)),
        Canvas,
    ))
    .observe(on_draw_start)
    .observe(on_draw);

    // Gizmos
    let (config, _) = config.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 10.;
    config.line_joints = GizmoLineJoint::Round(4);
}

fn on_draw_start(drag: Trigger<Pointer<DragStart>>, mut cmd: Commands, mut data: ResMut<DrawData>) {
    data.line_start = drag.hit.position.unwrap_or_default().xy();
    data.selected_color = (data.selected_color + 1) % COLORS.len();
    data.current = Some(
        cmd.spawn(Drawing::new(
            data.line_start,
            COLORS[data.selected_color].into(),
        ))
        .id(),
    );
}

fn on_draw(drag: Trigger<Pointer<Drag>>, mut drawings: Query<&mut Drawing>, data: Res<DrawData>) {
    let pos = data.line_start + drag.distance * Vec2::new(1., -1.);

    let mut drawing = drawings.get_mut(data.current.unwrap()).unwrap();
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

fn on_resize(
    mut readedr: EventReader<WindowResized>,
    mut canvas: Query<&mut Transform, With<Canvas>>,
) {
    for e in readedr.read() {
        let mut canvas = single_mut!(canvas);
        canvas.scale = Vec3::new(e.width, e.height, 1.);
    }
}

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
