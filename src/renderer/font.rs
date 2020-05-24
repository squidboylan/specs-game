use gl::types::*;
use std::path::Path;
use freetype::*;

type Texture = GLuint;
type Vbo = GLuint;
type Vao = GLuint;

pub struct Font {
    pub glyphs: [Glyph; 128],
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Glyph {
    pub texture: Texture,
    pub left: f32,
    pub top: f32,
    pub w: f32,
    pub h: f32,
    pub advance: (f32, f32),
}

impl Font {
    pub fn new(p: &Path) -> Self {
        let lib = library::Library::init().unwrap();
        let face = lib.new_face(p, 0).unwrap();
        let mut glyphs = [Glyph::default(); 128];

        face.set_char_size(40 * 64, 0, 50, 0).unwrap();
        for i in 0..128 {
            face.load_char(i, freetype::face::LoadFlag::RENDER).unwrap();
            let glyph = face.glyph();
            let bitmap = glyph.bitmap();
            glyphs[i].left = glyph.bitmap_left() as f32;
            glyphs[i].top = glyph.bitmap_top() as f32;
            glyphs[i].w = bitmap.width() as f32;
            glyphs[i].h = bitmap.rows() as f32;
            let a = glyph.advance();
            glyphs[i].advance = (a.x as f32, a.y as f32);
            println!("index: {}, {:?}", i, glyphs[i]);

            let mut texture = 0;
            unsafe {
                gl::GenTextures(1, &mut texture);
                gl::BindTexture(gl::TEXTURE_2D, texture);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
                gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
                gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RED as GLint, bitmap.width() as GLsizei, bitmap.rows() as GLsizei,
                    0, gl::RED, gl::UNSIGNED_BYTE, bitmap.buffer().as_ptr() as *const GLvoid);
            }
            glyphs[i].texture = texture;
        }
        Font {
            glyphs,
        }
    }
}
