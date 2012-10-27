use libc::*;

/* Base GL types */

#[cfg(GL_VERSION_1_0)]
pub mod GL_VERSION_1_0 {
    pub type GLenum             = c_uint;
    pub type GLboolean          = c_uchar;
    pub type GLbitfield         = c_uint;
    pub type GLbyte             = c_schar;
    pub type GLshort            = c_short;
    pub type GLint              = c_int;
    pub type GLsizei            = c_int;
    pub type GLubyte            = c_uchar;
    pub type GLushort           = c_ushort;
    pub type GLuint             = c_uint;
    pub type GLhalf             = c_ushort;
    pub type GLfloat            = c_float;
    pub type GLclampf           = c_float;
    pub type GLdouble           = c_double;
    pub type GLclampd           = c_double;
    pub type GLvoid             = c_void;
}

/*************************************************************/

#[cfg(GL_VERSION_2_0)]
pub mod GL_VERSION_2_0 {
    pub type GLchar             = c_char;
}

#[cfg(GL_VERSION_1_5)]
pub mod GL_VERSION_1_5 {
    pub type GLintptr           = ptrdiff_t;
    pub type GLsizeiptr         = ptrdiff_t;
}

#[cfg(GL_ARB_vertex_buffer_object)]
pub mod GL_ARB_vertex_buffer_object {
    /* GL types for handling large vertex buffer objects */
    pub type GLintptrARB        = ptrdiff_t;
    pub type GLsizeiptrARB      = ptrdiff_t;
}

#[cfg(GL_ARB_shader_objects)]
pub mod GL_ARB_shader_objects {
    /* GL types for program/shader text and shader object handles */
    pub type GLcharARB          = c_char;
    pub type GLhandleARB        = c_uint;
}

#[cfg(GL_ARB_half_float_pixel)]
pub mod GL_ARB_half_float_pixel {
    /* GL type for "half" precision (s10e5) float data in host memory */
    pub type GLhalfARB          = c_ushort;
}

#[cfg(GL_NV_half_float)]
pub mod GL_NV_half_float {
    pub type GLhalfNV           = c_ushort;
}

#[cfg(GL_EXT_timer_query)]
pub mod GL_EXT_timer_query {
    pub type GLint64EXT         = int64_t;
    pub type GLuint64EXT        = uint64_t;
}

#[cfg(GL_ARB_sync)]
pub mod GL_ARB_sync {
    pub type GLint64            = int64_t;
    pub type GLuint64           = uint64_t;
    pub type Struct___GLsync    = c_void;
    pub type GLsync             = *Struct___GLsync;
}

#[cfg(GL_ARB_cl_event)]
pub mod GL_ARB_cl_event {
    /* These incomplete types let us declare types compatible with OpenCL's cl_context and cl_event */
    pub type Struct__cl_context = c_void;
    pub type Struct__cl_event   = c_void;
}

#[cfg(GL_ARB_debug_output)]
pub mod GL_ARB_debug_output {
    pub type GLDEBUGPROCARB     = *u8;
}

#[cfg(GL_AMD_debug_output)]
pub mod GL_AMD_debug_output {
    pub type GLDEBUGPROCAMD     = *u8;
}

#[cfg(GL_KHR_debug)]
pub mod GL_KHR_debug {
    pub type GLDEBUGPROC        = *u8;
}

#[cfg(GL_NV_vdpau_interop)]
pub mod GL_NV_vdpau_interop {
    pub type GLvdpauSurfaceNV   = GLintptr;
}