use super::BasicParams;
use crate::core::{
    context,
    program::{Attribute, ParamsBase, ParamsVisitor, Program, Uniform},
    shader::{FragmentShader, VertexShader},
    vec::StepVec,
};
use cgmath::Vector2;
use wasm_bindgen::prelude::*;

pub struct TextureProgram {
    inner: Program<TextureParams>,
}

impl TextureProgram {
    /// フォンシェーディング版のTextureProgramを生成する
    pub fn phong() -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("texture-phong.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("texture-phong.frag"))?;

        let inner = Program::<TextureParams>::new(vert_shader, frag_shader)?;

        Ok(TextureProgram { inner })
    }

    /// グーローシェーディング版のTextureProgramを生成する
    pub fn gouraud() -> Result<Self, JsValue> {
        let vert_shader = VertexShader::compile(include_str!("texture-gouraud.vert"))?;
        let frag_shader = FragmentShader::compile(include_str!("texture-gouraud.frag"))?;

        let inner = Program::<TextureParams>::new(vert_shader, frag_shader)?;

        Ok(TextureProgram { inner })
    }

    pub(crate) fn params(&self) -> &TextureParams {
        &self.inner.params
    }

    pub(crate) fn params_mut(&mut self) -> &mut TextureParams {
        &mut self.inner.params
    }

    pub(crate) fn switch(&self) {
        context::with(|ctx| ctx.switch_program(&self.inner))
    }
}

pub struct TextureParams {
    pub basic: BasicParams,
    pub tex_coord: Attribute<StepVec<Vector2<f32>>>,
    pub texture: Uniform<i32>,
}

impl ParamsBase for TextureParams {
    fn from_visitor<'a>(visitor: &mut ParamsVisitor<'a>) -> Result<Self, JsValue> {
        Ok(TextureParams {
            basic: BasicParams::from_visitor(visitor)?,
            tex_coord: visitor.visit_attr("texCoord")?,
            texture: visitor.visit_uniform("uTexture")?,
        })
    }
}

impl AsRef<BasicParams> for TextureParams {
    fn as_ref(&self) -> &BasicParams {
        &self.basic
    }
}

impl AsMut<BasicParams> for TextureParams {
    fn as_mut(&mut self) -> &mut BasicParams {
        &mut self.basic
    }
}
