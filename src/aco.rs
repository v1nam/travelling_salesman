use rand::{seq::SliceRandom, thread_rng};
use std::collections::HashMap;

struct Ant {
    location: u32,
    alpha: f32,
    beta: f32,
    distance_travelled: f32,
    first_round: bool,
    allowed: Vec<u32>,
    pheromone_map: HashMap<(u32, u32), f32>,
    route: Vec<u32>,
}

impl Ant {
    fn new(
        location: u32,
        alpha: f32,
        beta: f32,
        first_round: bool,
        allowed: Vec<u32>,
        pheromone_map: HashMap<(u32, u32), f32>,
    ) -> Ant {
        Ant {
            location,
            alpha,
            beta,
            distance_travelled: 0.0,
            first_round,
            allowed,
            pheromone_map,
            route: Vec::new(),
        }
    }
    fn run(&mut self, rng: &mut rand::rngs::ThreadRng, nodes: &[(u32, u32)]) {
        while !self.allowed.is_empty() {
            let next = self.decide(rng, nodes);
            self.move_(next, nodes);
        }
    }
    fn decide(&self, rng: &mut rand::rngs::ThreadRng, nodes: &[(u32, u32)]) -> u32 {
        if self.first_round {
            return *self.allowed.choose(rng).unwrap();
        }
        let mut attractiveness: HashMap<u32, f32> = HashMap::new();
        let mut total_sum = 0.0;

        for allowed_location in &self.allowed {
            let pher_amount = match self.pheromone_map.get(&(self.location, *allowed_location)) {
                Some(x) => *x,
                None => 0.0,
            };
            let distance = self.distance(&self.location, allowed_location, nodes);
            let edge_prob = pher_amount.powf(self.alpha) * (1.0 / distance).powf(self.beta);
            attractiveness.insert(*allowed_location, edge_prob);
            total_sum += edge_prob;
        }
        if total_sum == 0.0 {
            total_sum = std::f32::MIN_POSITIVE;
            for v in attractiveness.values_mut() {
                *v = std::f32::MIN_POSITIVE;
            }
        }
        let random_fl: f32 = rand::random();
        let mut upto = 0.0;

        for location in attractiveness.keys() {
            let weight = attractiveness.get(location).unwrap() / total_sum;
            if weight + upto >= random_fl {
                return *location;
            }
            upto += weight;
        }
        0
    }
    fn move_(&mut self, end: u32, nodes: &[(u32, u32)]) {
        let val = self.allowed.iter().position(|x| x == &end).unwrap();
        self.allowed.remove(val);
        self.route.push(end);
        self.distance_travelled += self.distance(&self.location, &end, nodes);
        self.location = end;
    }
    fn distance(&self, p1: &u32, p2: &u32, nodes: &[(u32, u32)]) -> f32 {
        let p1 = *p1 as usize;
        let p2 = *p2 as usize;
        (((nodes[p2].0 as i32 - nodes[p1].0 as i32).pow(2)
            + (nodes[p2].1 as i32 - nodes[p1].1 as i32).pow(2)) as f32)
            .sqrt()
    }
}

pub struct Colony {
    nodes: Vec<(u32, u32)>,
    start: u32,
    ant_count: u32,
    alpha: f32,
    beta: f32,
    pheromone_evaporation: f32,
    pheromone_value: f32,
    pub iterations: u32,
    shortest_distance: f32,
    pub shortest_path: Vec<u32>,
    ants: Vec<Ant>,
    pheromone_map: HashMap<(u32, u32), f32>,
    ant_pheromone_map: HashMap<(u32, u32), f32>,
    first_round: bool,
}

impl Colony {
    pub fn default(nodes: Vec<(u32, u32)>) -> Self {
        let mut colony = Colony {
            nodes,
            start: 0,
            ant_count: 50,
            alpha: 0.5,
            beta: 1.2,
            pheromone_evaporation: 0.40,
            pheromone_value: 1000.,
            iterations: 80,
            shortest_distance: 0.0,
            shortest_path: Vec::new(),
            ants: Vec::new(),
            pheromone_map: HashMap::new(),
            ant_pheromone_map: HashMap::new(),
            first_round: true,
        };
        colony.init_ants();
        colony
    }
    pub fn mainloop(&mut self) {
        let mut rng = thread_rng();
        for ant in self.ants.iter_mut() {
            ant.run(&mut rng, &self.nodes);
        }
        for anti in 0..self.ants.len() {
            self.populate_ant_pheromone(&anti);
            if self.shortest_distance == 0.0 {
                self.shortest_distance = self.ants[anti].distance_travelled;
            }
            if self.shortest_path.is_empty() {
                self.shortest_path = self.ants[anti].route.clone();
            }
            if self.ants[anti].distance_travelled < self.shortest_distance {
                self.shortest_distance = self.ants[anti].distance_travelled;
                self.shortest_path = self.ants[anti].route.clone();
            }
        }
        self.update_pheromone();
        if self.first_round {
            self.first_round = false;
        }
        self.init_ants();
        self.ant_pheromone_map = HashMap::new();
    }
    fn update_pheromone(&mut self) {
        for v in self.pheromone_map.values_mut() {
            *v *= 1.0 - self.pheromone_evaporation;
        }
        for (k, v) in self.ant_pheromone_map.iter() {
            self.pheromone_map.entry(*k).and_modify(|x| { *x += v }).or_insert(*v);
        }
    }
    fn populate_ant_pheromone(&mut self, ant: &usize) {
        let route = self.ants[*ant].route.clone();
        for i in 0..route.len() - 1 {
            let current_pheromone = match self.ant_pheromone_map.get(&(route[i], route[i + 1])) {
                Some(x) => *x,
                None => 0.0,
            };
            let new_pheromone = self.pheromone_value / self.ants[*ant].distance_travelled;
            self.ant_pheromone_map
                .insert((route[i], route[i + 1]), current_pheromone + new_pheromone);
            self.ant_pheromone_map
                .insert((route[i + 1], route[i]), current_pheromone + new_pheromone);
        }
    }
    fn init_ants(&mut self) {
        if self.first_round {
            for _ in 0..self.ant_count {
                self.ants.push(Ant::new(
                    self.start,
                    self.alpha,
                    self.beta,
                    true,
                    (0..(self.nodes.len() as u32)).collect::<Vec<u32>>(),
                    self.pheromone_map.clone(),
                ));
            }
        } else {
            for i in 0..self.ant_count {
                self.ants[i as usize] = Ant::new(
                    self.start,
                    self.alpha,
                    self.beta,
                    false,
                    (0..(self.nodes.len() as u32)).collect::<Vec<u32>>(),
                    self.pheromone_map.clone(),
                );
            }
        }
    }
}
