//!
//! Provides types for representing scenes, including:
//! * The [`Mesh`](trait.Mesh.html) trait for types that represent triangle meshes,
//! * The [`Material`](struct.Material.html) and [`MaterialBuilder`](struct.MaterialBuilder.html) types for OBJ-compatible materials,
//! * [`Entity`](struct.Entity.html) as a standard struct for a named mesh with a referenced material.
//!
extern crate aitios_geom as geom;
extern crate tobj;

mod entity;
mod material;
mod mesh;

pub use entity::Entity;
pub use material::{Material, MaterialBuilder};
pub use mesh::*;
