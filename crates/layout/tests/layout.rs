use archviz_layout::*;
use tracing_test::traced_test;

#[traced_test]
#[test]
fn test_layout_set_1() {
    let nodes = vec![
        Node {
            id: "ECU1".to_string(),
            size: Size {
                width: 120.0,
                height: 80.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 35.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 120.0, y: 35.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 55.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 55.0, y: 80.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "ECU2".to_string(),
            size: Size {
                width: 100.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 100.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: 60.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Sensor".to_string(),
            size: Size {
                width: 80.0,
                height: 40.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 15.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 80.0, y: 15.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 35.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 35.0, y: 40.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
    ];
    let edges = vec![(0, 1, None, None), (1, 2, None, None), (0, 2, None, None)];

    let layout = ArchVizLayout {
        min_spacing: 120.0,
        ..Default::default()
    };
    let result = layout.layout(nodes, edges);

    println!("Set 1 Nodes:");
    for node in &result.nodes {
        println!(
            "  {}: size {:?}, position {:?}",
            node.id, node.size, node.position
        );
    }
    println!("Set 1 Edges:");
    for edge in &result.edges {
        println!("  {} -> {}: {:?}", edge.source, edge.target, edge.path);
    }

    generate_svg(&result, "test_set_1.svg", true, true);
}

#[traced_test]
#[test]
fn test_layout_set_2() {
    let nodes = vec![
        Node {
            id: "Gateway".to_string(),
            size: Size {
                width: 150.0,
                height: 100.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 150.0, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 70.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 70.0, y: 100.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Display".to_string(),
            size: Size {
                width: 90.0,
                height: 70.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 90.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 40.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 40.0, y: 70.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Battery".to_string(),
            size: Size {
                width: 60.0,
                height: 50.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 60.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 25.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 25.0, y: 50.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Motor".to_string(),
            size: Size {
                width: 110.0,
                height: 90.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 40.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 110.0, y: 40.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: 90.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
    ];
    let edges = vec![
        (0, 1, None, None),
        (0, 2, None, None),
        (0, 3, None, None),
        (1, 3, None, None),
        (2, 3, None, None),
    ];

    let layout = ArchVizLayout {
        spaced_edges: true,
        min_spacing: 120.0,
        ..Default::default()
    };
    let result = layout.layout(nodes, edges);

    println!("Set 2 Nodes:");
    for node in &result.nodes {
        println!(
            "  {}: size {:?}, position {:?}",
            node.id, node.size, node.position
        );
    }
    println!("Set 2 Edges:");
    for edge in &result.edges {
        println!("  {} -> {}: {:?}", edge.source, edge.target, edge.path);
    }

    generate_svg(&result, "test_set_2.svg", false, false);
}

#[traced_test]
#[test]
fn test_layout_set_3() {
    let nodes = vec![
        Node {
            id: "ABS".to_string(),
            size: Size {
                width: 100.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -5.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 95.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: -5.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: 55.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "ESP".to_string(),
            size: Size {
                width: 120.0,
                height: 80.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -5.0, y: 35.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 115.0, y: 35.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 55.0, y: -5.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 55.0, y: 75.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Airbag".to_string(),
            size: Size {
                width: 90.0,
                height: 50.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -5.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 85.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 40.0, y: -5.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 40.0, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Climate".to_string(),
            size: Size {
                width: 110.0,
                height: 70.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -5.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 105.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: -5.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: 65.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Infotainment".to_string(),
            size: Size {
                width: 140.0,
                height: 100.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -5.0, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 135.0, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 65.0, y: -5.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 65.0, y: 95.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
    ];
    let edges = vec![
        (0, 1, None, None),
        (1, 2, None, None),
        (2, 3, None, None),
        (3, 4, None, None),
        (0, 4, None, None),
        (1, 3, None, None),
    ];

    let layout = ArchVizLayout {
        allow_diagonals: false,
        min_spacing: 120.0,
        ..Default::default()
    };
    let result = layout.layout(nodes, edges);

    println!("Set 3 Nodes:");
    for node in &result.nodes {
        println!(
            "  {}: size {:?}, position {:?}",
            node.id, node.size, node.position
        );
    }
    println!("Set 3 Edges:");
    for edge in &result.edges {
        println!("  {} -> {}: {:?}", edge.source, edge.target, edge.path);
    }

    generate_svg(&result, "test_set_3.svg", true, false);
}

#[traced_test]
#[test]
fn test_layout_set_4() {
    let nodes = vec![
        Node {
            id: "Engine".to_string(),
            size: Size {
                width: 140.0,
                height: 90.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 40.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 140.0, y: 40.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 65.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 65.0, y: 90.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Transmission".to_string(),
            size: Size {
                width: 120.0,
                height: 70.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 120.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 55.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 55.0, y: 70.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Brakes".to_string(),
            size: Size {
                width: 100.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 100.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: 60.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Steering".to_string(),
            size: Size {
                width: 110.0,
                height: 65.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 27.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 110.0, y: 27.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: 65.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Sensors".to_string(),
            size: Size {
                width: 90.0,
                height: 50.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 90.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 40.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 40.0, y: 50.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Dashboard".to_string(),
            size: Size {
                width: 130.0,
                height: 75.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 32.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 130.0, y: 32.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 60.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 60.0, y: 75.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "ECU".to_string(),
            size: Size {
                width: 100.0,
                height: 55.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 22.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 100.0, y: 22.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 45.0, y: 55.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "FuelPump".to_string(),
            size: Size {
                width: 85.0,
                height: 45.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 17.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 85.0, y: 17.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 37.5, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 37.5, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Alternator".to_string(),
            size: Size {
                width: 95.0,
                height: 55.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 22.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 95.0, y: 22.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 42.5, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 42.5, y: 55.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Radiator".to_string(),
            size: Size {
                width: 110.0,
                height: 70.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 110.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 50.0, y: 70.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Battery".to_string(),
            size: Size {
                width: 80.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 80.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 35.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 35.0, y: 60.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "AirFilter".to_string(),
            size: Size {
                width: 75.0,
                height: 40.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 15.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 75.0, y: 15.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 32.5, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 32.5, y: 40.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Exhaust".to_string(),
            size: Size {
                width: 125.0,
                height: 50.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 125.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 57.5, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 57.5, y: 50.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Catalytic".to_string(),
            size: Size {
                width: 105.0,
                height: 45.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 17.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 105.0, y: 17.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 47.5, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 47.5, y: 45.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
    ];
    // Complex connectivity with cross-connections and potential routing challenges
    let edges = vec![
        (0, 1, None, None),
        (1, 2, None, None),
        (2, 3, None, None),
        (3, 4, None, None),
        (4, 5, None, None),
        (5, 6, None, None), // main chain
        (0, 3, None, None),
        (1, 4, None, None),
        (2, 5, None, None),
        (0, 6, None, None), // cross connections
        (3, 6, None, None),
        (4, 6, None, None), // additional connections to ECU
        // New component connections
        (0, 7, None, None),
        (7, 8, None, None),
        (8, 9, None, None),
        (9, 10, None, None), // fuel system chain
        (1, 11, None, None),
        (11, 12, None, None),
        (12, 13, None, None), // electrical system chain
        (2, 9, None, None),
        (3, 10, None, None),
        (4, 11, None, None), // cooling system connections
        (5, 12, None, None),
        (6, 13, None, None), // exhaust system connections
        (7, 10, None, None),
        (8, 13, None, None),
        (9, 6, None, None), // additional cross-connections
    ];

    let layout = ArchVizLayout {
        allow_diagonals: false,
        min_spacing: 120.0,
        ..Default::default()
    };
    let result = layout.layout(nodes, edges);

    println!("Set 4 Nodes:");
    for node in &result.nodes {
        println!(
            "  {}: size {:?}, position {:?}",
            node.id, node.size, node.position
        );
    }
    println!("Set 4 Edges:");
    for edge in &result.edges {
        println!("  {} -> {}: {:?}", edge.source, edge.target, edge.path);
    }

    generate_svg(&result, "test_set_4.svg", true, false);
}

#[traced_test]
#[test]
fn test_layout_set_5() {
    let nodes = vec![
        // CAN Bus node with 8 ports
        Node {
            id: "CAN_Bus".to_string(),
            size: Size {
                width: 200.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                // Left side inputs (2 ports)
                Port {
                    position: Position { x: -10.0, y: 15.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: -10.0, y: 35.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                // Right side outputs (2 ports)
                Port {
                    position: Position { x: 200.0, y: 15.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 200.0, y: 35.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                // Top side inputs (2 ports)
                Port {
                    position: Position { x: 45.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 145.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                // Bottom side outputs (2 ports)
                Port {
                    position: Position { x: 45.0, y: 60.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 145.0, y: 60.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![("color".to_string(), "grey".to_string())],
        },
        // Ethernet Backbone node with 4 ports
        Node {
            id: "Ethernet_Backbone".to_string(),
            size: Size {
                width: 180.0,
                height: 50.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                // Left side input
                Port {
                    position: Position { x: -10.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                // Right side output
                Port {
                    position: Position { x: 180.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                // Top side input
                Port {
                    position: Position { x: 85.0, y: -10.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                // Bottom side output
                Port {
                    position: Position { x: 85.0, y: 50.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![("color".to_string(), "grey".to_string())],
        },
        // Gateway ECU connected to both CAN and Ethernet
        Node {
            id: "Gateway_ECU".to_string(),
            size: Size {
                width: 120.0,
                height: 80.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: -10.0, y: 50.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 120.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
                Port {
                    position: Position { x: 120.0, y: 50.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![("color".to_string(), "violet".to_string())],
        },
        // Engine ECU connected to CAN
        Node {
            id: "Engine_ECU".to_string(),
            size: Size {
                width: 100.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 100.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        // Transmission ECU connected to CAN
        Node {
            id: "Transmission_ECU".to_string(),
            size: Size {
                width: 130.0,
                height: 70.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 130.0, y: 30.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        // Body Control ECU connected to CAN
        Node {
            id: "Body_ECU".to_string(),
            size: Size {
                width: 110.0,
                height: 65.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 27.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 110.0, y: 27.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        // Telematics ECU connected to Ethernet
        Node {
            id: "Telematics_ECU".to_string(),
            size: Size {
                width: 120.0,
                height: 55.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 22.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 120.0, y: 22.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        // ADAS ECU connected to Ethernet
        Node {
            id: "ADAS_ECU".to_string(),
            size: Size {
                width: 100.0,
                height: 50.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 100.0, y: 20.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        // Chassis ECU connected to CAN
        Node {
            id: "Chassis_ECU".to_string(),
            size: Size {
                width: 110.0,
                height: 65.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 27.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 110.0, y: 27.5 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        // Cluster ECU connected to CAN
        Node {
            id: "Cluster_ECU".to_string(),
            size: Size {
                width: 105.0,
                height: 60.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 105.0, y: 25.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
    ];

    // Unidirectional edges - no duplicates, with port indices
    let edges = vec![
        // Gateway ECU connections
        (2, 0, None, None), // Gateway_ECU -> CAN_Bus (port 0 on Gateway, port 0 on CAN)
        (2, 1, Some(1), Some(0)), // Gateway_ECU -> Ethernet_Backbone (port 1 on Gateway, port 0 on Ethernet)
        // CAN-connected ECUs - each uses a different CAN port
        (3, 0, Some(0), Some(1)), // Engine_ECU -> CAN_Bus (port 0 on Engine, port 1 on CAN)
        (4, 0, Some(0), Some(2)), // Transmission_ECU -> CAN_Bus (port 0 on Transmission, port 2 on CAN)
        (5, 0, Some(0), Some(3)), // Body_ECU -> CAN_Bus (port 0 on Body, port 3 on CAN)
        (8, 0, Some(0), Some(4)), // Chassis_ECU -> CAN_Bus (port 0 on Chassis, port 4 on CAN)
        (9, 0, Some(0), Some(5)), // Cluster_ECU -> CAN_Bus (port 0 on Cluster, port 5 on CAN)
        // Ethernet-connected ECUs
        (6, 1, Some(0), Some(1)), // Telematics_ECU -> Ethernet_Backbone (port 0 on Telematics, port 1 on Ethernet)
        (7, 1, Some(0), Some(2)), // ADAS_ECU -> Ethernet_Backbone (port 0 on ADAS, port 2 on Ethernet)
    ];

    let layout = ArchVizLayout {
        allow_diagonals: false,
        min_spacing: 120.0,
        ..Default::default()
    };
    let result = layout.layout(nodes, edges);

    println!("Set 5 Nodes:");
    for node in &result.nodes {
        println!(
            "  {}: size {:?}, position {:?}",
            node.id, node.size, node.position
        );
    }
    println!("Set 5 Edges:");
    for edge in &result.edges {
        println!("  {} -> {}: {:?}", edge.source, edge.target, edge.path);
    }

    generate_svg(&result, "test_set_5.svg", true, true);
}

#[traced_test]
#[test]
fn test_layout_set_6() {
    let nodes = vec![
        Node {
            id: "CAN_Powertrain".to_string(),
            size: Size {
                width: 144.79999,
                height: 48.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position {
                        x: 144.79999,
                        y: 14.0,
                    },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "CAN_Chassis".to_string(),
            size: Size {
                width: 140.0,
                height: 48.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 140.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "CAN_Body".to_string(),
            size: Size {
                width: 140.0,
                height: 48.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 140.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Ethernet_Backbone".to_string(),
            size: Size {
                width: 166.4,
                height: 48.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 166.4, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "Ethernet_ADAS".to_string(),
            size: Size {
                width: 140.0,
                height: 48.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 140.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "GatewayECU".to_string(),
            size: Size {
                width: 360.0,
                height: 291.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 360.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "EngineECU".to_string(),
            size: Size {
                width: 432.0,
                height: 243.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 432.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "TransmissionECU".to_string(),
            size: Size {
                width: 396.0,
                height: 198.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 396.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "VehicleDynamicsECU".to_string(),
            size: Size {
                width: 408.0,
                height: 291.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 408.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "ADASController".to_string(),
            size: Size {
                width: 336.0,
                height: 381.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 336.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
        Node {
            id: "ClusterECU".to_string(),
            size: Size {
                width: 324.0,
                height: 288.0,
            },
            position: Position { x: 0.0, y: 0.0 },
            ports: vec![
                Port {
                    position: Position { x: -10.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Input,
                    id: None,
                },
                Port {
                    position: Position { x: 324.0, y: 14.0 },
                    size: Size {
                        width: 10.0,
                        height: 10.0,
                    },
                    port_type: PortType::Output,
                    id: None,
                },
            ],
            attributes: vec![],
        },
    ];

    // Define edges based on the automotive network connections
    let edges = vec![
        (5, 0, None, None),  // GatewayECU -> CAN_Powertrain
        (5, 1, None, None),  // GatewayECU -> CAN_Chassis
        (5, 2, None, None),  // GatewayECU -> CAN_Body
        (5, 3, None, None),  // GatewayECU -> Ethernet_Backbone
        (6, 0, None, None),  // EngineECU -> CAN_Powertrain
        (7, 0, None, None),  // TransmissionECU -> CAN_Powertrain
        (8, 1, None, None),  // VehicleDynamicsECU -> CAN_Chassis
        (8, 3, None, None),  // VehicleDynamicsECU -> Ethernet_Backbone
        (9, 4, None, None),  // ADASController -> Ethernet_ADAS
        (9, 3, None, None),  // ADASController -> Ethernet_Backbone
        (10, 2, None, None), // ClusterECU -> CAN_Body
        (10, 3, None, None), // ClusterECU -> Ethernet_Backbone
    ];

    let layout = ArchVizLayout::default();
    let result = layout.layout(nodes, edges);

    println!("Automotive Network Nodes:");
    for node in &result.nodes {
        println!(
            "  {}: size {:?}, position {:?}",
            node.id, node.size, node.position
        );
    }

    println!("Automotive Network Edges:");
    for edge in &result.edges {
        println!(
            "  {} -> {}: {} path points",
            edge.source,
            edge.target,
            edge.path.len()
        );
    }

    generate_svg_with_obstacles(&result, "test_set_6.svg", true, true, result.grid.as_ref());

    // Basic assertions to ensure layout worked
    assert!(!result.nodes.is_empty());
    assert!(!result.edges.is_empty());

    // Check that all nodes have reasonable positions (not all at origin)
    let mut non_zero_positions = 0;
    for node in &result.nodes {
        if node.position.x != 0.0 || node.position.y != 0.0 {
            non_zero_positions += 1;
        }
    }
    assert!(
        non_zero_positions > 0,
        "At least some nodes should be positioned away from origin"
    );

    // Check that edges have paths
    for edge in &result.edges {
        assert!(!edge.path.is_empty(), "Edge should have a path");
    }
}

#[traced_test]
#[test]
fn test_layout_in_place() {
    // Create test data structures that implement the traits
    #[derive(Debug, Clone)]
    struct TestNode {
        id: String,
        position: Position,
        size: Size,
        ports: Vec<Port>,
    }

    impl LayoutNode for TestNode {
        fn position(&self) -> Position {
            self.position.clone()
        }
        fn size(&self) -> Size {
            self.size.clone()
        }
        fn set_position(&mut self, pos: Position) {
            self.position = pos;
        }
        fn id(&self) -> String {
            self.id.clone()
        }
        fn ports(&self) -> Vec<Port> {
            self.ports.clone()
        }
    }

    #[derive(Debug, Clone)]
    struct TestEdge {
        source: usize,
        target: usize,
        source_port: usize,
        target_port: usize,
        path: Vec<Position>,
    }

    impl LayoutEdge for TestEdge {
        fn source(&self) -> usize {
            self.source
        }
        fn target(&self) -> usize {
            self.target
        }
        fn source_port(&self) -> Option<usize> {
            Some(self.source_port)
        }
        fn target_port(&self) -> Option<usize> {
            Some(self.target_port)
        }
        fn set_path(&mut self, path: Vec<Position>) {
            self.path = path;
        }
    }

    // Create test nodes and edges with closer initial positions
    let mut nodes = vec![
        TestNode {
            id: "A".to_string(),
            position: Position { x: 0.0, y: 0.0 },
            size: Size {
                width: 100.0,
                height: 50.0,
            },
            ports: vec![],
        },
        TestNode {
            id: "B".to_string(),
            position: Position { x: 50.0, y: 0.0 }, // Closer together
            size: Size {
                width: 100.0,
                height: 50.0,
            },
            ports: vec![],
        },
    ];

    let mut edges = vec![TestEdge {
        source: 0,
        target: 1,
        source_port: 0,
        target_port: 0,
        path: vec![],
    }];

    // Create layout configuration with more iterations
    let config = ArchVizLayout {
        iterations: 100,
        repulsion_strength: 10000.0,
        attraction_strength: 0.01,
        initial_spacing: 100.0,
        min_spacing: 50.0,
        allow_diagonals: true,
        spaced_edges: true,
    };

    // Run in-place layout
    let result = layout_in_place(&mut nodes, &mut edges, &config);

    // Verify the result
    assert!(result.is_ok());

    // Check that positions have been updated (should move apart due to repulsion)
    assert_ne!(nodes[0].position.x, 0.0);
    assert_ne!(nodes[1].position.x, 50.0); // Should move away from each other

    // Check that edge path has been set
    assert!(!edges[0].path.is_empty());
}
