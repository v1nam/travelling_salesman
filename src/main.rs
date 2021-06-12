mod aco;

use aco::Colony;
use sfml::{
    graphics::{
        CircleShape, Color, PrimitiveType, RenderTarget, RenderWindow, Shape, Transformable,
        Vertex, VertexArray,
    },
    system::Vector2f,
    window::{mouse, Event, Key, Style},
};

fn main() {
    let mut window = RenderWindow::new(
        (950, 600),
        "Custom shape",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let mut nodes: Vec<(u32, u32)> = Vec::new();
    let mut circles: Vec<CircleShape> = Vec::new();
    let mut start = false;
    let mut lines = VertexArray::default();
    lines.set_primitive_type(PrimitiveType::LINE_STRIP);

    loop {
        window.clear(Color::rgb(30, 36, 38));

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::ESCAPE, ..
                } => return,
                Event::MouseButtonPressed { button, x, y } => match button {
                    mouse::Button::LEFT => {
                        let mut circ = CircleShape::default();
                        circ.set_position((x as f32, y as f32));
                        circ.set_origin((5., 5.));
                        circ.set_radius(10.);
                        circ.set_fill_color(Color::rgb(247, 244, 243));
                        nodes.push((x as u32, y as u32));
                        circles.push(circ);
                    }
                    _ => {}
                },
                Event::KeyPressed {
                    code: Key::ENTER, ..
                } => {
                    start = true;
                    lines = VertexArray::default();
                    lines.set_primitive_type(PrimitiveType::LINE_STRIP);
                }
                _ => {}
            }
        }
        if start {
            let mut colony = Colony::default(nodes.clone());
            colony.mainloop();
            for i in colony.shortest_path {
                let p = nodes[i as usize];
                lines.append(&Vertex::with_pos_color(
                    Vector2f::new(p.0 as f32, p.1 as f32),
                    Color::rgb(247, 244, 243),
                ));
            }
            lines.append(&Vertex::with_pos_color(
                Vector2f::new(nodes[0].0 as f32, nodes[0].1 as f32),
                Color::rgb(247, 244, 243),
            ));
            start = false;
        }

        for circ in circles.iter() {
            window.draw(circ);
        }
        window.draw(&lines);
        window.display();
    }
}
