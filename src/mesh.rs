//! Polygon mesh.

use glium::index::{NoIndices, IndicesSource};
use glium::vertex::{IntoVerticesSource, VerticesSource};
use graphics::{Shader, Renderer};

pub use glium::{VertexBuffer, Vertex};
pub use glium::index::BufferCreationError as IndexCreationError;
pub use glium::vertex::BufferCreationError as VertexCreationError;


/// Vertex index
pub type Index = u16;
pub type IndexBuffer = ::glium::IndexBuffer<Index>;


pub struct Mesh<T: Vertex> {
    /// Vertex buffer.
    vertices: VertexBuffer<T>,
    /// Index buffer or none.
    indices: Indices,
}


impl<T: Vertex> Mesh<T> {
    /// Creates a simple mesh object.
    /// Primitive type is triangles list, no indices need.
    pub fn new<S>(renderer: &Renderer<S>, vertices: &[T]) -> Result<Mesh<T>, VertexCreationError>
        where S: Shader<Vertex = T>
    {
        Ok(Mesh {
            vertices: VertexBuffer::new(&renderer.gfx.display, vertices)?,
            indices: Indices::None(NoIndices(S::primitive_type())),
        })
    }

    pub fn with_indices<S>(renderer: &Renderer<S>, vertices: &[T], indices: &[Index])
        -> Result<Mesh<T>, CreationError> where S: Shader<Vertex = T>
    {
        let vertices = VertexBuffer::new(&renderer.gfx.display, vertices)?;
        let indices = IndexBuffer::new(&renderer.gfx.display, S::primitive_type(), indices)?;
        Ok(Mesh {
            vertices: vertices,
            indices: Indices::Buffer(indices),
        })
    }

    /// Create a mesh with the given buffers.
    pub fn from_buffers(vertices: VertexBuffer<T>, indices: IndexBuffer) -> Mesh<T> {
        Mesh {
            vertices: vertices,
            indices: Indices::Buffer(indices),
        }
    }
}

enum Indices {
    None(NoIndices),
    Buffer(IndexBuffer),
}


impl<'a, T: Vertex> Into<IndicesSource<'a>> for &'a Mesh<T> {
    fn into(self) -> IndicesSource<'a> {
        match self.indices {
            Indices::None(ref x) => x.into(),
            Indices::Buffer(ref x) => x.into(),
        }
    }
}


impl<'a, T: Vertex> IntoVerticesSource<'a> for &'a Mesh<T> {
    fn into_vertices_source(self) -> VerticesSource<'a> {
        self.vertices.into_vertices_source()
    }
}


/// Errors which can occur when attempting to create a mesh.
#[derive(Debug)]
pub enum CreationError {
    /// Vertex buffer create failure.
    Vertex(VertexCreationError),
    /// Index buffer create failure.
    Index(IndexCreationError),
}


impl From<IndexCreationError> for CreationError {
    fn from(err: IndexCreationError) -> CreationError {
        CreationError::Index(err)
    }
}


impl From<VertexCreationError> for CreationError {
    fn from(err: VertexCreationError) -> CreationError {
        CreationError::Vertex(err)
    }
}
