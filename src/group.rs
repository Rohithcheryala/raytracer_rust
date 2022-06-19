use std::sync::{Arc, RwLock};

use crate::{
    body::{Body, Intersectable},
    intersections::Intersections,
    matrix::Matrix,
    ray::Ray,
};

#[derive(Clone, Debug)]
pub enum BodyOrGroup {
    Body(Body),
    Group(Arc<RwLock<Group>>),
}

#[derive(Clone, Debug)]
pub struct Group {
    parent: Option<Arc<RwLock<Group>>>,
    transform: Matrix<4>,
    items: Vec<BodyOrGroup>,
}

impl Group {
    pub fn new(transform: Matrix<4>, items: Vec<BodyOrGroup>) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            transform,
            items,
            parent: None,
        }))
    }

    pub fn transform(selff: &Arc<RwLock<Group>>) -> Matrix<4> {
        if let Some(par) = &selff.read().unwrap().parent {
            return Group::transform(&par).inverse() * selff.read().unwrap().transform;
        }
        selff.read().unwrap().transform
    }

    pub fn add_shape(mut body: Body, parent: &Arc<RwLock<Group>>) {
        match &mut body {
            Body::Sphere(s) => s.parent = Some(Arc::clone(parent)),
            Body::Plane(p) => p.parent = Some(Arc::clone(parent)),
            Body::Cube(c) => c.parent = Some(Arc::clone(parent)),
            Body::Cylinder(cyl) => cyl.parent = Some(Arc::clone(parent)),
            Body::DoubleCone(dc) => dc.parent = Some(Arc::clone(parent)),
        }
        // self.items.push(body);
        parent.write().unwrap().items.push(body.into());
    }

    pub fn add_group(grp: Arc<RwLock<Group>>, parent: &Arc<RwLock<Group>>) {
        grp.write().unwrap().parent = Some(Arc::clone(parent));
        parent.write().unwrap().items.push(grp.into());
    }

    pub fn intersect(selff: &Arc<RwLock<Group>>, ray: &Ray) -> Intersections {
        let mut xs = Intersections::default();
        for item in selff.as_ref().read().unwrap().items.iter() {
            xs.extend(item.intersect(ray));
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }
}

impl BodyOrGroup {
    pub fn intersect(&self, ray: &Ray) -> Intersections {
        match self {
            BodyOrGroup::Body(body) => body.intersect(ray),
            BodyOrGroup::Group(group) => Group::intersect(group, ray),
        }
    }
}

impl From<Body> for BodyOrGroup {
    fn from(body: Body) -> Self {
        BodyOrGroup::Body(body)
    }
}

impl From<Arc<RwLock<Group>>> for BodyOrGroup {
    fn from(group: Arc<RwLock<Group>>) -> Self {
        BodyOrGroup::Group(group)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        group::Group,
        material::{Material, Phong},
        matrix::Matrix,
        sphere::Sphere,
        tuple::Tuple,
    };

    #[test]
    fn init_works() {
        let group = Group::new(Matrix::Identity(), Vec::new());
        // assert_eq!(s.borrow(), 3);
        // (*group.as_ref().borrow_mut()).add_shape(Sphere::new(
        //     Matrix::Identity(),
        //     Material::new(
        //         Phong::new(
        //             Tuple::Point(0.0, 0.0, 0.0),
        //             Tuple::Point(0.0, 0.0, 0.0),
        //             Tuple::Point(0.0, 0.0, 0.0),
        //             0.0,
        //         ),
        //     ),
        // ));
        Group::add_shape(
            Sphere::new(
                Matrix::Translation(0, 0, -3),
                Material::Phong(Phong::default()),
            )
            .into(),
            &group,
        );
        Group::add_shape(
            Sphere::new(
                Matrix::Translation(0, 0, 5),
                Material::Phong(Phong::default()),
            )
            .into(),
            &group,
        );
        let _ray = Ray::new(Tuple::Point(0, 0, -5), Tuple::Vector(0, 0, 1));
        // let xs = group.intersect(&ray);
        // assert_eq!(
        //     xs[0].body,
        //     Body::from(Sphere::new(
        //         Matrix::Translation(0, 0, -3),
        //         Material::Phong(Phong::default()),
        //     ))
        // );
        // assert_eq!(group.items.len(), 2);
    }

    #[test]
    fn test2() {
        let grp = Group::new(Matrix::Scaling(2, 2, 2), vec![]);
        let s = Sphere::new(
            Matrix::Translation(5, 0, 0),
            Material::Phong(Default::default()),
        );
        Group::add_shape(s.into(), &grp);
        let ray = Ray::new(Tuple::Point(10, 0, -10), Tuple::Vector(0, 0, 1));
        let xs = Group::intersect(&grp, &ray);
        assert_eq!(xs.count(), 2);
    }

    #[test]
    fn test3() {
        let g1 = Group::new(Matrix::rotation_Y(std::f64::consts::FRAC_PI_2), vec![]);
        let g2 = Group::new(Matrix::Scaling(1, 2, 3), vec![]);
        let s = Sphere::new(
            Matrix::Translation(5, 0, 0),
            Material::Phong(Default::default()),
        );
        Group::add_shape(s.into(), &g2);
        Group::add_group(g2, &g1);
        let _n = match g1.read().unwrap().items[0].clone() {
            BodyOrGroup::Body(_) => todo!(),
            BodyOrGroup::Group(g2) => {
                let s = g2.read().unwrap().items[0].clone();
                match s {
                    BodyOrGroup::Body(b) => {
                        b.normal_at(Tuple::Vector(3.0f64.sqrt(), 3.0f64.sqrt(), 3.0f64.sqrt()))
                    }
                    BodyOrGroup::Group(_) => todo!(),
                }
            }
        };
    }
}
