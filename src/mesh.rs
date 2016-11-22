//! Polygon mesh.

use glium::Display;
use glium::index::{NoIndices, IndicesSource, PrimitiveType};
use glium::vertex::{IntoVerticesSource, VerticesSource};

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
    pub fn new(display: &Display, primitive_type: PrimitiveType, vertices: &[T])
        -> Result<Mesh<T>, VertexCreationError>
    {
        Ok(Mesh {
            vertices: VertexBuffer::new(display, vertices)?,
            indices: Indices::None(NoIndices(primitive_type)),
        })
    }

    pub fn with_indices(display: &Display, primitive_type: PrimitiveType, vertices: &[T], indices: &[Index])
        -> Result<Mesh<T>, CreationError>
    {
        let vertices = VertexBuffer::new(display, vertices)?;
        let indices = IndexBuffer::new(display, primitive_type, indices)?;
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
