use iterated_rps;
use nannou::prelude::*;
use rayon::prelude::*;

//use nannou to display a grid graph as it updates
fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    arena: iterated_rps::Arena,
}

fn model(app: &App) -> Model {
    let dim = 100;
    let _window = app.new_window().view(view).build().unwrap();
    let mut a = iterated_rps::graph::generate_grid_graph::<iterated_rps::NodeData>(dim, dim);
    // give each node a position
    for id in &a.keys {
        // get index from id
        let i = id.0;

        //get x and y from i
        let x = (i % dim) as f32;
        let y = (i / dim) as f32;

        a.data.nodes.get_mut(id).unwrap().data.position =
            (x / (dim as f32 - 1.0) - 0.5, y / (dim as f32 - 1.0) - 0.5);
    }
    let e = 0.0;
    let d = 0.0;
    let arena = iterated_rps::Arena {
        graph: a,
        payoff_matrix: iterated_rps::strategies::PayoffMatrix {
            matrix: [
                [0.0 + d, -1.0, 1.0 + e],
                [1.0 + e, 0.0 + d, -1.0],
                [-1.0, 1.0 + e, 0.0 + d],
            ],
            // matrix: [
            //     [1.0 + d, 0.0, 2.0 + e],
            //     [2.0 + e, 1.0 + d, -1.0],
            //     [0.0, 2.0 + e ,1.0 + d],
            // ],
        },
        beta: 100.0,
    };
    Model { arena }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    //only update when spacebar is pressed
    // if _app.keys.down.contains(&Key::Space) {
        model.arena.update();
    // }

    //reset all strategies to a random value and all banks to zero when r is hit
    if _app.keys.down.contains(&Key::R) {
        for (id, node) in &mut model.arena.graph.data.nodes {
            node.data.strategy = rand::random();
            node.data.bank = 0.0;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    //use rayon to find the largest bank value
    let max_bank = model
        .arena
        .graph
        .keys
        .par_iter()
        .map(|id| model.arena.node_static_data(*id).bank)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    for (id, node) in &model.arena.graph.data.nodes {
        let (x, y) = node.data.position;
        let b = node.data.bank;
        let alpha = b / max_bank * 0.2 + 0.7;
        let color = match node.data.strategy {
            iterated_rps::strategies::Strategy::Rock => rgba(0.5, 0.7, 0.1, alpha),
            iterated_rps::strategies::Strategy::Paper => rgba(0.1, 0.5, 0.7, alpha),
            iterated_rps::strategies::Strategy::Scissors => rgba(0.7, 0.1, 0.5, alpha),
        };
        //window width
        let w = app.window_rect().w();
        let h = app.window_rect().h();

        //scale x and y to fill the window
        let x = x * h;
        let y = y * h;

        //calculate the width and height of the rectangle
        let w = w / 100.0;
        let h = h / 100.0;

        draw.rect().x_y(x as f32, y as f32).w_h(h, h).color(color);
    }
    draw.to_frame(app, &frame).unwrap();
}
