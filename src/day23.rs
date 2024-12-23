use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

#[derive(Debug, Clone, Default)]
struct Graph<'s> {
    adjacency_list: FnvHashMap<&'s str, FnvHashSet<&'s str>>,
}

impl<'s> Graph<'s> {
    fn add_edge(&mut self, src: &'s str, dst: &'s str) {
        self.adjacency_list.entry(src).or_default().insert(dst);
        self.adjacency_list.entry(dst).or_default().insert(src);
    }

    fn all_sets_of_k3(&self) -> FnvHashSet<(&'s str, &'s str, &'s str)> {
        let mut result = FnvHashSet::default();
        for (src, adj) in self.adjacency_list.iter() {
            self.find_k3(src, adj, &mut result);
        }

        result
    }

    fn find_k3(
        &self,
        src: &'s str,
        adj: &FnvHashSet<&'s str>,
        k3_set: &mut FnvHashSet<(&'s str, &'s str, &'s str)>,
    ) {
        for (a, b) in adj.iter().tuple_combinations() {
            if self.connected(a, b) {
                k3_set.insert(in_sorted_order(src, a, b));
            }
        }
    }

    fn connected(&self, src: &str, dst: &str) -> bool {
        self.adjacency_list
            .get(src)
            .is_some_and(|adj| adj.contains(dst))
    }

    fn find_largest_clique(&self) -> FnvHashSet<&'s str> {
        let mut max_found = FnvHashSet::default();

        self.bron_kerbosch(
            &mut FnvHashSet::default(),
            &mut self.adjacency_list.keys().copied().collect(),
            &mut FnvHashSet::default(),
            &mut max_found,
        );

        max_found
    }

    fn bron_kerbosch(
        &self,
        current_clique: &mut FnvHashSet<&'s str>,
        potential: &mut FnvHashSet<&'s str>,
        excluded: &mut FnvHashSet<&'s str>,
        max_clique_found: &mut FnvHashSet<&'s str>,
    ) {
        if potential.is_empty() && excluded.is_empty() {
            if max_clique_found.len() < current_clique.len() {
                max_clique_found.clone_from(current_clique);
                return;
            }
        }

        while let Some(&v) = potential.iter().next() {
            let added = current_clique.insert(v);

            let mut new_potential = potential
                .intersection(
                    self.adjacency_list
                        .get(v)
                        .expect("Adjacency list not found?"),
                )
                .copied()
                .collect::<FnvHashSet<_>>();

            let mut new_excluded = excluded
                .intersection(
                    self.adjacency_list
                        .get(v)
                        .expect("Adjacency list not found?"),
                )
                .copied()
                .collect::<FnvHashSet<_>>();

            self.bron_kerbosch(
                current_clique,
                &mut new_potential,
                &mut new_excluded,
                max_clique_found,
            );

            if added {
                current_clique.remove(v);
            }

            potential.remove(v);
            excluded.insert(v);
        }
    }
}

impl std::fmt::Display for Graph<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "graph {{\n    layout=\"neato\"\n")?;

        for &vertex in self.adjacency_list.keys() {
            writeln!(f, "    {} [label={:?}]", vertex, vertex)?;
        }

        writeln!(f)?;

        for (&src, dests) in &self.adjacency_list {
            for &dst in dests {
                if src < dst {
                    writeln!(f, "    {} -- {}", src, dst)?;
                }
            }
        }

        writeln!(f, "}}")
    }
}

fn in_sorted_order<T: Ord + Clone>(a: T, b: T, c: T) -> (T, T, T) {
    let mut sorted = vec![a, b, c];
    sorted.sort_unstable();

    (sorted[0].clone(), sorted[1].clone(), sorted[2].clone())
}

fn parse(input: &str) -> Graph<'_> {
    let mut graph = Graph::default();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (src, dst) = line
            .split_once('-')
            .expect("Could not split connection on '-'");
        graph.add_edge(src, dst);
    }

    graph
}

fn keep_only_sets_with_node_starting_with_t<'s>(
    mut sets_of_k3: FnvHashSet<(&'s str, &'s str, &'s str)>,
) -> FnvHashSet<(&'s str, &'s str, &'s str)> {
    sets_of_k3
        .retain(|set| set.0.starts_with('t') || set.1.starts_with('t') || set.2.starts_with('t'));

    sets_of_k3
}

#[aoc(day23, part1)]
fn part1(input: &str) -> usize {
    let graph = parse(input);
    let k3 = graph.all_sets_of_k3();
    keep_only_sets_with_node_starting_with_t(k3).len()
}

#[aoc(day23, part2)]
fn part2(input: &str) -> String {
    let graph = parse(input);
    let mut largest = graph.find_largest_clique().into_iter().collect_vec();
    largest.sort_unstable();

    let mut result = largest[0].to_owned();
    for s in largest.into_iter().skip(1) {
        result.push(',');
        result.push_str(s);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn part1_example() {
        let graph = parse(EXAMPLE);
        let sets_of_k3 = graph.all_sets_of_k3();
        assert_eq!(sets_of_k3.len(), 12);

        let starting_with_t = keep_only_sets_with_node_starting_with_t(sets_of_k3);
        assert_eq!(starting_with_t.len(), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), "co,de,ka,ta");
    }
}
