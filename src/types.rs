/*
** Copyright (c) 2007-2012 The Khronos Group Inc.
** 
** Permission is hereby granted, free of charge, to any person obtaining a
** copy of this software and/or associated documentation files (the
** "Materials"), to deal in the Materials without restriction, including
** without limitation the rights to use, copy, modify, merge, publish,
** distribute, sublicense, and/or sell copies of the Materials, and to
** permit persons to whom the Materials are furnished to do so, subject to
** the following conditions:
** 
** The above copyright notice and this permission notice shall be included
** in all copies or substantial portions of the Materials.
** 
** THE MATERIALS ARE PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
** EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
** MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
** IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
** CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
** TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
** MATERIALS OR THE USE OR OTHER DEALINGS IN THE MATERIALS.
*/

/* glcorearb.h replaces gl3.h. It is for use with OpenGL core
 * profile implementations.
 *
 * glcorearb.h last updated on $Date: 2012-09-19 19:02:24 -0700 (Wed, 19 Sep 2012) $
 *
 * RELEASE NOTES - 2012/09/19
 *
 * glcorearb.h should be placed in the same directory as gl.h and
 * included as
 * '<GL/glcorearb.h>'.
 *
 * glcorearb.h includes only APIs in the latest OpenGL core profile
 * implementation together with APIs in newer ARB extensions which can be
 * can be supported by the core profile. It does not, and never will
 * include functionality removed from the core profile, such as
 * fixed-function vertex and fragment processing.
 *
 * It is not possible to #include both <GL/glcorearb.h> and either of
 * <GL/gl.h> or <GL/glext.h> in the same source file.
 *
 * Feedback can be given by registering for the Khronos Bugzilla
 * (www.khronos.org/bugzilla) and filing issues there under product
 * "OpenGL", category "Registry".
 */

use libc::*;

/* Base GL types */

#[cfg(version_1_0)]
pub mod version_1_0 {
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

#[cfg(version_2_0)]
pub mod version_2_0 {
    pub type GLchar             = c_char;
}

#[cfg(version_1_5)]
pub mod version_1_5 {
    pub type GLintptr           = ptrdiff_t;
    pub type GLsizeiptr         = ptrdiff_t;
}

#[cfg(arb_vertex_buffer_object)]
pub mod arb_vertex_buffer_object {
    /* GL types for handling large vertex buffer objects */
    pub type GLintptrARB        = ptrdiff_t;
    pub type GLsizeiptrARB      = ptrdiff_t;
}

#[cfg(arb_shader_objects)]
pub mod arb_shader_objects {
    /* GL types for program/shader text and shader object handles */
    pub type GLcharARB          = c_char;
    pub type GLhandleARB        = c_uint;
}

#[cfg(arb_half_float_pixel)]
pub mod arb_half_float_pixel {
    /* GL type for "half" precision (s10e5) float data in host memory */
    pub type GLhalfARB          = c_ushort;
}

#[cfg(nv_half_float)]
pub mod nv_half_float {
    pub type GLhalfNV           = c_ushort;
}

#[cfg(ext_timer_query)]
pub mod ext_timer_query {
    pub type GLint64EXT         = int64_t;
    pub type GLuint64EXT        = uint64_t;
}

#[cfg(arb_sync)]
pub mod arb_sync {
    pub type GLint64            = int64_t;
    pub type GLuint64           = uint64_t;
    pub type Struct___GLsync    = c_void;
    pub type GLsync             = *Struct___GLsync;
}

#[cfg(arb_cl_event)]
pub mod arb_cl_event {
    /* These incomplete types let us declare types compatible with OpenCL's cl_context and cl_event */
    pub type Struct__cl_context = c_void;
    pub type Struct__cl_event   = c_void;
}

#[cfg(arb_debug_output)]
pub mod arb_debug_output {
    pub type GLDEBUGPROCARB     = *u8;
}

#[cfg(amd_debug_output)]
pub mod amd_debug_output {
    pub type GLDEBUGPROCAMD     = *u8;
}

#[cfg(khr_debug)]
pub mod khr_debug {
    pub type GLDEBUGPROC        = *u8;
}

#[cfg(nv_vdpau_interop)]
pub mod nv_vdpau_interop {
    pub type GLvdpauSurfaceNV   = GLintptr;
}