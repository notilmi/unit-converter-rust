use crate::domain::conversion::Conversion;
use crate::domain::unit::Unit;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq)]
pub struct Edge {
    to: usize,
    conversion: Conversion,
}

#[derive(Debug)]
pub struct Area {
    units: Vec<Unit>,              // index -> Unit
    index: HashMap<String, usize>, // name  -> index
    graph: HashMap<usize, Vec<Edge>>,
}

impl Area {
    pub fn initialize(units: Vec<Unit>) -> Area {
        let mut index = HashMap::with_capacity(units.len());
        for (idx, u) in units.iter().enumerate() {
            index.insert(u.name.clone(), idx);
        }

        Self {
            units,
            index,
            graph: HashMap::new(),
        }
    }

    /// Adds a directed edge from `from` to `to`.
    /// Returns `true` if added, or `false` if either unit does not exist.
    pub fn add_edge(&mut self, from: &str, to: &str, conversion: Conversion) -> bool {
        let (Some(&u_from), Some(&u_to)) = (self.index.get(from), self.index.get(to)) else {
            return false;
        };

        let edges = self.graph.entry(u_from).or_default();
        if let Some(existing) = edges.iter_mut().find(|e| e.to == u_to) {
            existing.conversion = conversion;
        } else {
            edges.push(Edge {
                to: u_to,
                conversion,
            })
        }

        true
    }

    /// Adds bidirectional edges between `a` and `b`.
    pub fn add_bidirectional_edge(&mut self, a: &str, b: &str, factor: f64) -> bool {
        let forward = Conversion::new(factor);
        let reverse = forward.inverse();
        self.add_edge(a, b, forward) && self.add_edge(b, a, reverse)
    }

    pub fn get_unit(&self, name: &str) -> Option<&Unit> {
        let &i = self.index.get(name)?;
        self.units.get(i)
    }

    /// Returns direct (1-hop) neighbors of `name`.
    pub fn get_neighbor(&self, name: &str) -> Vec<&Unit> {
        let Some(&idx) = self.index.get(name) else {
            return Vec::new();
        };

        self.graph
            .get(&idx)
            .map(|neighbors| {
                neighbors
                    .iter()
                    .filter_map(|i| self.units.get(i.to))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Checks if `b` is an immediate direct (1-hop) neighbor of `a`.
    pub fn is_neighbor(&self, a: &str, b: &str) -> bool {
        let (Some(&u_a), Some(&u_b)) = (self.index.get(a), self.index.get(b)) else {
            return false;
        };

        self.graph
            .get(&u_a)
            .is_some_and(|neighbors| neighbors.iter().any(|edge| edge.to == u_b))
    }

    /// Returns all other units reachable from `name` via BFS traversal,
    /// excluding the starting unit itself.
    pub fn get_reachable(&self, name: &str) -> Vec<&Unit> {
        let Some(&start) = self.index.get(name) else {
            return Vec::new();
        };

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        let mut reachable = Vec::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if node != start
                && let Some(unit) = self.units.get(node)
            {
                reachable.push(unit);
            }

            if let Some(neighbors) = self.graph.get(&node) {
                for neighbor in neighbors {
                    if visited.insert(neighbor.to) {
                        queue.push_back(neighbor.to);
                    }
                }
            }
        }

        reachable
    }

    /// Checks if `target` is reachable from `src` (any number of hops).
    pub fn is_reachable(&self, src: &str, target: &str) -> bool {
        let (Some(&start), Some(&goal)) = (self.index.get(src), self.index.get(target)) else {
            return false;
        };

        if start == goal {
            return true;
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back(start);

        while let Some(node) = queue.pop_front() {
            if node == goal {
                return true;
            }

            if let Some(edges) = self.graph.get(&node) {
               for edge in edges {
                   if visited.insert(edge.to) {
                       queue.push_back(edge.to)
                   }
               }
            }
        }

        false
    }

    pub fn convert(&self, src: &str, target: &str, value: f64) -> Option<f64> {
        let (Some(&start), Some(&goal)) = (self.index.get(src), self.index.get(target)) else {
            return None
        };

        if start == goal {
            return Some(value);
        }

        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        visited.insert(start);
        queue.push_back((start, value));

        while let Some((node, curr_val)) = queue.pop_front() {
            if node == goal {
                return Some(curr_val)
            }

            if let Some(edges) = self.graph.get(&node) {
                for edge in edges {
                    if visited.insert(edge.to) {
                        let next_val = edge.conversion.convert(curr_val);
                        queue.push_back((edge.to, next_val))
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_area() -> Area {
        let units = vec![
            Unit {
                name: "sqm".to_string(),
            },
            Unit {
                name: "sqkm".to_string(),
            },
            Unit {
                name: "sqft".to_string(),
            },
            Unit {
                name: "acre".to_string(),
            },
        ];
        Area::initialize(units)
    }

    fn setup_area_with_edge() -> Area {
        let mut area = setup_area();
        area.add_bidirectional_edge("sqm", "sqkm", 10.0);
        area.add_edge("sqkm", "sqft", Conversion::new(10.0));

        area

    }

    #[test]
    fn test_issue_1_empty_graph_has_no_neighbors() {
        let area = setup_area();
        // Issue 1: get_neighbor should NOT return the unit itself when there are no connections
        assert!(area.get_neighbor("sqm").is_empty());
        assert!(area.get_reachable("sqm").is_empty());
    }

    #[test]
    fn test_issue_2_not_own_neighbor() {
        let area = setup_area();
        // Issue 2: A unit is not its own neighbor
        assert!(!area.is_neighbor("sqm", "sqm"));
    }

    #[test]
    fn test_issue_3_add_edge_and_query_neighbors() {
        let mut area = setup_area();

        // Issue 3: Graph can be populated
        assert!(area.add_bidirectional_edge("sqm", "sqkm", 10.0));
        assert!(area.add_edge("sqkm", "sqft", Conversion::new(10.0)));

        // Direct 1-hop checks
        assert!(area.is_neighbor("sqm", "sqkm"));
        assert!(area.is_neighbor("sqkm", "sqm"));
        assert!(area.is_neighbor("sqkm", "sqft"));
        assert!(!area.is_neighbor("sqft", "sqkm")); // directed edge
        assert!(!area.is_neighbor("sqm", "sqft")); // 2-hop, not direct neighbor

        // Reachability checks
        assert!(area.is_reachable("sqm", "sqft"));
        assert!(!area.is_reachable("sqft", "sqm"));

        // Neighbor list
        let sqm_neighbors = area.get_neighbor("sqm");
        assert_eq!(sqm_neighbors.len(), 1);
        assert_eq!(sqm_neighbors[0].name, "sqkm");
    }

    #[test]
    fn test_issue_4_conversiom() {
        let area = setup_area_with_edge();

        assert_eq!(area.convert("sqm", "sqkm", 10.0), Some(100.0));
        assert_eq!(area.convert("sqkm", "sqm", 10.0), Some(1.0));
    }
}
