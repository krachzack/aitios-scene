use super::TriangleMeshIter;
use geom::{Aabb, Position, Vec3};

/// Implemented by types that represent triangle meshes.
/// They must at least provide a method for iterating over vertices.
/// Each three successive vertices are assumed to form a triangle.
pub trait Mesh<'a> {
    type Vertex: Position;
    type VertexIter: Iterator<Item = Self::Vertex>;

    fn vertices(&'a self) -> Self::VertexIter;
    fn vertex_count(&'a self) -> usize;

    /// Yields successive triangles in the mesh.
    ///
    /// Each three consecutive vertices form a `TupleTriangle`.
    ///
    fn triangles(&'a self) -> TriangleMeshIter<Self::Vertex, Self::VertexIter> {
        TriangleMeshIter::new(self.vertices())
    }

    fn triangle_count(&'a self) -> usize {
        self.vertex_count() / 3
    }

    fn calculate_bounds(&'a self) -> Aabb {
        Aabb::from_points(self.vertices().map(|v| v.position()))
    }

    fn centroid(&'a self) -> Vec3 {
        let one_over_n = (self.vertex_count() as f32).recip();
        let vertex_sum = self.vertices().map(|v| v.position()).sum::<Vec3>();

        one_over_n * vertex_sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mesh::DeinterleavedIndexedMeshBuf;

    #[test]
    fn test_iterators() {
        let mesh = make_test_mesh();

        assert_eq!(2, mesh.triangle_count());

        let mut triangle_iter = mesh.triangles();
        assert!(triangle_iter.next().is_some());
        assert!(triangle_iter.next().is_some());
        assert!(triangle_iter.next().is_none());
        assert!(triangle_iter.next().is_none());

        mesh.triangles();

        assert_eq!(6, mesh.vertex_count());
        assert_eq!(mesh.vertices().count(), mesh.vertex_count());
    }

    fn make_test_mesh() -> DeinterleavedIndexedMeshBuf {
        DeinterleavedIndexedMeshBuf {
            positions: vec![
                1.0, 1.0, 1.0, 10.0, 10.0, 10.0, 100.0, 100.0, 100.0, -1.0, -1.0, -1.0, -10.0,
                -10.0, -10.0, -100.0, -100.0, -100.0,
            ],
            normals: vec![
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0,
            ],
            texcoords: vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            indices: vec![3, 4, 5, 0, 1, 2],
        }
    }
}
