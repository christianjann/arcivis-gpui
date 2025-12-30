use crate::grid::Grid;
use crate::types::*;
use std::fs;

pub fn generate_svg(result: &LayoutResult, filename: &str, show_grid: bool, show_all_ports: bool) {
    generate_svg_with_obstacles(
        result,
        filename,
        show_grid,
        show_all_ports,
        None, // Disable obstacle drawing by default
    )
}

pub fn generate_svg_with_obstacles(
    result: &LayoutResult,
    filename: &str,
    show_grid: bool,
    show_all_ports: bool,
    grid: Option<&Grid>,
) {
    use std::collections::HashSet;
    let mut used_ports = HashSet::new();

    for edge in &result.edges {
        used_ports.insert((edge.source, edge.source_port));
        used_ports.insert((edge.target, edge.target_port));
    }

    let mut svg = format!(
        r#"<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg">
<rect width="100%" height="100%" fill="white"/>
"#,
        result.canvas_width, result.canvas_height
    );

    // Add grid lines if requested
    if show_grid {
        let cell_size = 5.0;
        let num_x_lines = (result.canvas_width / cell_size).ceil() as usize;
        let num_y_lines = (result.canvas_height / cell_size).ceil() as usize;

        // Vertical grid lines
        for i in 0..=num_x_lines {
            let x = i as f64 * cell_size;
            svg.push_str(&format!(
                r#"<line x1="{}" y1="0" x2="{}" y2="{}" stroke="lightgray" stroke-width="0.5"/>
"#,
                x, x, result.canvas_height
            ));
        }

        // Horizontal grid lines
        for i in 0..=num_y_lines {
            let y = i as f64 * cell_size;
            svg.push_str(&format!(
                r#"<line x1="0" y1="{}" x2="{}" y2="{}" stroke="lightgray" stroke-width="0.5"/>
"#,
                y, result.canvas_width, y
            ));
        }
    }

    // Add obstacle visualization if grid is provided
    if let Some(grid) = grid {
        let cell_size = 5.0;
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.obstacles[y][x] {
                    // This is an obstacle (true means blocked)
                    let pos_x = grid.origin_x + x as f64 * cell_size;
                    let pos_y = grid.origin_y + y as f64 * cell_size;
                    svg.push_str(&format!(
                        r#"<rect x="{}" y="{}" width="{}" height="{}" fill="red" fill-opacity="0.3" stroke="red" stroke-width="0.5"/>
"#,
                        pos_x, pos_y, cell_size, cell_size
                    ));
                }
            }
        }
    }

    for node in result.nodes.iter() {
        let fill_color = node
            .attributes
            .iter()
            .find(|(k, _)| k == "color")
            .map(|(_, v)| v.as_str())
            .unwrap_or("lightblue");

        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="black"/>
<text x="{}" y="{}" font-family="Arial" font-size="12" text-anchor="middle">{}</text>
"#,
            node.position.x,
            node.position.y,
            node.size.width,
            node.size.height,
            fill_color,
            node.position.x + node.size.width / 2.0,
            node.position.y + node.size.height / 2.0 + 5.0,
            node.id
        ));
    }

    for edge in &result.edges {
        if edge.path.len() >= 2 {
            let mut path_data = format!("M {} {}", edge.path[0].x, edge.path[0].y);
            for point in &edge.path[1..] {
                path_data.push_str(&format!(" L {} {}", point.x, point.y));
            }
            svg.push_str(&format!(
                r#"<path d="{}" stroke="black" stroke-width="2" fill="none"/>
"#,
                path_data
            ));
        }
    }

    for (node_index, node) in result.nodes.iter().enumerate() {
        for (port_index, port) in node.ports.iter().enumerate() {
            if show_all_ports || used_ports.contains(&(node_index, Some(port_index))) {
                let fill = match port.port_type {
                    PortType::Input => "lightblue",
                    PortType::Output => "lightcoral",
                };
                svg.push_str(&format!(
                    r#"<rect x="{}" y="{}" width="{}" height="{}" fill="{}" stroke="black"/>
"#,
                    node.position.x + port.position.x,
                    node.position.y + port.position.y,
                    port.size.width,
                    port.size.height,
                    fill
                ));
            }
        }
    }

    svg.push_str("</svg>");

    fs::write(filename, svg).expect("Unable to write SVG");
}
