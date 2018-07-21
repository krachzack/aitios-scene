use geom::{FromVertices, Position, TupleTriangle};

/// Iterates over a mesh where each three consecutive vertices form a triangle
pub struct TriangleMeshIter<V, I>
where
    V: Position,
    I: Iterator<Item = V>,
{
    vertex_iter: I,
}

impl<V, I> TriangleMeshIter<V, I>
where
    V: Position,
    I: Iterator<Item = V>,
{
    pub fn new(vertex_iter: I) -> Self {
        TriangleMeshIter { vertex_iter }
    }
}

impl<V, I> Iterator for TriangleMeshIter<V, I>
where
    V: Position,
    I: Iterator<Item = V>,
{
    type Item = TupleTriangle<V>;

    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(v0), Some(v1), Some(v2)) = (
            self.vertex_iter.next(),
            self.vertex_iter.next(),
            self.vertex_iter.next(),
        ) {
            Some(TupleTriangle::new(v0, v1, v2))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::super::deinterleaved::DeinterleavedIndexedMeshBuf;
    use super::super::mesh::Mesh;
    use super::*;
    use geom::{Position, Texcoords, Triangle, Vec2, Vec3};

    #[test]
    fn test_iteration_empty_mesh() {
        let buf = DeinterleavedIndexedMeshBuf {
            positions: vec![],
            normals: vec![],
            texcoords: vec![],
            indices: vec![],
        };
        let mut iter = TriangleMeshIter::new(buf.into_iter());

        assert!(iter.next().is_none());
    }

    #[test]
    fn test_iteration_normal_mesh() {
        let buf = DeinterleavedIndexedMeshBuf {
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
        };
        let mut iter = TriangleMeshIter::new((&buf).vertices());

        let tri = iter.next().unwrap();
        let (vtx0, _, _) = tri.vertices();
        let pos0 = vtx0.position();

        assert_eq!(pos0, Vec3::new(-1.0, -1.0, -1.0));

        let tri = iter.next().unwrap();
        let (_, vtx1, vtx2) = tri.vertices();
        let pos2 = vtx2.position();
        let tex1 = vtx1.texcoords();

        assert_eq!(pos2, Vec3::new(100.0, 100.0, 100.0));
        assert_eq!(tex1, Vec2::new(1.0, 1.0));
    }

    #[test]
    fn test_iteration_non_full_vtx_at_end_mesh() {
        let buf = DeinterleavedIndexedMeshBuf {
            positions: vec![
                1.0, 1.0, 1.0, 10.0, 10.0, 10.0, 100.0, 100.0, 100.0, -1.0, -1.0, -1.0, -10.0,
                -10.0, -10.0, -100.0, -100.0, -100.0,
            ],
            normals: vec![
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0,
            ],
            texcoords: vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            indices: vec![3, 4, 5, 0, 1],
        };
        let mut iter = TriangleMeshIter::new(buf.into_iter());

        let tri = iter.next().unwrap();
        let (vtx0, _, _) = tri.vertices();
        let pos0 = vtx0.position();

        assert_eq!(pos0, Vec3::new(-1.0, -1.0, -1.0));

        assert!(iter.next().is_none());
    }
}
