use ffi::{
    AiVector3D, AiColor4D, AiMaterial, AiMaterialProperty, AiPropertyTypeInfo, AiString, AiShadingMode,
    AiTextureOp, AiTextureType, AiTextureMapMode, AiTextureMapping, AiTextureFlags, AiBlendMode, AiUVTransform,
    aiGetMaterialColor, aiGetMaterialString, aiGetMaterialIntegerArray, aiGetMaterialFloatArray,
};
use std::ptr::{null_mut};
use std::marker::PhantomData;

define_type_and_iterator_indirect! {
    /// Material type (not yet implemented)
    struct Material(&AiMaterial)
    /// Material iterator type.
    struct MaterialIter
}

define_type_and_iterator_indirect! {
    /// Material type (not yet implemented)
    struct MaterialProperty(&AiMaterialProperty)
    /// MaterialProperty iterator type.
    struct MaterialPropertyIter
}

pub struct TextureValue<'a, T: 'a, U> {
    pub texture_type: AiTextureType,
    pub index: u32,
    pub value: U,
    phantom: PhantomData<&'a T>
}

impl<'a> Material<'a> {
    pub fn num_properties(&self) -> u32 { self.num_properties }

    pub fn properties_iter(&self) -> MaterialPropertyIter {
        MaterialPropertyIter::new(
          self.properties as *const *const AiMaterialProperty,
          self.num_properties as usize
        )
    }

    pub fn property(&self, id: usize) -> Option<MaterialProperty> {
        if id < self.num_properties as usize {
            unsafe { Some(MaterialProperty::from_raw(*(self.properties.offset(id as isize)))) }
        } else {
            None
        }
    }

    pub fn global_background(&self) -> Option<AiString> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::GlobalBackground(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn diffuse_color(&self) -> Option<AiColor4D> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::DiffuseColor(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn ambient_color(&self) -> Option<AiColor4D> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::AmbientColor(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn emissive_color(&self) -> Option<AiColor4D> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::EmissiveColor(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn specular_color(&self) -> Option<AiColor4D> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::SpecularColor(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn transparent_color(&self) -> Option<AiColor4D> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TransparentColor(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn reflective_color(&self) -> Option<AiColor4D> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::ReflectiveColor(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn name(&self) -> Option<AiString> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::Name(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn shading_model(&self) -> Option<AiShadingMode> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::ShadingModel(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn two_sided(&self) -> Option<bool> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TwoSided(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn wireframe(&self) -> Option<bool> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::Wireframe(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn blend_mode(&self) -> Option<AiBlendMode> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::BlendMode(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn opacity(&self) -> Option<f32> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::Opacity(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn bump_scaling(&self) -> Option<f32> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::BumpScaling(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn shininess(&self) -> Option<f32> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::Shininess(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn reflectivity(&self) -> Option<f32> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::Reflectivity(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn shininess_strength(&self) -> Option<f32> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::ShininessStrength(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn index_of_refraction(&self) -> Option<f32> {
        self
          .properties_iter()
          .flat_map(|prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::IndexOfRefraction(value)) => Some(value),
                  _ => None,
              }
          })
          .nth(0)
    }

    pub fn texture_path(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiString>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TexturePath(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_uvw_source(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, i32>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureUvwSource(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_op(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiTextureOp>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureOp(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_mapping(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiTextureMapping>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureMapping(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_blend(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, f32>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureBlend(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_mapping_mode_u(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiTextureMapMode>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureMappingModeU(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_mapping_mode_v(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiTextureMapMode>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureMappingModeV(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_map_axis(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiVector3D>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureMapAxis(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_uv_transform(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiUVTransform>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureUvTransform(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }

    pub fn texture_flags(&'a self) -> impl Iterator<Item = TextureValue<'a, Material, AiTextureFlags>> {
        self
          .properties_iter()
          .flat_map(move |prop| {
              match prop.key(self) {
                  Some(MaterialPropertyKey::TextureFlags(texture_type, index, value)) => {
                      Some(TextureValue {
                          texture_type,
                          index,
                          value,
                          phantom: PhantomData,
                      })
                  },
                  _ => None,
              }
          })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MaterialPropertyKey {
    GlobalBackground(AiString),
    DiffuseColor(AiColor4D),
    AmbientColor(AiColor4D),
    EmissiveColor(AiColor4D),
    SpecularColor(AiColor4D),
    TransparentColor(AiColor4D),
    ReflectiveColor(AiColor4D),
    Name(AiString),
    ShadingModel(AiShadingMode),
    TwoSided(bool),
    Wireframe(bool),
    BlendMode(AiBlendMode),
    Opacity(f32),
    BumpScaling(f32),
    Shininess(f32),
    Reflectivity(f32),
    ShininessStrength(f32),
    IndexOfRefraction(f32),
    TexturePath(AiTextureType, u32, AiString),
    TextureUvwSource(AiTextureType, u32, i32),
    TextureOp(AiTextureType, u32, AiTextureOp),
    TextureMapping(AiTextureType, u32, AiTextureMapping),
    TextureBlend(AiTextureType, u32, f32),
    TextureMappingModeU(AiTextureType, u32, AiTextureMapMode),
    TextureMappingModeV(AiTextureType, u32, AiTextureMapMode),
    TextureMapAxis(AiTextureType, u32, AiVector3D),
    TextureUvTransform(AiTextureType, u32, AiUVTransform),
    TextureFlags(AiTextureType, u32, AiTextureFlags),
}

impl PartialEq for MaterialPropertyKey {
    fn eq(&self, other: &MaterialPropertyKey) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

// TODO: Move this into the FFI crate as a std::convert::From impl
fn texture_type_from_raw(item: u32) -> AiTextureType {
    match item {
        0x0 => AiTextureType::None,
        0x1 => AiTextureType::Diffuse,
        0x2 => AiTextureType::Specular,
        0x3 => AiTextureType::Ambient,
        0x4 => AiTextureType::Emissive,
        0x5 => AiTextureType::Height,
        0x6 => AiTextureType::Normals,
        0x7 => AiTextureType::Shininess,
        0x8 => AiTextureType::Opacity,
        0x9 => AiTextureType::Displacement,
        0xA => AiTextureType::Lightmap,
        0xB => AiTextureType::Reflection,
        _ => AiTextureType::Unknown,
    }
}

impl<'a> MaterialProperty<'a> {
    /// Returns the key of the property.
    pub fn key(&self, mat: &Material) -> Option<MaterialPropertyKey> {
        match self.key.as_ref() {
            "?bg.global" => self.string_value(mat).map(MaterialPropertyKey::GlobalBackground),
            "$clr.diffuse" => self.color_value(mat).map(MaterialPropertyKey::DiffuseColor),
            "$clr.specular" => self.color_value(mat).map(MaterialPropertyKey::SpecularColor),
            "$clr.ambient" => self.color_value(mat).map(MaterialPropertyKey::AmbientColor),
            "$clr.emissive" => self.color_value(mat).map(MaterialPropertyKey::EmissiveColor),
            "$clr.transparent" => self.color_value(mat).map(MaterialPropertyKey::TransparentColor),
            "$clr.reflective" => self.color_value(mat).map(MaterialPropertyKey::ReflectiveColor),
            "?mat.name" => self.string_value(mat).map(MaterialPropertyKey::Name),
            "$mat.shadingm" => self.shading_mode_value(mat).map(MaterialPropertyKey::ShadingModel),
            "$mat.twosided" => self.bool_value(mat).map(MaterialPropertyKey::TwoSided),
            "$mat.wireframe" => self.bool_value(mat).map(MaterialPropertyKey::Wireframe),
            "$mat.blend" => self.blend_mode_value(mat).map(MaterialPropertyKey::BlendMode),
            "$mat.opacity" => self.float_value(mat).map(MaterialPropertyKey::Opacity),
            "$mat.bumpscaling" => self.float_value(mat).map(MaterialPropertyKey::BumpScaling),
            "$mat.shininess" => self.float_value(mat).map(MaterialPropertyKey::Shininess),
            "$mat.reflectivity" => self.float_value(mat).map(MaterialPropertyKey::Reflectivity),
            "$mat.shinpercent" => self.float_value(mat).map(MaterialPropertyKey::ShininessStrength),
            "$mat.refracti" => self.float_value(mat).map(MaterialPropertyKey::IndexOfRefraction),
            "$tex.file" => self.string_value(mat).map(|v| MaterialPropertyKey::TexturePath(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.uvwsrc" => self.int_value(mat).map(|v| MaterialPropertyKey::TextureUvwSource(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.op" => self.texture_op_value(mat).map(|v| MaterialPropertyKey::TextureOp(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.mapping" => self.texture_mapping_value(mat).map(|v| MaterialPropertyKey::TextureMapping(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.blend" => self.float_value(mat).map(|v| MaterialPropertyKey::TextureBlend(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.mapmodeu" => self.texture_map_mode_value(mat).map(|v| MaterialPropertyKey::TextureMappingModeU(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.mapmodev" => self.texture_map_mode_value(mat).map(|v| MaterialPropertyKey::TextureMappingModeV(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.mapaxis" => self.vector_value(mat).map(|v| MaterialPropertyKey::TextureMapAxis(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.uvtrafo" => self.uv_transform_value(mat).map(|v| MaterialPropertyKey::TextureUvTransform(texture_type_from_raw(self.semantic), self.index, v)),
            "$tex.flags" => self.texture_flags_value(mat).map(|v| MaterialPropertyKey::TextureFlags(texture_type_from_raw(self.semantic), self.index, v)),
            _ => None,
        }
    }

    pub fn index(&self) -> u32 {
        self.index as u32
    }

    pub fn data_type(&self) -> AiPropertyTypeInfo {
        self.property_type
    }

    pub fn shading_mode_value(&self, mat: &Material) -> Option<AiShadingMode> {
        if let Some(value) = self.int_value(mat) {
            match value {
                0x1 => Some(AiShadingMode::Flat),
                0x2 => Some(AiShadingMode::Gouraud),
                0x3 => Some(AiShadingMode::Phong),
                0x4 => Some(AiShadingMode::Blinn),
                0x5 => Some(AiShadingMode::Toon),
                0x6 => Some(AiShadingMode::OrenNayar),
                0x7 => Some(AiShadingMode::Minnaert),
                0x8 => Some(AiShadingMode::CookTorrance),
                0x9 => Some(AiShadingMode::NoShading),
                0xA => Some(AiShadingMode::Fresnel),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn blend_mode_value(&self, mat: &Material) -> Option<AiBlendMode> {
        if let Some(value) = self.int_value(mat) {
            match value {
                0x1 => Some(AiBlendMode::Additive),
                _ => Some(AiBlendMode::Default),
            }
        } else {
            None
        }
    }

    pub fn texture_op_value(&self, mat: &Material) -> Option<AiTextureOp> {
        if let Some(value) = self.int_value(mat) {
            match value {
                0x0 => Some(AiTextureOp::Multiply),
                0x1 => Some(AiTextureOp::Add),
                0x2 => Some(AiTextureOp::Subtract),
                0x3 => Some(AiTextureOp::Divide),
                0x4 => Some(AiTextureOp::SmoothAdd),
                0x5 => Some(AiTextureOp::SignedAdd),
                _ => None
            }
        } else {
            None
        }
    }

    pub fn texture_mapping_value(&self, mat: &Material) -> Option<AiTextureMapping> {
        if let Some(value) = self.int_value(mat) {
            match value {
                0x0 => Some(AiTextureMapping::UV),
                0x1 => Some(AiTextureMapping::Sphere),
                0x2 => Some(AiTextureMapping::Cylinder),
                0x3 => Some(AiTextureMapping::Box),
                0x4 => Some(AiTextureMapping::Plane),
                0x5 => Some(AiTextureMapping::Other),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn texture_map_mode_value(&self, mat: &Material) -> Option<AiTextureMapMode> {
        if let Some(value) = self.int_value(mat) {
            match value {
                0x0 => Some(AiTextureMapMode::Wrap),
                0x1 => Some(AiTextureMapMode::Clamp),
                0x2 => Some(AiTextureMapMode::Mirror),
                0x3 => Some(AiTextureMapMode::Decal),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn texture_flags_value(&self, mat: &Material) -> Option<AiTextureFlags> {
        if let Some(value) = self.int_value(mat) {
            AiTextureFlags::from_bits(value as u32)
        } else {
            None
        }
    }

    pub fn uv_transform_value(&self, mat: &Material) -> Option<AiUVTransform> {
        if self.property_type == AiPropertyTypeInfo::Float {
            unsafe {
                let mut out: AiUVTransform = std::mem::zeroed();
                let mut length: u32 = std::mem::size_of_val(&out) as u32;
                aiGetMaterialFloatArray(mat.0, std::mem::transmute(self.key.data.as_ptr()), self.semantic, self.index, std::mem::transmute(&mut out), &mut length);
                Some(out)
            }
        } else {
            None
        }
    }

    pub fn bool_value(&self, mat: &Material) -> Option<bool> {
        self.int_value(mat).map(|i| {
            match i {
                0 => false,
                _ => true,
            }
        })
    }

    pub fn float_value(&self, mat: &Material) -> Option<f32> {
        if self.property_type == AiPropertyTypeInfo::Float {
            let mut out = 0.0f32;
            let mut length: u32 = 1;
            unsafe { aiGetMaterialFloatArray(mat.0, std::mem::transmute(self.key.data.as_ptr()), self.semantic, self.index, &mut out, &mut length); };
            Some(out)
        } else {
            None
        }
    }

    pub fn vector_value(&self, mat: &Material) -> Option<AiVector3D> {
        if self.property_type == AiPropertyTypeInfo::Float {
            let mut out = AiVector3D { x: 0.0, y: 0.0, z: 0.0 };
            let mut length: u32 = std::mem::size_of::<AiVector3D>() as u32;
            unsafe { aiGetMaterialFloatArray(mat.0, std::mem::transmute(self.key.data.as_ptr()), self.semantic, self.index, std::mem::transmute(&mut out), &mut length); };
            Some(out)
        } else {
            None
        }
    }

    pub fn int_value(&self, mat: &Material) -> Option<i32> {
        if self.property_type == AiPropertyTypeInfo::Integer {
            let mut out: i32 = 0;
            unsafe { aiGetMaterialIntegerArray(mat.0, std::mem::transmute(self.key.data.as_ptr()), self.semantic, self.index, &mut out, null_mut()); };
            Some(out)
        } else {
            None
        }
    }

    fn color_value(&self, mat: &Material) -> Option<AiColor4D> {
        if self.property_type == AiPropertyTypeInfo::Float {
            let mut out = AiColor4D { r: 0.0, g: 0.0, b:0.0, a: 0.0 };
            unsafe { aiGetMaterialColor(mat.0, std::mem::transmute(self.key.data.as_ptr()), self.semantic, self.index, &mut out); };
            Some(out)
        } else {
            None
        }
    }

    fn string_value(&self, mat: &Material) -> Option<AiString> {
        if self.property_type == AiPropertyTypeInfo::String {
            let mut out = AiString::default();
            unsafe { aiGetMaterialString(mat.0, std::mem::transmute(self.key.data.as_ptr()), self.semantic, self.index, &mut out); };
            Some(out)
        } else {
            None
        }
    }
}
