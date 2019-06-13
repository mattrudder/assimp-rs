use ffi::{
    AiVector3D, AiColor4D, AiMaterial, AiMaterialProperty, AiPropertyTypeInfo, AiString, AiShadingMode,
    AiTextureOp, AiTextureMapMode, AiTextureMapping, AiTextureType, AiTextureFlags, AiBlendMode, AiUVTransform,
    aiGetMaterialColor, aiGetMaterialString, aiGetMaterialIntegerArray, aiGetMaterialFloatArray,
};
use std::os::raw::{c_char, c_int, c_float};
use std::ptr::{null, null_mut};
use std::convert::{TryFrom, TryInto};

define_type_and_iterator_indirect! {
    /// Material type (not yet implemented)
    struct Material(&AiMaterial)
    /// Material iterator type.
    struct MaterialIter
}

//define_type_and_iterator_indirect! {
//    /// Material type (not yet implemented)
//    struct MaterialProperty(&AiMaterialProperty)
//    /// MaterialProperty iterator type.
//    struct MaterialPropertyIter
//}

pub struct MaterialProperty<'a>((&'a AiMaterial, &'a AiMaterialProperty));

#[doc(hidden)]
impl<'a> MaterialProperty<'a> {
    pub fn from_raw(parent: *const AiMaterial, raw: *const AiMaterialProperty) -> MaterialProperty<'a> {
        unsafe { MaterialProperty((&*parent, &*raw)) }
    }
}

impl<'a> ::std::ops::Deref for MaterialProperty<'a> {
    type Target = AiMaterialProperty;
    fn deref(&self) -> &AiMaterialProperty {
        let (_, prop) = self.0;
        prop
    }
}

pub struct MaterialPropertyIter<'a> {
    parent: *const AiMaterial,
    ptr: *const *const AiMaterialProperty,
    idx: isize,
    len: usize,
    _mk: ::std::marker::PhantomData<&'a ()>
}

#[doc(hidden)]
impl<'a> MaterialPropertyIter<'a> {
    pub fn new(parent: *const AiMaterial, ptr: *const *const AiMaterialProperty, len: usize) -> MaterialPropertyIter<'a> {
        MaterialPropertyIter { parent, ptr, idx: 0, len, _mk: ::std::marker::PhantomData }
    }
}

impl<'a> Iterator for MaterialPropertyIter<'a> {
    type Item = MaterialProperty<'a>;
    fn next(&mut self) -> Option<MaterialProperty<'a>> {
        if self.idx < self.len as isize {
            let item = MaterialProperty::from_raw(self.parent, unsafe { *self.ptr.offset(self.idx) });
            self.idx = self.idx + 1;
            Some(item)
        } else {
            None
        }
    }
}

impl<'a> ExactSizeIterator for MaterialPropertyIter<'a> {
fn len(&self) -> usize { self.len }
}

impl<'a> Material<'a> {
    pub fn num_properties(&self) -> u32 { self.num_properties }

    pub fn properties_iter(&self) -> MaterialPropertyIter {
        MaterialPropertyIter::new(
            &*self.0 as *const AiMaterial,
          self.properties as *const *const AiMaterialProperty,
          self.num_properties as usize
        )
    }

    pub fn property(&self, id: usize) -> Option<MaterialProperty> {
        if id < self.num_properties as usize {
            unsafe { Some(MaterialProperty::from_raw(self.0 as *const AiMaterial,*(self.properties.offset(id as isize)))) }
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MaterialPropertyValue {
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
    TexturePath(AiString),
    TextureUvwSource(i32),
    TextureOp(AiTextureOp),
    TextureMapping(AiTextureMapping),
    TextureBlend(f32),
    TextureMappingModeU(AiTextureMapMode),
    TextureMappingModeV(AiTextureMapMode),
    TextureMapAxis(AiVector3D),
    TextureUvTransform(AiUVTransform),
    TextureFlags(AiTextureFlags),
}

impl<'a> MaterialProperty<'a> {
    /// Returns the key of the property.
    pub fn value(&self) -> Option<MaterialPropertyValue> {
        match self.key.as_ref() {
            "?bg.global" => self.string_value().map(MaterialPropertyValue::GlobalBackground),
            "$clr.diffuse" => self.color_value().map(MaterialPropertyValue::DiffuseColor),
            "$clr.specular" => self.color_value().map(MaterialPropertyValue::SpecularColor),
            "$clr.ambient" => self.color_value().map(MaterialPropertyValue::AmbientColor),
            "$clr.emissive" => self.color_value().map(MaterialPropertyValue::EmissiveColor),
            "$clr.transparent" => self.color_value().map(MaterialPropertyValue::TransparentColor),
            "$clr.reflective" => self.color_value().map(MaterialPropertyValue::ReflectiveColor),
            "?mat.name" => self.string_value().map(MaterialPropertyValue::Name),
            "$mat.shadingm" => self.shading_mode_value().map(MaterialPropertyValue::ShadingModel),
            "$mat.twosided" => self.bool_value().map(MaterialPropertyValue::TwoSided),
            "$mat.wireframe" => self.bool_value().map(MaterialPropertyValue::Wireframe),
            "$mat.blend" => self.blend_mode_value().map(MaterialPropertyValue::BlendMode),
            "$mat.opacity" => self.float_value().map(MaterialPropertyValue::Opacity),
            "$mat.bumpscaling" => self.float_value().map(MaterialPropertyValue::BumpScaling),
            "$mat.shininess" => self.float_value().map(MaterialPropertyValue::Shininess),
            "$mat.reflectivity" => self.float_value().map(MaterialPropertyValue::Reflectivity),
            "$mat.shinpercent" => self.float_value().map(MaterialPropertyValue::ShininessStrength),
            "$mat.refracti" => self.float_value().map(MaterialPropertyValue::IndexOfRefraction),
            "$tex.file" => self.string_value().map(MaterialPropertyValue::TexturePath),
            "$tex.uvwsrc" => self.int_value().map(MaterialPropertyValue::TextureUvwSource),
            "$tex.op" => self.texture_op_value().map(MaterialPropertyValue::TextureOp),
            "$tex.mapping" => self.texture_mapping_value().map(MaterialPropertyValue::TextureMapping),
            "$tex.blend" => self.float_value().map(MaterialPropertyValue::TextureBlend),
            "$tex.mapmodeu" => self.texture_map_mode_value().map(MaterialPropertyValue::TextureMappingModeU),
            "$tex.mapmodev" => self.texture_map_mode_value().map(MaterialPropertyValue::TextureMappingModeV),
            "$tex.mapaxis" => self.vector_value().map(MaterialPropertyValue::TextureMapAxis),
            "$tex.uvtrafo" => self.uv_transform_value().map(MaterialPropertyValue::TextureUvTransform),
            "$tex.flags" => self.texture_flags_value().map(MaterialPropertyValue::TextureFlags),
            _ => None,
        }
    }

    pub fn index(&self) -> u32 {
        self.index as u32
    }

    pub fn data_type(&self) -> AiPropertyTypeInfo {
        self.property_type
    }

    fn shading_mode_value(&self) -> Option<AiShadingMode> {
        if let Some(value) = self.int_value() {
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

    fn blend_mode_value(&self) -> Option<AiBlendMode> {
        if let Some(value) = self.int_value() {
            match value {
                0x1 => Some(AiBlendMode::Additive),
                _ => Some(AiBlendMode::Default),
            }
        } else {
            None
        }
    }

    fn texture_op_value(&self) -> Option<AiTextureOp> {
        if let Some(value) = self.int_value() {
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

    fn texture_mapping_value(&self) -> Option<AiTextureMapping> {
        if let Some(value) = self.int_value() {
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

    fn texture_map_mode_value(&self) -> Option<AiTextureMapMode> {
        if let Some(value) = self.int_value() {
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

    fn texture_flags_value(&self) -> Option<AiTextureFlags> {
        if let Some(value) = self.int_value() {
            AiTextureFlags::from_bits(value as u32)
        } else {
            None
        }
    }

    fn uv_transform_value(&self) -> Option<AiUVTransform> {
        let (mat, prop) = self.0;
        if prop.property_type == AiPropertyTypeInfo::Float {
            unsafe {
                let mut out: AiUVTransform = std::mem::zeroed();
                let mut length: u32 = std::mem::size_of_val(&out) as u32;
                aiGetMaterialFloatArray(mat, std::mem::transmute(prop.key.data.as_ptr()), prop.semantic, prop.index, std::mem::transmute(&mut out), &mut length);
                Some(out)
            }
        } else {
            None
        }
    }

    fn bool_value(&self) -> Option<bool> {
        self.int_value().map(|i| {
            match i {
                0 => false,
                _ => true,
            }
        })
    }

    fn float_value(&self) -> Option<f32> {
        let (mat, prop) = self.0;
        if prop.property_type == AiPropertyTypeInfo::Float {
            let mut out = 0.0f32;
            let mut length: u32 = 1;
            unsafe { aiGetMaterialFloatArray(mat, std::mem::transmute(prop.key.data.as_ptr()), prop.semantic, prop.index, &mut out, &mut length); };
            Some(out)
        } else {
            None
        }
    }

    fn vector_value(&self) -> Option<AiVector3D> {
        let (mat, prop) = self.0;
        if prop.property_type == AiPropertyTypeInfo::Float {
            let mut out = AiVector3D { x: 0.0, y: 0.0, z: 0.0 };
            let mut length: u32 = std::mem::size_of::<AiVector3D>() as u32;
            unsafe { aiGetMaterialFloatArray(mat, std::mem::transmute(prop.key.data.as_ptr()), prop.semantic, prop.index, std::mem::transmute(&mut out), &mut length); };
            Some(out)
        } else {
            None
        }
    }

    fn int_value(&self) -> Option<i32> {
        let (mat, prop) = self.0;
        if prop.property_type == AiPropertyTypeInfo::Integer {
            let mut out: i32 = 0;
            unsafe { aiGetMaterialIntegerArray(mat, std::mem::transmute(prop.key.data.as_ptr()), prop.semantic, prop.index, &mut out, null_mut()); };
            Some(out)
        } else {
            None
        }
    }

    fn color_value(&self) -> Option<AiColor4D> {
        let (mat, prop) = self.0;
        if prop.property_type == AiPropertyTypeInfo::Float {
            let mut out = AiColor4D { r: 0.0, g: 0.0, b:0.0, a: 0.0 };
            unsafe { aiGetMaterialColor(mat, std::mem::transmute(prop.key.data.as_ptr()), prop.semantic, prop.index, &mut out); };
            Some(out)
        } else {
            None
        }
    }

    fn string_value(&self) -> Option<AiString> {
        let (mat, prop) = self.0;
        if prop.property_type == AiPropertyTypeInfo::String {
            let mut out = AiString::default();
            unsafe { aiGetMaterialString(mat, std::mem::transmute(prop.key.data.as_ptr()), prop.semantic, prop.index, &mut out); };
            Some(out)
        } else {
            None
        }
    }
}
