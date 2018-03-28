use std::path::PathBuf;
use std::collections::HashMap;

const DIFFUSE_COLOR_MAP_KEY : &str = "map_Kd";
const AMBIENT_COLOR_MAP_KEY : &str = "map_Ka";
const SPECULAR_COLOR_MAP_KEY : &str = "map_Ks";

#[derive(Debug, Clone)]
pub struct Material {
    name: String,
    /// Maps strings against texture map files, where possible following MTL file conventions.
    ///
    /// See the [inofficial spec](http://paulbourke.net/dataformats/mtl/) for more information
    /// about MTL conventions.
    ///
    /// | Key           | Associated map |
    /// | ------------- | -------------- |
    /// | map_Kd        | Diffuse color  |
    /// | map_Ka        | Ambient color  |
    /// | map_Ks        | Specular color |
    maps: HashMap<String, PathBuf>,
}

impl Material {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn diffuse_color_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(DIFFUSE_COLOR_MAP_KEY))
    }

    pub fn ambient_color_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(AMBIENT_COLOR_MAP_KEY))
    }

    pub fn specular_color_map(&self) -> Option<&PathBuf> {
        self.maps.get(&String::from(SPECULAR_COLOR_MAP_KEY))
    }

    pub fn get_map(&self, key: &String) -> Option<&PathBuf> {
        self.maps.get(key)
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

    pub fn diffuse_color_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(DIFFUSE_COLOR_MAP_KEY), path.into());
        self
    }

    pub fn ambient_color_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(AMBIENT_COLOR_MAP_KEY), path.into());
        self
    }

    pub fn specular_color_map<P : Into<PathBuf>>(mut self, path: P) -> Self {
        self.mat.maps.insert(String::from(SPECULAR_COLOR_MAP_KEY), path.into());
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
