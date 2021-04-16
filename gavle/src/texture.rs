use glow::{Context, HasContext};
use std::rc::Rc;
use crate::access::{AccessLock, UnitAccessLock};
use std::num::NonZeroU32;

/** Inner shared structure of the texture. */
#[derive(Debug)]
pub(crate) struct InnerTexture {
	/** Reference to the shared context. */
	pub(crate) context: Rc<Context>,
	/** Name of this texture inside of that context. */
	pub(crate) texture: <Context as HasContext>::Texture,
	/** Access control structure. */
	pub(crate) access: UnitAccessLock,
	/** Format this texture is in. */
	pub(crate) format: TextureFormat,
	/** Extent of this texture. */
	pub(crate) extent: TextureExtent,
}
impl Drop for InnerTexture {
	fn drop(&mut self) {
		unsafe {
			let _atom = self.access.acquire_write_guarded();
			self.context.delete_texture(self.texture)
		}
	}
}
impl AccessLock for InnerTexture {
	fn acquire_write(&self) {
		self.access.acquire_write();
	}
	fn release_write(&self) {
		self.access.release_write();
	}
	fn acquire_read(&self) {
		self.access.acquire_read();
	}
	fn release_read(&self) {
		self.access.release_read();
	}
}

#[derive(Debug)]
pub struct Texture {
	/** The inner shared structure of this texture. */
	pub(crate) inner: Rc<InnerTexture>
}
impl Texture {
	/** The format this texture is stored in. */
	pub fn format(&self) -> TextureFormat {
		self.inner.format
	}
	/** Returns the underlying handle to the texture object. */
	pub unsafe fn as_raw_handle(&self) -> <Context as HasContext>::Texture {
		self.inner.texture
	}
}
impl AccessLock for Texture {
	fn acquire_write(&self) {
		self.inner.access.acquire_write()
	}
	fn release_write(&self) {
		self.inner.access.release_write()
	}
	fn acquire_read(&self) {
		self.inner.access.acquire_read()
	}
	fn release_read(&self) {
		self.inner.access.release_read()
	}
}

/** Formats textures are allowed to have. */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureFormat {
	/** RGBA with a 32-bit floating point for every component. */
	Rgba32Float,
	/** RGBA with an 8-bit unsigned integer for every component. */
	Rgba8Unorm,
	/** Combined depth-stencil format. 24-bit depth and 8-bit stencil. */
	Depth24Stencil8
}


/** Filtering options for textures.
 *
 * The names in this enum are based on the one-dimensional filtering methods,
 * however, when used for higher dimensional textures, they correspond to the
 * higher dimensional equivalent of the method (e.g. "Linear" refers to bilinear
 * filtering in two dimensions and trilinear filtering in three dimensions). */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureFilter {
	/** Nearest neighbor based filtering. */
	Nearest,
	/** Linear based filtering. */
	Linear
}
impl TextureFilter {
	/** Get the OpenGL enum value for the current variant. */
	pub(crate) fn as_opengl(&self) -> u32 {
		match self {
			Self::Nearest => glow::NEAREST,
			Self::Linear => glow::LINEAR,
		}
	}
}

/** Descriptor specifying all of the parameters for a newly created texture. */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TextureDescriptor {
	/** Extent and dimensional layout of this texture. */
	pub extent: TextureExtent,
	/** Format the texture is gonna be stored and uploaded in. */
	pub format: TextureFormat,
	/** How this texture  */
	pub mip: Mipmap,
}

/** Mipmap behavior of a texture. */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Mipmap {
	/** No mips will be generated or available for this texture. */
	None,
	/** The given number of mips will be available, and the user must provide
	 * the image data for each of the mips manually. */
	Manual {
		/** Number of mip levels of the texture. */
		levels: NonZeroU32,
	},
	/** All of the mip levels that can be generated for this texture will be
	 * generated automatically by OpenGL.
	 *
	 * Please don't use this.
	 */
	Automatic,
}

/** Extents of a given texture in their given dimensional layout. */
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TextureExtent {
	/** One-dimensional texture. */
	D1 {
		length: u32,
	},
	/** Two-dimensional texture. */
	D2 {
		width: u32,
		height: u32,
	},
	/** Array of two-dimensional textures. */
	D2Array {
		width: u32,
		height: u32,
		layers: u32,
	},
	/** Three-dimensional texture. */
	D3 {
		width: u32,
		height: u32,
		depth: u32
	}
}

#[derive(Debug, thiserror::Error)]
pub enum TextureError {
	#[error("failed to create a new texture: {what}")]
	CreationError {
		what: String
	},
	#[error("the bounds given to the texture are invalid")]
	InvalidBounds {
		what: String
	}
}
