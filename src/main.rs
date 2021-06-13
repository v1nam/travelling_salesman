use macroquad::prelude::*;
mod aco;

use aco::Colony;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut nodes: Vec<(u32, u32)> = Vec::new();
    let mut i = 0;
    let mut start = false;
    let mut colony = Colony::default(vec![(0, 0)]);
    let mut edges: Vec<(f32, f32, f32, f32)> = Vec::new();
    loop {
        clear_background(Color::from_rgba(30, 36, 38, 255));

        if start {
            if i >= colony.iterations {
                start = false;
                i = 0;
                for j in 0..colony.shortest_path.len() - 1 {
                    let p1 = nodes[colony.shortest_path[j] as usize];
                    let p2 = nodes[colony.shortest_path[j + 1] as usize];
                    edges.push((p1.0 as f32, p1.1 as f32, p2.0 as f32, p2.1 as f32));
                }
            }
            colony.mainloop();
            i += 1;
        }
        if is_key_pressed(KeyCode::Space) {
            start = true;
            colony = Colony::default(nodes.clone());
            edges = Vec::new();
        }
        if !start && is_mouse_button_pressed(MouseButton::Left) {
            let pos = mouse_position();
            nodes.push((pos.0 as u32, pos.1 as u32));
        }
        for node_pos in &nodes {
            draw_circle(node_pos.0 as f32, node_pos.1 as f32, 8.0, Color::from_rgba(247, 244, 243, 255));
        }
        for edge_pos in &edges {
            draw_line(edge_pos.0, edge_pos.1, edge_pos.2, edge_pos.3, 2.0, Color::from_rgba(247, 244, 243, 255));
        }

        draw_text(if start { "RUNNING" } else { "PAUSED" }, 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}
