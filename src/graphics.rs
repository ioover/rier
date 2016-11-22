use glium::{Program, Vertex, Surface, DrawParameters, DrawError, SwapBuffersError, Frame};
use glium::program::ProgramCreationError;
use glium::uniforms::Uniforms;
use std::marker::PhantomData;
use std::cell::RefCell;
use std::ops::Deref;
use mesh::Mesh;

pub use glium::Display;
pub use std::rc::Rc;


pub struct Renderer<S, T>
    where S: Shader, T: Surface
{
    target: Target<T>,
    pub display: Display,
    program: Program,
    params: DrawParameters<'static>,
    _mark: PhantomData<S>,
}

impl<S: Shader, T: Surface> Renderer<S, T> {
    pub fn new(display: Display, target: Target<T>) -> Result<Renderer<S, T>, ProgramCreationError> {
        let program = S::build(&display)?;
        let params = S::draw_parameters();
        let renderer = Renderer {
            target: target,
            display: display,
            program: program,
            params: params,
            _mark: PhantomData,
        };
        Ok(renderer)
    }

    pub fn draw(&self, mesh: &Mesh<S::Vertex>, uniforms: &S::Uniforms)
        -> Result<(), DrawError>
    {
        let mut frame = self.target.borrow_mut();
        frame.draw(mesh, mesh, &self.program, uniforms, &self.params)
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


pub struct Target<T: Surface>(Rc<RefCell<T>>);

impl<T: Surface> Clone for Target<T> {
    fn clone(&self) -> Target<T> {
        Target(self.0.clone())
    }
}

impl<T: Surface> Target<T> {
    pub fn from_surface(surface: T) -> Target<T> {
        Target(Rc::new(RefCell::new(surface)))
    }
}

impl<T: Surface> Deref for Target<T> {
    type Target = RefCell<T>;
    fn deref(&self) -> &RefCell<T> { self.0.deref() }
} 


impl Target<Frame> {
    pub fn swap_buffers(&self, display: &Display) -> Result<(), SwapBuffersError> {
        let mut frame = self.borrow_mut();
        frame.set_finish()?;
        *frame = display.draw();
        Ok(())
    }

    pub fn finish(&self) -> Result<(), SwapBuffersError> {
        self.borrow_mut().set_finish()
    }
}