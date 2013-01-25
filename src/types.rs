use libc::*;

/* Base GL types */

#[cfg(GL_VERSION_1_0)]
pub mod GL_VERSION_1_0 {
    pub type GLenum             = libc::c_uint;
    pub type GLboolean          = libc::c_uchar;
    pub type GLbitfield         = libc::c_uint;
    pub type GLbyte             = libc::c_schar;
    pub type GLshort            = libc::c_short;
    pub type GLint              = libc::c_int;
    pub type GLsizei            = libc::c_int;
    pub type GLubyte            = libc::c_uchar;
    pub type GLushort           = libc::c_ushort;
    pub type GLuint             = libc::c_uint;
    pub type GLhalf             = libc::c_ushort;
    pub type GLfloat            = libc::c_float;
    pub type GLclampf           = libc::c_float;
    pub type GLdouble           = libc::c_double;
    pub type GLclampd           = libc::c_double;
    pub type GLvoid             = libc::c_void;
}

/*************************************************************/

#[cfg(GL_VERSION_2_0)]
pub mod GL_VERSION_2_0 {
    pub type GLchar             = libc::c_char;
}

#[cfg(GL_VERSION_1_5)]
pub mod GL_VERSION_1_5 {
    pub type GLintptr           = libc::ptrdiff_t;
    pub type GLsizeiptr         = libc::ptrdiff_t;
}

#[cfg(GL_ARB_vertex_buffer_object)]
pub mod GL_ARB_vertex_buffer_object {
    /* GL types for handling large vertex buffer objects */
    pub type GLintptrARB        = libc::ptrdiff_t;
    pub type GLsizeiptrARB      = libc::ptrdiff_t;
}

#[cfg(GL_ARB_shader_objects)]
pub mod GL_ARB_shader_objects {
    /* GL types for program/shader text and shader object handles */
    pub type GLcharARB          = libc::c_char;
    pub type GLhandleARB        = libc::c_uint;
}

#[cfg(GL_ARB_half_float_pixel)]
pub mod GL_ARB_half_float_pixel {
    /* GL type for "half" precision (s10e5) float data in host memory */
    pub type GLhalfARB          = libc::c_ushort;
}

#[cfg(GL_NV_half_float)]
pub mod GL_NV_half_float {
    pub type GLhalfNV           = libc::c_ushort;
}

#[cfg(GL_EXT_timer_query)]
pub mod GL_EXT_timer_query {
    pub type GLint64EXT         = i64;
    pub type GLuint64EXT        = u64;
}

#[cfg(GL_ARB_sync)]
pub mod GL_ARB_sync {
    pub type GLint64            = i64;
    pub type GLuint64           = u64;
    pub struct _GLsync {}
    pub type GLsync             = *_GLsync;
}

#[cfg(GL_ARB_cl_event)]
pub mod GL_ARB_cl_event {
    /* These incomplete types let us declare types compatible with OpenCL's cl_context and cl_event */
    pub struct cl_context {}
    pub struct cl_event {}
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