use std::path::PathBuf;
use std::collections::HashMap;

/// Diffuse color, also known as albedo or basecolor.
const DIFFUSE_COLOR_MAP_KEY : &str = "map_Kd";
/// Ambient color map.
const AMBIENT_COLOR_MAP_KEY : &str = "map_Ka";
/// Specular color map.
const SPECULAR_COLOR_MAP_KEY : &str = "map_Ks";
/// Scalar bump map.
const BUMP_MAP_KEY : &str = "bump";
/// Scalar displacment map with midpoint at 0.5.
const DISPLACEMENT_MAP_KEY : &str = "disp";
/// Tangent-space normal map in any format supported by the target application.
/// Inofficial in MTL format, only supported by some target applications.
const NORMAL_MAP_KEY : &str = "norm";
/// Scalar roughness map.
/// Inofficial in MTL format, only supported by some target applications.
const ROUGHNESS_MAP_KEY : &str = "map_Pr";
/// Scalar metallicity map.
/// Inofficial in MTL format, only supported by some target applications.
const METALLIC_MAP_KEY : &str = "map_Pm";
/// Scalar sheen map.
/// Inofficial in MTL format, only supported by some target applications.
const SHEEN_MAP_KEY : &str = "map_Ps";
/// Emission map.
/// Inofficial in MTL format, only supported by some target applications.
const EMISSIVE_MAP_KEY : &str = "map_Ps";

/// Models the appearance of an [Entity](struct.Entity.html) using paths
/// to texture maps.
///
/// Where possible, the maps adhere to a subset of the
/// [inofficial spec](http://paulbourke.net/dataformats/mtl/) for MTL files.
///
/// | Method Name           | Associated MTL map       | Alternative names |
/// | --------------------- | ------------------------ | ----------------- |
/// | `diffuse_color_map`   | `map_Kd`, Diffuse color  | Albedo, Basecolor |
/// | `ambient_color_map`   | `map_Ka`, Ambient color  | —                 |
/// | `specular_color_map`  | `map_Ks`, Specular color | —                 |
/// | `bump_map`            | `bump`, Bump map         | —                 |
/// | `displacement_map`    | `disp`, Displacement map | —                 |
///
/// Further, some proposed additions for physically-based rendering can be used,
/// compatible to the map names proposed in the
/// [Exocortex Blog](http://exocortex.com/blog/extending_wavefront_mtl_to_support_pbr),
/// which are derived from the PBR techniques described in
/// [Physically-based Shading At Disney](https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf):
///
/// | Method Name     | Associated MTL map           | Alternative names     |
/// | --------------- | ---------------------------- | --------------------- |
/// | `normal_map`    | `norm`, Tangent-space normal | —                     |
/// | `roughness_map` | `map_Pr`, Roughness          | —                     |
/// | `metallic_map`  | `map_Pm`, Metallic           | Metallicity           |
/// | `sheen_map`     | `map_Ps`, Sheen              | —                     |
/// | `emissive_map`  | `map_Ke`, Emissive           | Emission              |
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Material {
    name: String,
    /// Maps strings against texture map files, where possible adhering to a subset of
    /// the OBJ/MTL naming conventions.
    ///
    /// See the [inofficial spec](http://paulbourke.net/dataformats/mtl/) for more information
    /// about MTL conventions. The following maps are compatible with this inofficial spec:
    ///
    /// | Key           | Associated OBJ map   | Alternative names |
    /// | ------------- | -------------------- | ----------------- |
    /// | map_Kd        | Diffuse color        | Albedo, Basecolor |
    /// | map_Ka        | Ambient color        | —                 |
    /// | map_Ks        | Specular color       | —                 |
    /// | bump          | Bump map             | —                 |
    /// | disp          | Displacement map     | —                 |
    ///
    /// Further, some proposed additions for physically-based rendering can be used,
    /// compatible to the map names proposed in the
    /// [Exocortex Blog](http://exocortex.com/blog/extending_wavefront_mtl_to_support_pbr),
    /// which are derived from
    /// [Physically-based Shading At Disney](https://disney-animation.s3.amazonaws.com/library/s2012_pbs_disney_brdf_notes_v2.pdf):
    ///
    /// | Key           | Associated OBJ map   | Alternative names |
    /// | ------------- | -------------------- | ----------------- |
    /// | norm          | Tangent-space normal | —                 |
    /// | map_Pr        | Roughness            | —                 |
    /// | map_Pm        | Metallic             | Metallicity       |
    /// | map_Ps        | Sheen                | —                 |
    /// | map_Ke        | Emissive             | Emission          |
    maps: HashMap<String, PathBuf>,
}

impl Material {
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Diffuse color, also known as albedo or basecolor.
    pub fn diffuse_color_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(DIFFUSE_COLOR_MAP_KEY))
    }

    /// Ambient color map.
    pub fn ambient_color_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(AMBIENT_COLOR_MAP_KEY))
    }

    /// Specular color map.
    pub fn specular_color_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(SPECULAR_COLOR_MAP_KEY))
    }

    /// Gets the scalar bump map.
    pub fn bump_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(BUMP_MAP_KEY))
    }

    /// Scalar displacment map with midpoint at 0.5.
    pub fn displacement_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(DISPLACEMENT_MAP_KEY))
    }

    /// Tangent-space normal map in any format supported by the target application.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn normal_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(NORMAL_MAP_KEY))
    }

    /// Scalar roughness map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn roughness_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(ROUGHNESS_MAP_KEY))
    }

    /// Scalar metallicity map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn metallic_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(METALLIC_MAP_KEY))
    }

    /// Scalar sheen map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn sheen_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(SHEEN_MAP_KEY))
    }

    /// Emission map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn emissive_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(EMISSIVE_MAP_KEY))
    }

    /// Gets map names in MTL format mapped against paths.
    /// Useful for export of MTL files.
    pub fn maps(&self) -> &HashMap<String, PathBuf> {
        &self.maps
    }
}

/// Creates new and derived materials.
///
/// ```
/// use aitios_scene::MaterialBuilder;
/// use std::path::PathBuf;
///
/// let new_material = MaterialBuilder::new()
///     .diffuse_color_map("/tmp/textures/1113_diffuse.jpg")
///     .ambient_color_map("/tmp/textures/1113_ambient.jpg")
///     .build();
///
/// let derived_material = MaterialBuilder::from(&new_material)
///     .specular_color_map("/tmp/textures/1113_specular.jpg")
///     .build();
///
/// assert_eq!(new_material.ambient_color_map(), derived_material.ambient_color_map());
/// assert_eq!(new_material.diffuse_color_map(), derived_material.diffuse_color_map());
/// assert_ne!(new_material.specular_color_map(), derived_material.specular_color_map());
/// # assert_eq!(derived_material.diffuse_color_map(), Some(&PathBuf::from("/tmp/textures/1113_diffuse.jpg")));
/// # assert_eq!(derived_material.ambient_color_map(), Some(&PathBuf::from("/tmp/textures/1113_ambient.jpg")));
/// # assert_eq!(derived_material.specular_color_map(), Some(&PathBuf::from("/tmp/textures/1113_specular.jpg")));
/// # assert_eq!(new_material.diffuse_color_map(), Some(&PathBuf::from("/tmp/textures/1113_diffuse.jpg")));
/// # assert_eq!(new_material.ambient_color_map(), Some(&PathBuf::from("/tmp/textures/1113_ambient.jpg")));
/// # assert!(new_material.specular_color_map().is_none());
/// ```
pub struct MaterialBuilder {
    mat: Material
}

impl MaterialBuilder {
    pub fn new() -> Self {
        MaterialBuilder {
            mat: Material {
                name: String::new(),
                maps: HashMap::new()
            }
        }
    }

    pub fn name<S : Into<String>>(mut self, name : S) -> Self {
        self.mat.name = name.into();
        self
    }

    /// Sets the diffuse color, also known as albedo or basecolor.
    pub fn diffuse_color_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(DIFFUSE_COLOR_MAP_KEY), path.into());
        self
    }

    /// Sets the ambient color map.
    pub fn ambient_color_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(AMBIENT_COLOR_MAP_KEY), path.into());
        self
    }

    /// Sets the specular color map.
    pub fn specular_color_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(SPECULAR_COLOR_MAP_KEY), path.into());
        self
    }

    /// Sets the scalar bump map.
    pub fn bump_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(BUMP_MAP_KEY), path.into());
        self
    }

    // Sets the scalar displacement map with midpoint at 0.5.
    pub fn displacement_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(DISPLACEMENT_MAP_KEY), path.into());
        self
    }

    /// Sets the tangent-space normal map in any format supported by the target application.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn normal_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(NORMAL_MAP_KEY), path.into());
        self
    }

    /// Sets the scalar roughness map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn roughness_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(ROUGHNESS_MAP_KEY), path.into());
        self
    }

    /// Sets the scalar metallicity map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn metallic_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(METALLIC_MAP_KEY), path.into());
        self
    }

    /// Sets the scalar sheen map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn sheen_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(SHEEN_MAP_KEY), path.into());
        self
    }

    /// Sets the emission map.
    /// Inofficial in MTL format, only supported by some target applications.
    pub fn emissive_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(EMISSIVE_MAP_KEY), path.into());
        self
    }

    pub fn build(self) -> Material {
        self.mat
    }
}

/// Creates a new material builder, consuming the given existing material.
impl From<Material> for MaterialBuilder {
    fn from(mat: Material) -> Self {
        MaterialBuilder { mat }
    }
}

/// Creates a new material builder by cloning the given existing material.
impl<'a> From<&'a Material> for MaterialBuilder {
    fn from(from: &'a Material) -> Self {
        MaterialBuilder { mat: from.clone() }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn equality() {
        let mat1 = MaterialBuilder::new()
            .name("Holodrio")
            .diffuse_color_map("/tmp/nonexistent.png")
            .normal_map("/tmp/temp.png")
            .build();

        let same1 = MaterialBuilder::from(&mat1).build();
        let same2 = mat1.clone();

        assert_eq!(mat1, same1);
        assert_eq!(mat1, same2);

        let other_name = MaterialBuilder::from(&mat1)
            .name("Olodriho2")
            .build();

        let other_diffuse = MaterialBuilder::from(&mat1)
            .diffuse_color_map("/tmp/othermap1.png")
            .build();

        let other_normal = MaterialBuilder::from(&mat1)
            .normal_map("/tmp/othermap1_normal2.png")
            .build();

        assert_ne!(mat1, other_name);
        assert_ne!(mat1, other_diffuse);
        assert_ne!(mat1, other_normal);
        assert_ne!(other_diffuse, other_normal);
    }
}
