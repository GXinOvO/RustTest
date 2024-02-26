#![doc = include_str!("../README.md")]

/*
--TODO: trait
    NextAfter: 一个trait，为浮点数类型提供了next_after方法，该方法返回比给定浮点数大或小的最接近的浮点数。
*/
use float_next_after::NextAfter;
use rtree_rs::RTree;
use rtree_rs::Rect as RTreeRect;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    min: Point,
    max: Point,
}

impl Rect {
    // -> 用于检查给定的点p是否位于矩形范围内。它通过比较点的x和y坐标是否在矩形的最小值和最大值之间确定。表示点是否在矩形内部。
    pub fn contains_point(&self, p: Point) -> bool {
        return p.x >= self.min.x && p.x <= self.max.x && p.y >= self.min.y && p.y <= self.max.y;
    }

    // -> 用于检查当前矩形是否与另一个矩形相交。
    pub fn intersects_rect(&self, other: Rect) -> bool {
        if self.min.y > other.max.y || self.max.y < other.min.y {
            return false;
        }
        if self.min.x > other.max.x || self.max.x < other.min.x {
            return false;
        }
        return true;
    }

    // -> 分别返回矩形的四个角点: nw(北西)、sw(南西)、se(南东)和ne(北东)。通过使用最小值和最大值的坐标来构造一个新的Point构造体。
    pub fn nw(&self) -> Point {
        Point {
            x: self.min.x,
            y: self.max.y,
        }
    }

    pub fn sw(&self) -> Point {
        Point {
            x: self.min.x,
            y: self.min.y,
        }
    }

    pub fn se(&self) -> Point {
        Point {
            x: self.max.x,
            y: self.min.y,
        }
    }

    pub fn ne(&self) -> Point {
        Point {
            x: self.max.x,
            y: self.max.y,
        }
    }

    // -> 返回矩形的四条边
    pub fn south(&self) -> Segment {
        Segment {
            a: self.sw(),
            b: self.se(),
        }
    }

    pub fn east(&self) -> Segment {
        Segment {
            a: self.se(),
            b: self.ne(),
        }
    }

    pub fn north(&self) -> Segment {
        Segment {
            a: self.ne(),
            b: self.nw(),
        }
    }

    pub fn west(&self) -> Segment {
        Segment {
            a: self.nw(),
            b: self.sw(),
        }
    }

    // -> 根据给定的索引返回矩形的边。
    pub fn segment_at(&self, index: i64) -> Segment {
        match index {
            0 => return self.south(),
            1 => return self.east(),
            2 => return self.north(),
            3 => return self.west(),
            _ => return self.south(), // TODO(ringsaturn): raise err
        }
    }
}

/*
--TODO:
    根据给定的索引从一个带你的向量中获取两个点，并使用这两个点构造一个线段Segment。
*/
fn segment_at_for_vec_point(exterior: &Vec<Point>, index: i64) -> Segment {
    // -> 通过索引从exterior向量中获取seg_a点。
    let seg_a: Point = exterior[index as usize];
    
    // -> 根据索引计算seg_b_index的值，如果seg_b_index等于exterior向量的最后一个索引，就将其设置为0，否则将其递增1.
    let mut seg_b_index = index;
    if seg_b_index == (exterior.len() - 1) as i64 {
        seg_b_index = 0
    } else {
        seg_b_index += 1
    }

    // -> 根据seg_b_index从exterior向量中获取seg_b点，然后使用这两个点构造并返回一个线段。
    let seg_b: Point = exterior[seg_b_index as usize];
    return Segment { a: seg_a, b: seg_b };
}

/*
--TODO:
    用于检查一个点是否在一个由点的向量表示的环内。
*/
fn rings_contains_point(ring: &Vec<Point>, point: Point, allow_on_edge: bool) -> bool {
    // -> 创建一个表示与point在y轴上相交的无限宽度矩形rect。
    let rect = Rect {
        min: Point {
            x: std::f64::NEG_INFINITY,
            y: point.y,
        },
        max: Point {
            x: std::f64::INFINITY,
            y: point.y,
        },
    };
    // -> 记录点是否在环内部。
    let mut inside: bool = false;
    let n: i64 = (ring.len() - 1) as i64;

    // -> 通过迭代环内的所有线段，并检查每个线段是否与矩形rect相交。如果相交，则使用raycast函数对点和线段进行光线投射。
    for i in 0..n {
        let seg: Segment = segment_at_for_vec_point(&ring, i);

        if seg.rect().intersects_rect(rect) {
            let res: RaycastResult = raycast(&seg, point);
            // print!("res= inside:{:?} on:{:?}\n", res.inside, res.on);
            // -> 如果投射结果中的on字段为真，则根据allow_on_edge变量决定是否将inside设置为真，并跳出循环。
            if res.on {
                inside = allow_on_edge;
                break;
            }
            // -> 如果inside为真，则将inside取反。
            if res.inside {
                inside = !inside;
            }
        }
    }
    return inside;
}

/*
--TODO:
    使用R树数据结构来判断点是否在一个环内。
*/
fn rings_contains_point_by_rtree_index(
    ring: &Vec<Point>,
    ring_rtree: &rtree_rs::RTree<2, f64, i64>,
    point: Point,
    allow_on_edge: bool,
) -> bool {
    // -> 创建了一个表示与point在y轴上相交的无限宽度矩形rect。
    let rect = Rect {
        min: Point {
            x: std::f64::NEG_INFINITY,
            y: point.y,
        },
        max: Point {
            x: std::f64::INFINITY,
            y: point.y,
        },
    };
    // -> 在R树上搜索与rect相交的项来确定与point相关的线段。
    for item in ring_rtree.search(RTreeRect::new(
        [std::f64::NEG_INFINITY, point.y],
        [std::f64::INFINITY, point.y],
    )) {
        // -> 对于每个相交的项，使用segment_at_for_vec_point根据索引从ring中获取相应的线段seg，并获取改线段的矩形irect。
        let seg: Segment = segment_at_for_vec_point(&ring, *item.data);
        let irect = seg.rect();
        // -> 检查irect是否与rect相交
        if irect.intersects_rect(rect) {
            // -> 如果相交，则使用raycast对点和线段进行光线投射。
            let res: RaycastResult = raycast(&seg, point);
            if res.on {
                return allow_on_edge;
            }
            if res.inside {
                return true;
            }
        }
    }
    return false;
}
/*
--TODO: Polygon
    exterior: 一个存储外环点的向量
    exterior_rtree: 一个R树实例，用于存储外环的空间信息
    holes: 一个存储内环点集的向量，每个内环有一个点的向量表示
    holes_rtree: 一个存储内环R树实例的向量，每个实例用于存储对应内环的空间信息
    rect: 一个矩形，用于表示整个多边形的边界范围
    with_index: 用于指示是否使用索引进行查询
*/
pub struct Polygon {
    exterior: Vec<Point>,
    exterior_rtree: rtree_rs::RTree<2, f64, i64>,
    holes: Vec<Vec<Point>>,
    holes_rtree: Vec<rtree_rs::RTree<2, f64, i64>>,
    pub rect: Rect,
    with_index: bool,
}

impl Polygon {
    /// Point-In-Polygon check with RTree index.
    /// `with_index` param must true for this query.
    /// Query speed is faster compare with [contains_point_normal].
    fn contains_point_with_index(&self, p: Point) -> bool {
        // -> 调用rings_contains_point_by_rtree_index，检查待判断的点p是否在外环内。
        if !rings_contains_point_by_rtree_index(&self.exterior, &self.exterior_rtree, p, false) {
            return false;
        }

        let mut contains: bool = false;
        let mut i: usize = 0;
        // -> 遍历内环的向量holes。对每个内环使用对应的R树holes_rtree进行点的判断。
        for hole in self.holes.iter() {
            let tr = self.holes_rtree.get(i).unwrap();
            if rings_contains_point_by_rtree_index(&hole, &tr, p, false) {
                contains = true;
                break;
            }

            i += 1;
        }
        return contains;
    }

    /// Point-In-Polygon check, the normal way.
    /// It's most used algorithm implementation, port from Go's [geojson]
    ///
    /// [geojson]: https://github.com/tidwall/geojson
    /*
    --TODO:
        用于使用普通的点在多边形内判断算法来判断点p是否在多边形内部
     */
    fn contains_point_normal(&self, p: Point) -> bool {
        if !rings_contains_point(&self.exterior, p, false) {
            return false;
        }
        let mut contains: bool = true;
        for hole in self.holes.iter() {
            if rings_contains_point(&hole, p, false) {
                contains = false;
                break;
            }
        }
        return contains;
    }

    /// Do point-in-polygon search.
    /*
    --TODO:
        对点在多边形内判断的统一入口
     */
    pub fn contains_point(&self, p: Point) -> bool {
        if !self.rect.contains_point(p) {
            println!("=== contains_point ===");
            return false;
        }
        if self.with_index {
            return self.contains_point_with_index(p);
        }
        return self.contains_point_normal(p);
    }

    /// Create a new Polygon instance from exterior and holes.
    ///
    /// Please note that set `with_index` to true will increase performance, but requires more memory.
    /// See [#4] for more details.
    ///
    /// [#4]: https://github.com/ringsaturn/geometry-rs/pull/4
    ///
    /// Example:
    ///
    /// ```rust
    /// use std::vec;
    /// use geometry_rs;
    /// let poly = geometry_rs::Polygon::new(
    ///     vec![
    ///         geometry_rs::Point {
    ///             x: 90.48826291293898,
    ///             y: 45.951129815858565,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 90.48826291293898,
    ///             y: 27.99437617512571,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 122.83201291294,
    ///             y: 27.99437617512571,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 122.83201291294,
    ///             y: 45.951129815858565,
    ///         },
    ///         geometry_rs::Point {
    ///             x: 90.48826291293898,
    ///             y: 45.951129815858565,
    ///         },
    ///     ],
    ///     vec![],
    /// );
    ///
    /// let p_out = geometry_rs::Point {
    ///     x: 130.74216916294148,
    ///     y: 37.649011392900306,
    /// };
    ///
    /// print!("{:?}\n", poly.contains_point(p_out));
    ///
    /// let p_in = geometry_rs::Point {
    ///     x: 99.9804504129416,
    ///     y: 39.70716466970461,
    /// };
    /// print!("{:?}\n", poly.contains_point(p_in));
    /// ```
    pub fn new(exterior: Vec<Point>, holes: Vec<Vec<Point>>) -> Polygon {
        return Polygon::new_with_rtree_index_opt(exterior, holes, false);
    }

    /*--TODO: 用于指示否是使用索引 */
    pub fn new_with_rtree_index_opt(
        exterior: Vec<Point>,
        holes: Vec<Vec<Point>>,
        with_index: bool,
    ) -> Polygon {
        // -> 计算多边形的边界矩形.
        let mut minx: f64 = exterior.get(0).unwrap().x;
        let mut miny: f64 = exterior.get(0).unwrap().y;
        let mut maxx: f64 = exterior.get(0).unwrap().x;
        let mut maxy: f64 = exterior.get(0).unwrap().y;

        // for p in exterior.iter() {
        for i in 0..exterior.len() - 1 {
            let p = exterior[i];
            if p.x < minx {
                minx = p.x;
            }
            if p.y < miny {
                miny = p.y;
            }
            if p.x > maxx {
                maxx = p.x;
            }
            if p.y > maxy {
                maxy = p.y;
            }
        }

        // println!("minx: {}, miny: {}, maxx: {}, maxy: {}", minx, miny, maxx, maxy);

        let rect = Rect {
            min: Point { x: minx, y: miny },
            max: Point { x: maxx, y: maxy },
        };

        // -> 创建一个空的R树用于存储外环的空间信息
        let mut exterior_rtree = RTree::new();
        let n = (exterior.len() - 1) as i64;
        for i in 0..n {
            let segrect = segment_at_for_vec_point(&exterior, i).rect();
            // println!("{i} segrect: {:#?}", segrect);
            if with_index {
                exterior_rtree.insert(
                    RTreeRect::new(
                        [segrect.min.x, segrect.min.y],
                        [segrect.max.x, segrect.max.y],
                    ),
                    i as i64,
                );
            }
        }

        let mut holes_rtree = vec![];
        for hole_poly in holes.iter() {
            println!("===== hole_poly =====");
            let mut hole_rtre = RTree::new();
            let n = (hole_poly.len() - 1) as i64;
            for i in 0..n {
                let segrect = segment_at_for_vec_point(&hole_poly, i).rect();
                if with_index {
                    hole_rtre.insert(
                        RTreeRect::new(
                            [segrect.min.x, segrect.min.y],
                            [segrect.max.x, segrect.max.y],
                        ),
                        i as i64,
                    );
                }
            }
            if with_index {
                holes_rtree.push(hole_rtre);
            }
        }

        return Polygon {
            exterior,
            exterior_rtree,
            holes,
            holes_rtree,
            rect,
            with_index,
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    a: Point,
    b: Point,
}

impl Segment {
    pub fn rect(&self) -> Rect {
        let mut min_x: f64 = self.a.x;
        let mut min_y: f64 = self.a.y;
        let mut max_x: f64 = self.b.x;
        let mut max_y: f64 = self.b.y;

        if min_x > max_x {
            let actual_min_x = max_x;
            let actual_max_x = min_x;
            min_x = actual_min_x;
            max_x = actual_max_x;
        }

        if min_y > max_y {
            let actual_min_y = max_y;
            let actual_max_y = min_y;
            min_y = actual_min_y;
            max_y = actual_max_y;
        }

        return Rect {
            min: Point { x: min_x, y: min_y },
            max: Point { x: max_x, y: max_y },
        };
    }
}

pub struct RaycastResult {
    inside: bool, // point on the left
    on: bool,     // point is directly on top of
}

/*
--TODO:
    用于进行射线与线段的相交检测。
*/
pub fn raycast(seg: &Segment, point: Point) -> RaycastResult {
    let mut p = point;
    let a = seg.a;
    let b = seg.b;

    // make sure that the point is inside the segment bounds
    if a.y < b.y && (p.y < a.y || p.y > b.y) {
        return RaycastResult {
            inside: false,
            on: false,
        };
    } else if a.y > b.y && (p.y < b.y || p.y > a.y) {
        return RaycastResult {
            inside: false,
            on: false,
        };
    }

    // test if point is in on the segment
    if a.y == b.y {
        if a.x == b.x {
            if p.x == a.x && p.y == a.y {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
        if p.y == b.y {
            // horizontal segment
            // check if the point in on the line
            if a.x < b.x {
                if p.x >= a.x && p.x <= b.x {
                    return RaycastResult {
                        inside: false,
                        on: true,
                    };
                }
            } else {
                if p.x >= b.x && p.x <= a.x {
                    return RaycastResult {
                        inside: false,
                        on: true,
                    };
                }
            }
        }
    }
    if a.x == b.x && p.x == b.x {
        // vertical segment
        // check if the point in on the line
        if a.y < b.y {
            if p.y >= a.y && p.y <= b.y {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
        } else {
            if p.y >= b.y && p.y <= a.y {
                return RaycastResult {
                    inside: false,
                    on: true,
                };
            }
        }
    }
    if (p.x - a.x) / (b.x - a.x) == (p.y - a.y) / (b.y - a.y) {
        return RaycastResult {
            inside: false,
            on: true,
        };
    }

    // do the actual raycast here.
    while p.y == a.y || p.y == b.y {
        // p.y = NextAfter(p.y, &std::f64::INFINITY)
        // let next = big_num.next_after(&std::f64::INFINITY);
        p.y = p.y.next_after(std::f64::INFINITY);
    }

    if a.y < b.y {
        if p.y < a.y || p.y > b.y {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
    } else {
        if p.y < b.y || p.y > a.y {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
    }
    if a.x > b.x {
        if p.x >= a.x {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
        if p.x <= b.x {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    } else {
        if p.x >= b.x {
            return RaycastResult {
                inside: false,
                on: false,
            };
        }
        if p.x <= a.x {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    }
    if a.y < b.y {
        if (p.y - a.y) / (p.x - a.x) >= (b.y - a.y) / (b.x - a.x) {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    } else {
        if (p.y - b.y) / (p.x - b.x) >= (a.y - b.y) / (a.x - b.x) {
            return RaycastResult {
                inside: true,
                on: false,
            };
        }
    }
    return RaycastResult {
        inside: false,
        on: false,
    };
}
