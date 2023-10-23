//! A collection of triangle primitives.
use crate::core::Rectangle;
use crate::graphics::mesh;
use crate::graphics::Transformation;

/// A mesh of triangles.
#[derive(Debug, Clone, Copy)]
pub enum Mesh<'a> {
    /// A mesh of triangles with a solid color.
    Solid {
        /// The [`Transformation`] for the vertices of the [`Mesh`].
        transformation: Transformation,

        /// The vertex and index buffers of the [`Mesh`].
        buffers: &'a mesh::Indexed<mesh::SolidVertex2D>,

        /// The clipping bounds of the [`Mesh`].
        clip_bounds: Rectangle<f32>,
    },
    /// A mesh of triangles with a gradient color.
    Gradient {
        /// The [`Transformation`] for the vertices of the [`Mesh`].
        transformation: Transformation,

        /// The vertex and index buffers of the [`Mesh`].
        buffers: &'a mesh::Indexed<mesh::GradientVertex2D>,

        /// The clipping bounds of the [`Mesh`].
        clip_bounds: Rectangle<f32>,
    },
}

impl Mesh<'_> {
    /// Returns the origin of the [`Mesh`].
    pub fn transformation(&self) -> Transformation {
        match self {
            Self::Solid { transformation, .. }
            | Self::Gradient { transformation, .. } => *transformation,
        }
    }

    /// Returns the indices of the [`Mesh`].
    pub fn indices(&self) -> &[u32] {
        match self {
            Self::Solid { buffers, .. } => &buffers.indices,
            Self::Gradient { buffers, .. } => &buffers.indices,
        }
    }

    /// Returns the clip bounds of the [`Mesh`].
    pub fn clip_bounds(&self) -> Rectangle<f32> {
        match self {
            Self::Solid { clip_bounds, .. }
            | Self::Gradient { clip_bounds, .. } => *clip_bounds,
        }
    }
}

/// The result of counting the attributes of a set of meshes.
#[derive(Debug, Clone, Copy, Default)]
pub struct AttributeCount {
    /// The total amount of solid vertices.
    pub solid_vertices: usize,

    /// The total amount of solid meshes.
    pub solids: usize,

    /// The total amount of gradient vertices.
    pub gradient_vertices: usize,

    /// The total amount of gradient meshes.
    pub gradients: usize,

    /// The total amount of indices.
    pub indices: usize,
}

/// Returns the number of total vertices & total indices of all [`Mesh`]es.
pub fn attribute_count_of<'a>(meshes: &'a [Mesh<'a>]) -> AttributeCount {
    meshes
        .iter()
        .fold(AttributeCount::default(), |mut count, mesh| {
            match mesh {
                Mesh::Solid { buffers, .. } => {
                    count.solids += 1;
                    count.solid_vertices += buffers.vertices.len();
                    count.indices += buffers.indices.len();
                }
                Mesh::Gradient { buffers, .. } => {
                    count.gradients += 1;
                    count.gradient_vertices += buffers.vertices.len();
                    count.indices += buffers.indices.len();
                }
            }

            count
        })
}
