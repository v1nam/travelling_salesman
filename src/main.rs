use macroquad::prelude::*;
mod aco;

use aco::Colony;

fn window_conf() -> Conf {
    Conf {
        window_title: "Travelling Salesman".to_owned(),
        window_width: 1000,
        window_height: 700,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut nodes: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    let mut start = false;
    let mut colony = Colony::default(vec![(0, 0)]);
    let mut edges: Vec<(f32, f32, f32, f32)> = Vec::new();
    let mut help_screen = false;
    let help_controls = [
        "Click to add Nodes",
        "Press Space to Run",
        "U / CTRL + Z to Undo",
        "Press C to Clear Screen",
    ];
    loop {
        clear_background(Color::from_rgba(23, 26, 32, 255));

        if help_screen {
            show_help(&help_controls);
            if is_key_pressed(KeyCode::Escape) {
                help_screen = false;
            }
        } else if start {
            if !colony.shortest_path.is_empty() {
                edges = Vec::new();
                let x = colony.shortest_path.len() - 1;
                for j in 0..x {
                    let p1 = nodes[colony.shortest_path[j] as usize];
                    let p2 = nodes[colony.shortest_path[j + 1] as usize];
                    edges.push((p1.0 as f32, p1.1 as f32, p2.0 as f32, p2.1 as f32));
                }
                let p1 = nodes[colony.shortest_path[x] as usize];
                let p2 = nodes[0];
                edges.push((p1.0 as f32, p1.1 as f32, p2.0 as f32, p2.1 as f32));
            }

            colony.mainloop();
            i += 1;
            if i >= colony.iterations {
                start = false;
                i = 0;
            }
        } else if is_key_pressed(KeyCode::Space) && !nodes.is_empty() {
            start = true;
            colony = Colony::default(nodes.clone());
            edges = Vec::new();
        } else if is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position();
            let pos = (pos.0 as u32, pos.1 as u32);
            if !nodes.contains(&pos) {
                nodes.push(pos);
            }
        } else if is_key_pressed(KeyCode::C) {
            nodes = Vec::new();
            edges = Vec::new();
            colony = Colony::default(vec![(0, 0)]);
        } else if is_key_pressed(KeyCode::U)
            || (is_key_pressed(KeyCode::Z) && is_key_down(KeyCode::LeftControl))
        {
            if !nodes.is_empty() {
                nodes.pop();
            }
        } else if is_key_pressed(KeyCode::H) {
            help_screen = true;
        }

        if !help_screen {
            for node_pos in &nodes {
                draw_circle(
                    node_pos.0 as f32,
                    node_pos.1 as f32,
                    8.0,
                    Color::from_rgba(216, 222, 233, 255),
                );
            }
            for edge_pos in &edges {
                draw_line(
                    edge_pos.0,
                    edge_pos.1,
                    edge_pos.2,
                    edge_pos.3,
                    2.0,
                    Color::from_rgba(216, 222, 233, 255),
                );
            }

            draw_text(
                if start { "RUNNING" } else { "PAUSED" },
                10.0,
                20.0,
                25.0,
                Color::from_rgba(216, 222, 233, 180),
            );

            draw_text(
                &format!("Shortest Distance: {}px", colony.shortest_distance as u32),
                10.0,
                50.0,
                25.0,
                Color::from_rgba(216, 222, 233, 180),
            );

            draw_text(
                &format!("Nodes: {}", nodes.len()),
                10.0,
                80.0,
                25.0,
                Color::from_rgba(216, 222, 233, 180),
            );

            draw_text(
                "H for help",
                screen_width() - 135.0,
                20.0,
                25.0,
                Color::from_rgba(216, 222, 233, 180),
            );
        }
        next_frame().await
    }
}

fn show_help(help_controls: &[&'static str]) {
    draw_text(
        "ESC to go back",
        10.0,
        20.0,
        25.0,
        Color::from_rgba(216, 222, 233, 180),
    );
    draw_text(
        "HELP",
        (screen_width() / 2.0) - 4.0 * 22.0 / 2.0,
        100.0,
        44.0,
        Color::from_rgba(216, 222, 233, 180),
    );
    for (i, text) in help_controls.iter().enumerate() {
        draw_text(
            text,
            (screen_width() / 2.0) - (text.len() as f32 * 15.0 / 2.0),
            (screen_height() / 2.0) - (60.0 - (30.0 * i as f32)),
            30.0,
            Color::from_rgba(216, 222, 233, 180),
        );
    }
}
