use mesh::DeinterleavedIndexedMeshBuf;
use material::Material;
use std::rc::Rc;

pub struct Entity {
    pub name: String,
    /// References the one material associated with this entity, more is not permitted by now.
    /// The reference is possibly shared and the contained material may not be directly mutated.
    /// The reference itself can be set to a new material however
    pub material: Rc<Material>,
    /// The geometry of the entity, represented as an indexed triangle mesh.
    pub mesh: DeinterleavedIndexedMeshBuf
    // TODO model transform if need be
}

impl Entity {
    pub fn new<S : Into<String>>(mesh: DeinterleavedIndexedMeshBuf, name: S, material: Rc<Material>) -> Self {
        Entity {
            mesh,
            name: name.into(),
            material
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use material::MaterialBuilder;
    use mesh::Mesh;

    #[test]
    fn test_share_material() {
        // Make two entities sharing a material
        let mat = make_material();

        let mut ent1 = Entity::new(make_mesh(), "Ent1", Rc::clone(&mat));
        let mut ent2 = Entity::new(make_mesh(), "Ent2", Rc::clone(&mat));
        assert_eq!(2, ent1.mesh.triangle_count());
        assert_eq!(6, ent1.mesh.vertex_count());

        assert_eq!(3, Rc::strong_count(&mat));

        // Then, replace both with new materials
        ent1.material = make_material();
        ent2.material = make_material();

        assert_eq!(1, Rc::strong_count(&mat));
        assert_eq!(1, Rc::strong_count(&ent1.material));
        assert_eq!(1, Rc::strong_count(&ent2.material));
    }

    fn make_material() -> Rc<Material> {
        Rc::new(MaterialBuilder::new().build())
    }

    fn make_mesh() -> DeinterleavedIndexedMeshBuf {
        DeinterleavedIndexedMeshBuf {
            positions: vec![
                1.0, 1.0, 1.0,
                10.0, 10.0, 10.0,
                100.0, 100.0, 100.0,
                -1.0, -1.0, -1.0,
                -10.0, -10.0, -10.0,
                -100.0, -100.0, -100.0
            ],
            normals: vec![
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                1.0, 0.0, 0.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
                0.0, 0.0, 1.0,
            ],
            texcoords: vec![
                0.0, 0.0,
                1.0, 1.0,
                0.0, 1.0,
                0.7, 0.7,
                0.7, 0.7,
                0.7, 0.7
            ],
            indices: vec![
                3, 4, 5,
                0, 1, 2
            ]
        }
    }
}

/*/// An entity with associated triangles.
pub trait Entity {
    type Mesh : IntoIterator<Item = T>;

    fn name() -> String;
    fn mesh() -> &Self::Mesh;
}
*/
