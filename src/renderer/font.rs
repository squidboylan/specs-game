use gl::types::*;
use std::path::Path;
use freetype::*;

type Texture = GLuint;
type Vbo = GLuint;
type Vao = GLuint;

pub struct Font {
    pub font_textures: [Texture; 128],
    pub glyphs: Vec<freetype::glyph_slot::GlyphSlot>,
    pub bitmaps: Vec<freetype::bitmap::Bitmap>,
}


impl Font {
    pub fn new(p: &Path) -> Self {
        let mut font_textures = [0; 128];
        let lib = library::Library::init().unwrap();
        let face = lib.new_face(p, 0).unwrap();
        let mut glyphs = Vec::new();
        let mut bitmaps = Vec::new();

        face.set_char_size(40 * 64, 0, 50, 0).unwrap();
        for i in 0..128 {
            face.load_char(i, freetype::face::LoadFlag::RENDER).unwrap();
            let glyph = face.glyph().clone();
            let bitmap = glyph.bitmap();
            println!("{}: width: {}, rows: {}, buffer_size: {}, mode: {:?}", i, bitmap.width(), bitmap.rows(), bitmap.buffer().len(), bitmap.pixel_mode().unwrap());
            println!("{}: left: {}, top: {}", i, glyph.bitmap_left(), glyph.bitmap_top());

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
            glyphs.push(glyph);
            bitmaps.push(bitmap);
            font_textures[i] = texture;
        }
        Font {
            font_textures,
            glyphs,
            bitmaps,
        }
    }
}
