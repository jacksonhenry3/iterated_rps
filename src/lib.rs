pub use graph::{self, Node, ID};
use rayon::prelude::*;
use std::collections::HashMap;
pub use strategies::Strategy;
pub mod strategies;

//derive debug
#[derive(Debug, Copy, Clone)]
pub struct NodeData {
    pub strategy: strategies::Strategy,
    pub bank: f64,
    pub position: (f32, f32),
}

impl Default for NodeData {
    fn default() -> Self {
        NodeData {
            strategy: rand::random(),
            bank: 0.0,
            position: (0.0, 0.0),
        }
    }
}

pub struct Arena {
    pub payoff_matrix: strategies::PayoffMatrix,
    pub graph: graph::Graph<NodeData>,
    pub beta: f32,
}

impl Arena {
    pub fn node_mutable_data(&mut self, id: ID) -> NodeData {
        self.graph.data.nodes.get_mut(&id).unwrap().data
    }

    pub fn node_static_data(&self, id: ID) -> NodeData {
        self.graph.data.nodes[&id].data
    }
}

//ignor unused
#[allow(unused)]
impl Arena {
    fn ns(&self, id: ID) -> HashMap<Strategy, f64> {
        let mut ns = HashMap::new();

        // loop over the neighborhood of a and update ns with the bank values
        for i in self.graph.neighborhood(id) {
            let b = self.node_static_data(i);

            // add the bank value to the strategy
            *ns.entry(b.strategy).or_insert(0.0) += b.bank; //community formation without MEMORY! remove the +
        }
        ns
    }

    pub fn play(&mut self) {
        let keys = &self.graph.keys;

        // use rayon to construct a hashtable id to bank value
        let bank: HashMap<ID, f64> = keys
            .par_iter()
            .map(|id| {
                let a = self.node_static_data(*id);

                let bank = self
                    .graph
                    .neighborhood(*id)
                    .iter()
                    .map(|i| {
                        self.payoff_matrix[(a.strategy, self.node_static_data(*i).strategy)] as f64
                    })
                    .sum();

                (*id, bank)
            })
            .collect();

        // update the bank values
        keys.iter().for_each(|id| {
            self.graph.data.nodes.get_mut(id).unwrap().data.bank += bank[id];
        });
    }

    pub fn update_strategies(&mut self) {

        let keys = &self.graph.keys;

        // use rayon to construct a hashtable id to strategy
        let strategies: HashMap<ID, Strategy> = keys
            .par_iter()
            .map(|id| {
                let ns = self.ns(*id);

                //use a weighted boltzman distrobution to choose a strategy
                let strategy = strategies::weighted_boltzman(self.beta, &ns);
                (*id, strategy)
            })
            .collect();

        // update the strategies using rayon
        keys.iter().for_each(|id| {
            self.graph.data.nodes.get_mut(id).unwrap().data.strategy = strategies[id];
        });
    }

    pub fn update(&mut self) {
        self.play();
        self.update_strategies();
    }
}
