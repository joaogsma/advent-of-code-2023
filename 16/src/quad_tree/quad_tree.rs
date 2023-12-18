use std::cmp::Ordering;
use std::collections::{HashMap, BTreeMap};

use crate::EPSILON;
use crate::geometry::{Point, Ray, Bounds};
use super::node::{Node, NodeContent};
use super::Positioned2D;

#[derive(Debug)]
pub struct QuadTree<'t, T: Positioned2D> {
    nodes: HashMap<u32, Node<'t, T>>,
    max_fill: usize
}

impl <'t, T: 't + Positioned2D> QuadTree<'t, T> {
    const ROOT: u32 = 0;

    pub fn from_bulk<'a>(items: &'a Vec<T>, max_fill: usize) -> QuadTree<'a, T> {
        let mut tree = QuadTree {
            nodes: HashMap::from([(Self::ROOT, Self::init_root(&items))]),
            max_fill
        };
        items.iter().for_each(|item| { tree.insert(Self::ROOT, item); } );
        tree
    }

    fn init_root<'a>(points: &'a Vec<T>) -> Node<'a, T> {
        let min_x: f32 = points.iter().map(|item| item.position()).fold(f32::MAX, |acc, p| acc.min(p.0));
        let max_x: f32 = points.iter().map(|item| item.position()).fold(f32::MIN, |acc, p| acc.max(p.0));
        let min_y: f32 = points.iter().map(|item| item.position()).fold(f32::MAX, |acc, p| acc.min(p.1));
        let max_y: f32 = points.iter().map(|item| item.position()).fold(f32::MIN, |acc, p| acc.max(p.1));
        let bounds = Bounds::from_two(&Point(min_x, min_y), &Point(max_x, max_y));
        Node::<T> { bounds, content: NodeContent::Items(Vec::new()), parent_id: None }
    }

    fn insert(&mut self, node_id: u32, item: &'t T) -> bool {
        let node: &Node<T> = self.nodes.get(&node_id).expect("Node should exist");
        if !node.bounds.contains(&item.position()) {
            return false;
        }
        let children =
            match &node.content {
                NodeContent::Items(items) => {
                    if items.len() < self.max_fill {
                        let mut new_items = items.clone();
                        new_items.push(item);
                        let new_node = Node { content: NodeContent::Items(new_items), ..*node };
                        self.nodes.insert(node_id, new_node);
                        return true;
                    }
                    self.split_node(node_id);
                    return self.insert(node_id, item);
                }
                NodeContent::Children { top_left_id, top_right_id, bottom_right_id, bottom_left_id } => {
                    [*top_left_id, *top_right_id, *bottom_right_id, *bottom_left_id]
                }
            };
        children.iter().fold(false, |acc, child| self.insert(*child, item) || acc)
    }

    fn split_node(&mut self, node_id: u32) {
        let top_left_id = self.nodes.len() as u32;
        let top_right_id = top_left_id + 1;
        let bottom_right_id = top_left_id + 2;
        let bottom_left_id = top_left_id + 3;
        
        let node: Node<T> = self.nodes.remove(&node_id).expect("Node should exist");
        let parent_id = node.parent_id;
        let bounds = node.bounds;
        let items: Vec<&T> = node.get_items();
        let middle = bounds.bottom_left + ((bounds.top_right - bounds.bottom_left) / 2.0);

        let top_left_bounds = Bounds::from_two(&Point(bounds.bottom_left.0, bounds.top_right.1), &middle);
        let top_right_bounds = Bounds::from_two(&middle, &bounds.top_right);
        let bottom_right_bounds = Bounds::from_two(&middle, &Point(bounds.top_right.0, bounds.bottom_left.1));
        let bottom_left_bounds = Bounds::from_two(&bounds.bottom_left, &middle);

        let top_left_items: Vec<&T> =
            items.iter()
                .filter(|item| top_left_bounds.contains(&item.position()))
                .cloned()
                .collect();
        let top_right_items: Vec<&T> =
            items.iter()
                .filter(|item| top_right_bounds.contains(&item.position()))
                .cloned()
                .collect();
        let bottom_right_items: Vec<&T> =
            items.iter()
                .filter(|item| bottom_right_bounds.contains(&item.position()))
                .cloned()
                .collect();
        let bottom_left_items: Vec<&T> =
            items.iter()
                .filter(|item| bottom_left_bounds.contains(&item.position()))
                .cloned()
                .collect();

        let top_left = Node::<T> {
            parent_id: Some(node_id),
            content: NodeContent::Items(top_left_items),
            bounds: top_left_bounds
        };
        let top_right = Node::<T> {
            parent_id: Some(node_id),
            content: NodeContent::Items(top_right_items),
            bounds: top_right_bounds
        };
        let bottom_right = Node::<T> {
            parent_id: Some(node_id),
            content: NodeContent::Items(bottom_right_items),
            bounds: bottom_right_bounds
        };
        let bottom_left = Node::<T> {
            parent_id: Some(node_id),
            content: NodeContent::Items(bottom_left_items),
            bounds: bottom_left_bounds
        };
        let updated_node = Node {
            content: NodeContent::Children::<T> { top_left_id, top_right_id, bottom_right_id, bottom_left_id },
            bounds,
            parent_id
        };

        self.nodes.insert(top_left_id, top_left);
        self.nodes.insert(top_right_id, top_right);
        self.nodes.insert(bottom_right_id, bottom_right);
        self.nodes.insert(bottom_left_id, bottom_left);
        self.nodes.insert(node_id, updated_node);
    }

    pub fn intersect(&self, ray: &Ray, ignore_ray_origin: bool) -> Option<&T> {
        let root = self.nodes.get(&Self::ROOT).unwrap();
        let mut queue: BTreeMap<u64, Vec<&Node<T>>> = BTreeMap::from([(0_u64, vec![root])]);
        let mut best_hit: Option<(u64, &T)> = None;

        while !queue.is_empty() {
            let (priority, closest_nodes) = queue.pop_first().unwrap();
            if best_hit.is_some_and(|(best_priority, _)| best_priority < priority) {
                break;
            }
            let hit: Option<(u64, &T)> =
                closest_nodes.iter()
                    .filter_map(|node| self.visit_node(ray, node, &mut queue, ignore_ray_origin))
                    .min_by(|item1, item2| Self::cmp_closest(ray, item1, item2))
                    .map(|hit| {
                        let hit_priority = Self::to_priority(hit.position().distance(&ray.0));
                        (hit_priority, hit)
                    });
            match (&hit, &best_hit) {
                (None, _) => {}
                (Some(_), None) => best_hit = hit,
                (Some((chp, _)), Some((bhp, _))) => best_hit = if chp < bhp { hit } else { best_hit }
            }
        }
        return best_hit.map(|(_, bh)| bh);
    }

    fn visit_node<'a: 't>(
        &'a self,
        ray: &Ray,
        node: &Node<'t, T>,
        queue: &mut BTreeMap<u64, Vec<&'a Node<'t, T>>>,
        ignore_ray_origin: bool) -> Option<&'t T> {
        match &node.content {
            NodeContent::Items(items) => return self.visit_leaf(ray, items, ignore_ray_origin),
            NodeContent::Children { top_left_id, top_right_id, bottom_right_id, bottom_left_id } => {
                self.enqueue_children(ray, &[*top_left_id, *top_right_id, *bottom_left_id, *bottom_right_id], queue);
                None
            }
        }
    }

    fn visit_leaf(&self, ray: &Ray, items: &Vec<&'t T>, ignore_ray_origin: bool) -> Option<&'t T> {
        return items.iter()
            .filter(|item| ray.hits(&item.position()))
            .filter(|item| !ignore_ray_origin || item.position().distance(&ray.0) > EPSILON)
            .min_by(|item1, item2| Self::cmp_closest(ray, item1, item2))
            .map(|item| *item);
    }

    fn enqueue_children<'a>(&'a self, ray: &Ray, child_ids: &[u32], queue: &mut BTreeMap<u64, Vec<&'a Node<'t, T>>>) {
        let children_with_priority =
            child_ids.iter().filter_map(|child_id| {
                let child: &Node<'t, T> = self.nodes.get(child_id).unwrap();
                match ray.intersect_bounds(&child.bounds) {
                    None => None,
                    Some(intersection) => {
                        let priority = Self::to_priority(intersection.distance(&ray.0));
                        Some((child, priority))
                    }
                }
            });
        for (child, priority) in children_with_priority {
            if !queue.contains_key(&priority) {
                queue.insert(priority, vec![&child]);
            }
            queue.get_mut(&priority).unwrap().push(&child);
        }
    }

    #[allow(dead_code)]
    pub fn path_to_string(&self, target: &Point) -> Vec<String> {
        self.path_to_string_rec(Self::ROOT, target, String::from("Root"))
    }

    #[allow(dead_code)]
    fn path_to_string_rec(&self, current: u32, target: &Point, acc: String) -> Vec<String> {
        let node = self.nodes.get(&current).unwrap();
        match &node.content {
            NodeContent::Items(items) => {
                let contains = items.iter().find(|item| *item.position() == *target).is_some();
                if contains { vec![acc] } else { Vec::new() }
            }
            NodeContent::Children { top_left_id, top_right_id, bottom_right_id, bottom_left_id } => {
                [(top_left_id, "TL"), (top_right_id, "TR"), (bottom_right_id, "BR"), (bottom_left_id, "BL")].iter()
                    .flat_map(|(id, prefix)| self.path_to_string_rec(**id, target, acc.clone() + "/" + prefix))
                    .collect()
            }
        }
    }

    fn cmp_closest(ray: &Ray, item1: &T, item2: &T) -> Ordering{
        let distance1: f32 = item1.position().distance(&ray.0);
        let distance2: f32 = item2.position().distance(&ray.0);
        distance1.partial_cmp(&distance2).unwrap()
    }

    fn to_priority(value: f32) -> u64 { ((value * 10.0_f32.powi(3)).round() * 10.0) as u64 }
}