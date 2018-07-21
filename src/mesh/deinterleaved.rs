use super::mesh::Mesh;
use geom::{Normal, Position, Texcoords, Vec2, Vec3, Vertex};
use std::iter::FromIterator;

/// An indexed triangle mesh with de-interleaved vertices, i.e.
/// each attribute has its own vector.
///
/// Format is compatible with [tobj meshes](http://www.willusher.io/tobj/tobj/struct.Mesh.html)
/// and can be directly created from position, normal, texcoord and index data.
///
/// ```
/// use aitios_scene::{Mesh, DeinterleavedIndexedMeshBuf};
///
/// // Directly create a mesh from vectors (make sure the layout is correct!)
/// let mesh = DeinterleavedIndexedMeshBuf {
///     positions: vec![
///         1.0, 1.0, 1.0,
///         10.0, 10.0, 10.0,
///         100.0, 100.0, 100.0,
///         -1.0, -1.0, -1.0,
///         -10.0, -10.0, -10.0,
///         -100.0, -100.0, -100.0
///     ],
///     normals: vec![
///         1.0, 0.0, 0.0,
///         1.0, 0.0, 0.0,
///         1.0, 0.0, 0.0,
///         0.0, 0.0, 1.0,
///         0.0, 0.0, 1.0,
///         0.0, 0.0, 1.0,
///     ],
///     texcoords: vec![
///         0.0, 0.0,
///         1.0, 1.0,
///         0.0, 1.0,
///         0.7, 0.7,
///         0.7, 0.7,
///         0.7, 0.7
///     ],
///     indices: vec![
///         3, 4, 5,
///         0, 1, 2
///     ]
/// };
///
/// assert_eq!(mesh.vertex_count(), 6);
/// assert_eq!(mesh.triangle_count(), 2);
/// ```
///
/// The mesh can also be collected from an unindexed iterator over vertices:
///
/// ```
/// # extern crate aitios_geom;
/// # extern crate aitios_scene;
///
/// use aitios_scene::{Mesh, DeinterleavedIndexedMeshBuf};
/// use aitios_geom::{Vertex, Vec2, Vec3};
///
/// # fn main() {
/// let mesh = (0..102)
///     .map(|n| n as f32)
///     // Zero-area triangles in a straight line
///     .map(|n| Vertex {
///         position: Vec3::new(n, n, n),
///         normal: Vec3::new(0.0, 1.0, 0.0),
///         texcoords: Vec2::new(n / 102.0, n / 102.0)
///     }).collect::<DeinterleavedIndexedMeshBuf>();
///
/// assert_eq!(mesh.vertex_count(), 102);
/// # }
/// ```
#[derive(Clone)]
pub struct DeinterleavedIndexedMeshBuf {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub texcoords: Vec<f32>,
    pub indices: Vec<u32>,
}

impl<'a> Mesh<'a> for DeinterleavedIndexedMeshBuf {
    type Vertex = Vertex;
    type VertexIter = DeinterleavedIndexedMeshBufIter<'a>;

    fn vertices(&'a self) -> Self::VertexIter {
        self.into_iter()
    }

    fn vertex_count(&'a self) -> usize {
        self.indices.len()
    }
}

impl DeinterleavedIndexedMeshBuf {
    pub fn vertex_at(&self, index_index: usize) -> Vertex {
        let idx = self.indices[index_index] as usize;

        let position = &self.positions[idx * 3..(idx + 1) * 3];
        let normal = &self.normals[idx * 3..(idx + 1) * 3];
        let texcoords = &self.texcoords[idx * 2..(idx + 1) * 2];

        Vertex {
            position: Vec3::new(position[0], position[1], position[2]),
            normal: Vec3::new(normal[0], normal[1], normal[2]),
            texcoords: Vec2::new(texcoords[0], texcoords[1]),
        }
    }
}

impl<'a> IntoIterator for &'a DeinterleavedIndexedMeshBuf {
    type Item = Vertex;
    type IntoIter = DeinterleavedIndexedMeshBufIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        DeinterleavedIndexedMeshBufIter {
            mesh: self,
            next_indices_idx: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct DeinterleavedIndexedMeshBufIter<'a> {
    mesh: &'a DeinterleavedIndexedMeshBuf,
    next_indices_idx: usize,
}

impl<'a> Iterator for DeinterleavedIndexedMeshBufIter<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.next_indices_idx;

        if idx >= self.mesh.indices.len() {
            None
        } else {
            self.next_indices_idx += 1;
            let vtx = self.mesh.vertex_at(idx);
            Some(vtx)
        }
    }
}

impl<V> FromIterator<V> for DeinterleavedIndexedMeshBuf
where
    V: Position + Normal + Texcoords,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = V>,
    {
        let mut buf = DeinterleavedIndexedMeshBuf {
            positions: Vec::new(),
            normals: Vec::new(),
            texcoords: Vec::new(),
            indices: Vec::new(),
        };

        iter.into_iter().for_each(|vtx| {
            let Vec3 {
                x: pos_x,
                y: pos_y,
                z: pos_z,
            } = vtx.position();
            let Vec3 {
                x: normal_x,
                y: normal_y,
                z: normal_z,
            } = vtx.normal();
            let Vec2 { x: tex_x, y: tex_y } = vtx.texcoords();
            let idx = buf.indices.len() as u32;

            buf.positions.extend(&[pos_x, pos_y, pos_z]);
            buf.normals.extend(&[normal_x, normal_y, normal_z]);
            buf.texcoords.extend(&[tex_x, tex_y]);
            buf.indices.push(idx);
        });

        buf
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use geom::{Position, Texcoords, Vec2, Vec3};

    #[test]
    fn test_iteration_empty_mesh() {
        let mesh = DeinterleavedIndexedMeshBuf {
            positions: vec![],
            normals: vec![],
            texcoords: vec![],
            indices: vec![],
        };

        assert!(mesh.into_iter().next().is_none());
    }

    #[test]
    fn test_iteration_normal_mesh() {
        let mesh = DeinterleavedIndexedMeshBuf {
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

        let mut iter = mesh.into_iter();
        let mut _iter2 = mesh.into_iter(); // should be able to make multiple of them

        let vtx3 = iter.next().unwrap();
        let pos3 = vtx3.position();

        assert_eq!(pos3, Vec3::new(-1.0, -1.0, -1.0));

        assert!(iter.next().is_some()); // 4
        assert!(iter.next().is_some()); // 5
        assert!(iter.next().is_some()); // 0

        let vtx1 = iter.next().unwrap();
        let vtx2 = iter.next().unwrap();

        let tex1 = vtx1.texcoords();
        let pos2 = vtx2.position();

        assert_eq!(tex1, Vec2::new(1.0, 1.0));
        assert_eq!(pos2, Vec3::new(100.0, 100.0, 100.0));

        assert!(iter.next().is_none(), "Should have no more vertices");
    }

    #[test]
    fn test_iteration_non_full_vtx_at_end_mesh() {
        let mesh = DeinterleavedIndexedMeshBuf {
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

        assert_eq!(mesh.vertex_count(), 5);
    }

    #[test]
    fn test_from_triangle_iterator() {
        let mesh = DeinterleavedIndexedMeshBuf {
            positions: vec![
                1.0, 1.0, 1.0, 10.0, 10.0, 10.0, 100.0, 100.0, 100.0, -1.0, -1.0, -1.0, -10.0,
                -10.0, -10.0, -100.0, -100.0, -100.0,
            ],
            normals: vec![
                1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
                0.0, 1.0,
            ],
            texcoords: vec![0.0, 0.0, 1.0, 1.0, 0.0, 1.0, 0.7, 0.7, 0.7, 0.7, 0.7, 0.7],
            indices: vec![0, 1, 2, 3, 4, 5],
        };

        // Collect another deinterleavedindexedmeshbuf from vertex iterator
        // Should yield the same mesh since indices are in order
        // If would be out of order, the new mesh would have that order and
        // all vertices duplicated
        let cloned_mesh: DeinterleavedIndexedMeshBuf = mesh.vertices().collect();

        // Since indices are 0..4 the order should be the same
        assert_eq!(mesh.positions, cloned_mesh.positions);
        assert_eq!(mesh.normals, cloned_mesh.normals);
        assert_eq!(mesh.texcoords, cloned_mesh.texcoords);
        assert_eq!(mesh.positions, cloned_mesh.positions);

        // This time, first collect the vertex references into a vector and then create a mesh from that
        // to ensure an iterator over vertex references will work as well

        // FIXME no signature seems to work for both cases

        /*let cloned_vertices : Vec<VertexRef> = mesh.vertices().collect();
        let cloned_mesh : DeinterleavedIndexedMeshBuf = cloned_vertices.iter().collect();

        assert_eq!(mesh.positions, cloned_mesh.positions);
        assert_eq!(mesh.normals, cloned_mesh.normals);
        assert_eq!(mesh.texcoords, cloned_mesh.texcoords);
        assert_eq!(mesh.positions, cloned_mesh.positions);*/
    }
}
