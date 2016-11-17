use glium::{Program, Vertex, Surface, DrawParameters, DrawError, Frame};
use glium::index::PrimitiveType;
use glium::program::ProgramCreationError;
use glium::uniforms::Uniforms;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::ops::Deref;
use mesh::Mesh;

pub use glium::Display;
pub use std::rc::Rc;


/// Reference to `Graphics`.
#[derive(Clone)]
pub struct Gfx(Rc<Graphics>);

impl Gfx {
    pub fn new(gfx: Graphics) -> Gfx {
        Gfx(Rc::new(gfx))
    }
}


impl Deref for Gfx {
    type Target = Graphics;

    fn deref(&self) -> &Graphics {
        &*self.0
    }
}

/// Graphics context.
pub struct Graphics {
    pub display: Display,
    /// Framebuffer
    /// `Option<Frame>` always `Some`.
    frame: RefCell<Option<Frame>>,
}


impl Graphics {
    pub fn new(display: Display) -> Graphics {
        Graphics {
            frame: RefCell::new(Some(display.draw())),
            display: display,
        }
    }

    pub fn swap_buffers(&self) {
        let mut frame = self.frame.borrow_mut();
        let old_frame: Frame = frame.take().unwrap();
        if let Err(_) = old_frame.finish() {
            unreachable!()
        }
        let mut new_frame = self.display.draw();
        new_frame.clear_color_and_depth((0.0, 0.0, 0.0, 0.0), 0.0);
        *frame = Some(new_frame);
    }

    #[inline]
    pub fn frame<F: Fn(&mut Frame)>(&self, f: F) {
        let mut frame = self.frame.borrow_mut();
        f(frame.as_mut().unwrap());
    }

    pub fn gfx(self) -> Gfx {
        Gfx::new(self)
    }
}


impl Drop for Graphics {
    fn drop(&mut self) {
        let frame = self.frame.borrow_mut().take().unwrap();
        frame.finish().unwrap();
    }
}


pub struct Renderer<S>
    where S: Shader
{
    pub gfx: Gfx,
    program: Program,
    params: DrawParameters<'static>,
    _mark: PhantomData<S>,
}

impl<S: Shader> Renderer<S> {
    pub fn new(gfx: Gfx) -> Result<Renderer<S>, ProgramCreationError> {
        let program = S::build(&gfx.display)?;
        let params = S::draw_parameters();
        let renderer = Renderer {
            gfx: gfx,
            program: program,
            params: params,
            _mark: PhantomData,
        };
        Ok(renderer)
    }

    pub fn draw(&self, mesh: &Mesh<S::Vertex>, uniforms: &S::Uniforms)
        -> Result<(), DrawError>
    {
        let mut frame = self.gfx.frame.borrow_mut();
        frame.as_mut().unwrap().draw(mesh, mesh, &self.program, uniforms, &self.params)
    }
}

/// Shaders, and the parameters associated with it.
pub trait Shader {
    type Vertex: Vertex;
    type Uniforms: Uniforms;

    fn vertex() -> &'static str;

    fn fragment() -> &'static str;

    fn geometry() -> Option<&'static str> {
        None
    }

    fn primitive_type() -> PrimitiveType {
        PrimitiveType::TrianglesList
    }

    fn draw_parameters() -> DrawParameters<'static> {
        use glium::Blend;

        DrawParameters {
            blend: Blend::alpha_blending(),
            ..::std::default::Default::default()
        }
    }

    fn build(display: &Display) -> Result<Program, ProgramCreationError> {
        Program::from_source(display, Self::vertex(), Self::fragment(), Self::geometry())
    }
}
