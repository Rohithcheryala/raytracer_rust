use crate::{
    body::{Body, Intersectable},
    intersections::Intersections,
    matrix::Matrix,
    ray::Ray,
};

#[derive(Clone, Debug)]
pub enum BodyOrGroup {
    Body(Body),
    Group(Group),
}

#[derive(Clone, Debug)]
pub struct Group {
    transform: Matrix<4>,
    items: Vec<BodyOrGroup>,
}

pub struct GroupBuilder {
    transform: Matrix<4>,
    items: Vec<BodyOrGroup>,
}

impl Group {
    pub fn new(transform: Matrix<4>, items: Vec<BodyOrGroup>) -> GroupBuilder {
        GroupBuilder { transform, items }
    }

    pub fn transform(&self) -> Matrix<4> {
        self.transform
    }

    pub fn transform_mut(&mut self) -> &mut Matrix<4> {
        &mut self.transform
    }

    pub fn add_shape(&mut self, body: Body) {
        self.items.push(body.into());
    }

    pub fn add_group(&mut self, grp: Group) {
        self.items.push(grp.into());
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let mut xs = Intersections::default();
        for item in self.items.iter() {
            xs.extend(item.intersect(ray));
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }
}

impl GroupBuilder {
    pub fn new(transform: Matrix<4>, items: Vec<BodyOrGroup>) -> GroupBuilder {
        GroupBuilder { transform, items }
    }

    pub fn transform(&self) -> Matrix<4> {
        self.transform
    }

    pub fn add_shape(&mut self, body: Body) {
        self.items.push(body.into());
    }

    pub fn add_group(&mut self, grp: Group) {
        self.items.push(grp.into());
    }

    pub fn build(mut self) -> Group {
        let trans_inv = self.transform().inverse();
        self.items.iter_mut().for_each(|it| match it {
            BodyOrGroup::Body(b) => *b.transform_mut() = trans_inv * b.transform(),
            BodyOrGroup::Group(g) => *g.transform_mut() = trans_inv * g.transform(),
        });
        Group {
            transform: self.transform(),
            items: self.items,
        }
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

impl From<Group> for BodyOrGroup {
    fn from(group: Group) -> Self {
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
        let mut group = Group::new(Matrix::Identity(), Vec::new());
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
        group.add_shape(
            Sphere::new(
                Matrix::Translation(0, 0, -3),
                Material::Phong(Phong::default()),
            )
            .into(),
        );
        group.add_shape(
            Sphere::new(
                Matrix::Translation(0, 0, 5),
                Material::Phong(Phong::default()),
            )
            .into(),
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
        let mut grp = Group::new(Matrix::Scaling(2, 2, 2), vec![]);
        let s = Sphere::new(
            Matrix::Translation(5, 0, 0),
            Material::Phong(Default::default()),
        );
        grp.add_shape(s.into());
        let ray = Ray::new(Tuple::Point(10, 0, -10), Tuple::Vector(0, 0, 1));
        let xs = grp.build().intersect(&ray);
        assert_eq!(xs.count(), 2);
    }

    #[test]
    fn test3() {
        let mut g1 = Group::new(Matrix::rotation_Y(std::f64::consts::FRAC_PI_2), vec![]);
        let mut g2 = Group::new(Matrix::Scaling(1, 2, 3), vec![]);
        let s = Sphere::new(
            Matrix::Translation(5, 0, 0),
            Material::Phong(Default::default()),
        );
        g2.add_shape(s.into());
        g1.add_group(g2.build());
        let _n = match g1.items[0].clone() {
            BodyOrGroup::Body(_) => todo!(),
            BodyOrGroup::Group(g2) => {
                let s = g2.items[0].clone();
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
