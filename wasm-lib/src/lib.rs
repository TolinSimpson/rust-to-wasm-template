use wasm_bindgen::prelude::*;

#[derive(Copy, Clone)]
struct Aabb {
    min: [f32; 3],
    max: [f32; 3],
}

impl Aabb {
    fn contains_point(&self, x: f32, y: f32, z: f32) -> bool {
        x >= self.min[0] && x <= self.max[0]
            && y >= self.min[1] && y <= self.max[1]
            && z >= self.min[2] && z <= self.max[2]
    }

    fn intersects_aabb(&self, other: &Aabb) -> bool {
        !(self.max[0] < other.min[0] || self.min[0] > other.max[0]
            || self.max[1] < other.min[1] || self.min[1] > other.max[1]
            || self.max[2] < other.min[2] || self.min[2] > other.max[2])
    }

    fn intersects_sphere(&self, cx: f32, cy: f32, cz: f32, r: f32) -> bool {
        let clamped_x = cx.clamp(self.min[0], self.max[0]);
        let clamped_y = cy.clamp(self.min[1], self.max[1]);
        let clamped_z = cz.clamp(self.min[2], self.max[2]);
        let dx = cx - clamped_x;
        let dy = cy - clamped_y;
        let dz = cz - clamped_z;
        dx * dx + dy * dy + dz * dz <= r * r
    }
}

#[derive(Copy, Clone)]
struct Point {
    x: f32,
    y: f32,
    z: f32,
    id: u32,
}

struct Node {
    bounds: Aabb,
    points: Vec<Point>,
    children: Option<[Box<Node>; 8]>,
    capacity: usize,
}

impl Node {
    fn new(bounds: Aabb, capacity: usize) -> Self {
        Self {
            bounds,
            points: Vec::new(),
            children: None,
            capacity,
        }
    }

    fn subdivide(&mut self) {
        if self.children.is_some() {
            return;
        }
        let mid = [
            0.5 * (self.bounds.min[0] + self.bounds.max[0]),
            0.5 * (self.bounds.min[1] + self.bounds.max[1]),
            0.5 * (self.bounds.min[2] + self.bounds.max[2]),
        ];

        let mut children: [Box<Node>; 8] = std::array::from_fn(|i| {
            let x_hi = (i & 1) != 0;
            let y_hi = (i & 2) != 0;
            let z_hi = (i & 4) != 0;

            let min = [
                if x_hi { mid[0] } else { self.bounds.min[0] },
                if y_hi { mid[1] } else { self.bounds.min[1] },
                if z_hi { mid[2] } else { self.bounds.min[2] },
            ];
            let max = [
                if x_hi { self.bounds.max[0] } else { mid[0] },
                if y_hi { self.bounds.max[1] } else { mid[1] },
                if z_hi { self.bounds.max[2] } else { mid[2] },
            ];

            Box::new(Node::new(Aabb { min, max }, self.capacity))
        });

        let old_points = std::mem::take(&mut self.points);
        for p in old_points {
            let _ = Self::insert_into_children(&mut children, p);
        }
        self.children = Some(children);
    }

    fn insert(&mut self, p: Point) {
        if !self.bounds.contains_point(p.x, p.y, p.z) {
            return;
        }
        if let Some(children) = &mut self.children {
            let _ = Self::insert_into_children(children, p);
            return;
        }
        if self.points.len() < self.capacity {
            self.points.push(p);
        } else {
            self.subdivide();
            if let Some(children) = &mut self.children {
                let _ = Self::insert_into_children(children, p);
            }
        }
    }

    fn insert_into_children(children: &mut [Box<Node>; 8], p: Point) -> bool {
        for child in children.iter_mut() {
            if child.bounds.contains_point(p.x, p.y, p.z) {
                child.insert(p);
                return true;
            }
        }
        false
    }

    fn query_aabb(&self, range: &Aabb, out_ids: &mut Vec<u32>) {
        if !self.bounds.intersects_aabb(range) {
            return;
        }
        for p in &self.points {
            if range.contains_point(p.x, p.y, p.z) {
                out_ids.push(p.id);
            }
        }
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_aabb(range, out_ids);
            }
        }
    }

    fn query_sphere(&self, cx: f32, cy: f32, cz: f32, r: f32, out_ids: &mut Vec<u32>) {
        if !self.bounds.intersects_sphere(cx, cy, cz, r) {
            return;
        }
        let r2 = r * r;
        for p in &self.points {
            let dx = p.x - cx;
            let dy = p.y - cy;
            let dz = p.z - cz;
            if dx * dx + dy * dy + dz * dz <= r2 {
                out_ids.push(p.id);
            }
        }
        if let Some(children) = &self.children {
            for child in children.iter() {
                child.query_sphere(cx, cy, cz, r, out_ids);
            }
        }
    }

    fn len(&self) -> usize {
        let mut n = self.points.len();
        if let Some(children) = &self.children {
            for child in children.iter() {
                n += child.len();
            }
        }
        n
    }

    fn clear(&mut self) {
        self.points.clear();
        if let Some(children) = &mut self.children {
            for child in children.iter_mut() {
                child.clear();
            }
        }
        self.children = None;
    }
}

#[wasm_bindgen]
pub struct Octree {
    root: Node,
}

#[wasm_bindgen]
impl Octree {
    #[wasm_bindgen(constructor)]
    pub fn new(minx: f32, miny: f32, minz: f32, maxx: f32, maxy: f32, maxz: f32, capacity: u32) -> Octree {
        let capacity = capacity.max(1) as usize;
        let bounds = Aabb { min: [minx, miny, minz], max: [maxx, maxy, maxz] };
        Octree { root: Node::new(bounds, capacity) }
    }

    pub fn insert(&mut self, x: f32, y: f32, z: f32, id: u32) {
        let p = Point { x, y, z, id };
        self.root.insert(p);
    }

    pub fn query_aabb(&self, minx: f32, miny: f32, minz: f32, maxx: f32, maxy: f32, maxz: f32) -> Vec<u32> {
        let range = Aabb { min: [minx, miny, minz], max: [maxx, maxy, maxz] };
        let mut out = Vec::new();
        self.root.query_aabb(&range, &mut out);
        out
    }

    pub fn query_sphere(&self, cx: f32, cy: f32, cz: f32, r: f32) -> Vec<u32> {
        let mut out = Vec::new();
        self.root.query_sphere(cx, cy, cz, r, &mut out);
        out
    }

    pub fn len(&self) -> u32 {
        self.root.len() as u32
    }

    pub fn clear(&mut self) {
        self.root.clear();
    }

    pub fn all_node_aabbs(&self) -> Vec<f32> {
        fn gather(node: &Node, out: &mut Vec<f32>) {
            out.extend_from_slice(&[
                node.bounds.min[0], node.bounds.min[1], node.bounds.min[2],
                node.bounds.max[0], node.bounds.max[1], node.bounds.max[2],
            ]);
            if let Some(children) = &node.children {
                for c in children.iter() {
                    gather(c, out);
                }
            }
        }
        let mut out = Vec::new();
        gather(&self.root, &mut out);
        out
    }

    pub fn all_points(&self) -> Vec<f32> {
        fn gather(node: &Node, out: &mut Vec<f32>) {
            for p in node.points.iter() {
                out.extend_from_slice(&[p.x, p.y, p.z]);
            }
            if let Some(children) = &node.children {
                for c in children.iter() {
                    gather(c, out);
                }
            }
        }
        let mut out = Vec::new();
        gather(&self.root, &mut out);
        out
    }
}


