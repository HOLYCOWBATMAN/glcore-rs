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

#[nolink]
#[link_args="-framework OpenGL"]
#[cfg(target_os = "macos")]
extern mod linkhack {}

#[nolink]
#[link_args="-lGL"]
#[cfg(target_os = "linux")]
extern mod linkhack {}

pub mod version_1_0 {
    #[nolink]
    extern {
        fn glCullFace(++mode: GLenum);
        fn glFrontFace(++mode: GLenum);
        fn glHint(++target: GLenum, ++mode: GLenum);
        fn glLineWidth(++width: GLfloat);
        fn glPointSize(++size: GLfloat);
        fn glPolygonMode(++face: GLenum, ++mode: GLenum);
        fn glScissor(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);
        fn glTexParameterf(++target: GLenum, ++pname: GLenum, ++param: GLfloat);
        fn glTexParameterfv(++target: GLenum, ++pname: GLenum, ++params: *GLfloat);
        fn glTexParameteri(++target: GLenum, ++pname: GLenum, ++param: GLint);
        fn glTexParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glTexImage1D(++target: GLenum, ++level: GLint, ++internalformat: GLint, ++width: GLsizei, ++border: GLint, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLint, ++width: GLsizei, ++height: GLsizei, ++border: GLint, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glDrawBuffer(++mode: GLenum);
        fn glClear(++mask: GLbitfield);
        fn glClearColor(++red: GLfloat, ++green: GLfloat, ++blue: GLfloat, ++alpha: GLfloat);
        fn glClearStencil(++s: GLint);
        fn glClearDepth(++depth: GLdouble);
        fn glStencilMask(++mask: GLuint);
        fn glColorMask(++red: GLboolean, ++green: GLboolean, ++blue: GLboolean, ++alpha: GLboolean);
        fn glDepthMask(++flag: GLboolean);
        fn glDisable(++cap: GLenum);
        fn glEnable(++cap: GLenum);
        fn glFinish();
        fn glFlush();
        fn glBlendFunc(++sfactor: GLenum, ++dfactor: GLenum);
        fn glLogicOp(++opcode: GLenum);
        fn glStencilFunc(++func: GLenum, ++glref: GLint, ++mask: GLuint);
        fn glStencilOp(++glfail: GLenum, ++zfail: GLenum, ++zpass: GLenum);
        fn glDepthFunc(++func: GLenum);
        fn glPixelStoref(++pname: GLenum, ++param: GLfloat);
        fn glPixelStorei(++pname: GLenum, ++param: GLint);
        fn glReadBuffer(++mode: GLenum);
        fn glReadPixels(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glGetBooleanv(++pname: GLenum, ++params: *GLboolean);
        fn glGetDoublev(++pname: GLenum, ++params: *GLdouble);
        fn glGetError() -> GLenum;
        fn glGetFloatv(++pname: GLenum, ++params: *GLfloat);
        fn glGetIntegerv(++pname: GLenum, ++params: *GLint);
        fn glGetString(++name: GLenum) -> *GLubyte;
        fn glGetTexImage(++target: GLenum, ++level: GLint, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glGetTexParameterfv(++target: GLenum, ++pname: GLenum, ++params: *GLfloat);
        fn glGetTexParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glGetTexLevelParameterfv(++target: GLenum, ++level: GLint, ++pname: GLenum, ++params: *GLfloat);
        fn glGetTexLevelParameteriv(++target: GLenum, ++level: GLint, ++pname: GLenum, ++params: *GLint);
        fn glIsEnabled(++cap: GLenum) -> GLboolean;
        fn glDepthRange(++near: GLdouble, ++far: GLdouble);
        fn glViewport(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);
    }
    type PFNGLCULLFACEPROC = *u8;
    type PFNGLFRONTFACEPROC = *u8;
    type PFNGLHINTPROC = *u8;
    type PFNGLLINEWIDTHPROC = *u8;
    type PFNGLPOINTSIZEPROC = *u8;
    type PFNGLPOLYGONMODEPROC = *u8;
    type PFNGLSCISSORPROC = *u8;
    type PFNGLTEXPARAMETERFPROC = *u8;
    type PFNGLTEXPARAMETERFVPROC = *u8;
    type PFNGLTEXPARAMETERIPROC = *u8;
    type PFNGLTEXPARAMETERIVPROC = *u8;
    type PFNGLTEXIMAGE1DPROC = *u8;
    type PFNGLTEXIMAGE2DPROC = *u8;
    type PFNGLDRAWBUFFERPROC = *u8;
    type PFNGLCLEARPROC = *u8;
    type PFNGLCLEARCOLORPROC = *u8;
    type PFNGLCLEARSTENCILPROC = *u8;
    type PFNGLCLEARDEPTHPROC = *u8;
    type PFNGLSTENCILMASKPROC = *u8;
    type PFNGLCOLORMASKPROC = *u8;
    type PFNGLDEPTHMASKPROC = *u8;
    type PFNGLDISABLEPROC = *u8;
    type PFNGLENABLEPROC = *u8;
    type PFNGLFINISHPROC = *u8;
    type PFNGLFLUSHPROC = *u8;
    type PFNGLBLENDFUNCPROC = *u8;
    type PFNGLLOGICOPPROC = *u8;
    type PFNGLSTENCILFUNCPROC = *u8;
    type PFNGLSTENCILOPPROC = *u8;
    type PFNGLDEPTHFUNCPROC = *u8;
    type PFNGLPIXELSTOREFPROC = *u8;
    type PFNGLPIXELSTOREIPROC = *u8;
    type PFNGLREADBUFFERPROC = *u8;
    type PFNGLREADPIXELSPROC = *u8;
    type PFNGLGETBOOLEANVPROC = *u8;
    type PFNGLGETDOUBLEVPROC = *u8;
    type PFNGLGETERRORPROC = *u8;
    type PFNGLGETFLOATVPROC = *u8;
    type PFNGLGETINTEGERVPROC = *u8;
    type PFNGLGETSTRINGPROC = *u8;
    type PFNGLGETTEXIMAGEPROC = *u8;
    type PFNGLGETTEXPARAMETERFVPROC = *u8;
    type PFNGLGETTEXPARAMETERIVPROC = *u8;
    type PFNGLGETTEXLEVELPARAMETERFVPROC = *u8;
    type PFNGLGETTEXLEVELPARAMETERIVPROC = *u8;
    type PFNGLISENABLEDPROC = *u8;
    type PFNGLDEPTHRANGEPROC = *u8;
    type PFNGLVIEWPORTPROC = *u8;
}

pub mod version_1_1 {
    #[nolink]
    extern {
        fn glDrawArrays(++mode: GLenum, ++first: GLint, ++count: GLsizei);
        fn glDrawElements(++mode: GLenum, ++count: GLsizei, ++gltype: GLenum, ++indices: *GLvoid);
        fn glGetPointerv(++pname: GLenum, ++params: **GLvoid);
        fn glPolygonOffset(++factor: GLfloat, ++units: GLfloat);
        fn glCopyTexImage1D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++x: GLint, ++y: GLint, ++width: GLsizei, ++border: GLint);
        fn glCopyTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++border: GLint);
        fn glCopyTexSubImage1D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++x: GLint, ++y: GLint, ++width: GLsizei);
        fn glCopyTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);
        fn glTexSubImage1D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++width: GLsizei, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glBindTexture(++target: GLenum, ++texture: GLuint);
        fn glDeleteTextures(++n: GLsizei, ++textures: *GLuint);
        fn glGenTextures(++n: GLsizei, ++textures: *GLuint);
        fn glIsTexture(++texture: GLuint) -> GLboolean;
    }
    type PFNGLDRAWARRAYSPROC = *u8;
    type PFNGLDRAWELEMENTSPROC = *u8;
    type PFNGLGETPOINTERVPROC = *u8;
    type PFNGLPOLYGONOFFSETPROC = *u8;
    type PFNGLCOPYTEXIMAGE1DPROC = *u8;
    type PFNGLCOPYTEXIMAGE2DPROC = *u8;
    type PFNGLCOPYTEXSUBIMAGE1DPROC = *u8;
    type PFNGLCOPYTEXSUBIMAGE2DPROC = *u8;
    type PFNGLTEXSUBIMAGE1DPROC = *u8;
    type PFNGLTEXSUBIMAGE2DPROC = *u8;
    type PFNGLBINDTEXTUREPROC = *u8;
    type PFNGLDELETETEXTURESPROC = *u8;
    type PFNGLGENTEXTURESPROC = *u8;
    type PFNGLISTEXTUREPROC = *u8;
}

pub mod version_1_2 {
    #[nolink]
    extern {
        fn glBlendColor(++red: GLfloat, ++green: GLfloat, ++blue: GLfloat, ++alpha: GLfloat);
        fn glBlendEquation(++mode: GLenum);
        fn glDrawRangeElements(++mode: GLenum, ++start: GLuint, ++end: GLuint, ++count: GLsizei, ++gltype: GLenum, ++indices: *GLvoid);
        fn glTexImage3D(++target: GLenum, ++level: GLint, ++internalformat: GLint, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++border: GLint, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glTexSubImage3D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++zoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++format: GLenum, ++gltype: GLenum, ++pixels: *GLvoid);
        fn glCopyTexSubImage3D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++zoffset: GLint, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);
    }
    type PFNGLBLENDCOLORPROC = *u8;
    type PFNGLBLENDEQUATIONPROC = *u8;
    type PFNGLDRAWRANGEELEMENTSPROC = *u8;
    type PFNGLTEXIMAGE3DPROC = *u8;
    type PFNGLTEXSUBIMAGE3DPROC = *u8;
    type PFNGLCOPYTEXSUBIMAGE3DPROC = *u8;
}

pub mod version_1_3 {
    #[nolink]
    extern {
        fn glActiveTexture(++texture: GLenum);
        fn glSampleCoverage(++value: GLfloat, ++invert: GLboolean);
        fn glCompressedTexImage3D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++border: GLint, ++imageSize: GLsizei, ++data: *GLvoid);
        fn glCompressedTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++border: GLint, ++imageSize: GLsizei, ++data: *GLvoid);
        fn glCompressedTexImage1D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++width: GLsizei, ++border: GLint, ++imageSize: GLsizei, ++data: *GLvoid);
        fn glCompressedTexSubImage3D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++zoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++format: GLenum, ++imageSize: GLsizei, ++data: *GLvoid);
        fn glCompressedTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++imageSize: GLsizei, ++data: *GLvoid);
        fn glCompressedTexSubImage1D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++width: GLsizei, ++format: GLenum, ++imageSize: GLsizei, ++data: *GLvoid);
        fn glGetCompressedTexImage(++target: GLenum, ++level: GLint, ++img: *GLvoid);
    }
    type PFNGLACTIVETEXTUREPROC = *u8;
    type PFNGLSAMPLECOVERAGEPROC = *u8;
    type PFNGLCOMPRESSEDTEXIMAGE3DPROC = *u8;
    type PFNGLCOMPRESSEDTEXIMAGE2DPROC = *u8;
    type PFNGLCOMPRESSEDTEXIMAGE1DPROC = *u8;
    type PFNGLCOMPRESSEDTEXSUBIMAGE3DPROC = *u8;
    type PFNGLCOMPRESSEDTEXSUBIMAGE2DPROC = *u8;
    type PFNGLCOMPRESSEDTEXSUBIMAGE1DPROC = *u8;
    type PFNGLGETCOMPRESSEDTEXIMAGEPROC = *u8;
}

pub mod version_1_4 {
    #[nolink]
    extern {
        fn glBlendFuncSeparate(++sfactorRGB: GLenum, ++dfactorRGB: GLenum, ++sfactorAlpha: GLenum, ++dfactorAlpha: GLenum);
        fn glMultiDrawArrays(++mode: GLenum, ++first: *GLint, ++count: *GLsizei, ++drawcount: GLsizei);
        fn glMultiDrawElements(++mode: GLenum, ++count: *GLsizei, ++gltype: GLenum, ++indices: **GLvoid, ++drawcount: GLsizei);
        fn glPointParameterf(++pname: GLenum, ++param: GLfloat);
        fn glPointParameterfv(++pname: GLenum, ++params: *GLfloat);
        fn glPointParameteri(++pname: GLenum, ++param: GLint);
        fn glPointParameteriv(++pname: GLenum, ++params: *GLint);
    }
    type PFNGLBLENDFUNCSEPARATEPROC = *u8;
    type PFNGLMULTIDRAWARRAYSPROC = *u8;
    type PFNGLMULTIDRAWELEMENTSPROC = *u8;
    type PFNGLPOINTPARAMETERFPROC = *u8;
    type PFNGLPOINTPARAMETERFVPROC = *u8;
    type PFNGLPOINTPARAMETERIPROC = *u8;
    type PFNGLPOINTPARAMETERIVPROC = *u8;
}

pub mod version_1_5 {
    #[nolink]
    extern {
        fn glGenQueries(++n: GLsizei, ++ids: *GLuint);
        fn glDeleteQueries(++n: GLsizei, ++ids: *GLuint);
        fn glIsQuery(++id: GLuint) -> GLboolean;
        fn glBeginQuery(++target: GLenum, ++id: GLuint);
        fn glEndQuery(++target: GLenum);
        fn glGetQueryiv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glGetQueryObjectiv(++id: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetQueryObjectuiv(++id: GLuint, ++pname: GLenum, ++params: *GLuint);
        fn glBindBuffer(++target: GLenum, ++buffer: GLuint);
        fn glDeleteBuffers(++n: GLsizei, ++buffers: *GLuint);
        fn glGenBuffers(++n: GLsizei, ++buffers: *GLuint);
        fn glIsBuffer(++buffer: GLuint) -> GLboolean;
        fn glBufferData(++target: GLenum, ++size: GLsizeiptr, ++data: *GLvoid, ++usage: GLenum);
        fn glBufferSubData(++target: GLenum, ++offset: GLintptr, ++size: GLsizeiptr, ++data: *GLvoid);
        fn glGetBufferSubData(++target: GLenum, ++offset: GLintptr, ++size: GLsizeiptr, ++data: *GLvoid);
        fn glMapBuffer(++target: GLenum, ++access: GLenum) -> *GLvoid;
        fn glUnmapBuffer(++target: GLenum) -> GLboolean;
        fn glGetBufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glGetBufferPointerv(++target: GLenum, ++pname: GLenum, ++params: **GLvoid);
    }
    type PFNGLGENQUERIESPROC = *u8;
    type PFNGLDELETEQUERIESPROC = *u8;
    type PFNGLISQUERYPROC = *u8;
    type PFNGLBEGINQUERYPROC = *u8;
    type PFNGLENDQUERYPROC = *u8;
    type PFNGLGETQUERYIVPROC = *u8;
    type PFNGLGETQUERYOBJECTIVPROC = *u8;
    type PFNGLGETQUERYOBJECTUIVPROC = *u8;
    type PFNGLBINDBUFFERPROC = *u8;
    type PFNGLDELETEBUFFERSPROC = *u8;
    type PFNGLGENBUFFERSPROC = *u8;
    type PFNGLISBUFFERPROC = *u8;
    type PFNGLBUFFERDATAPROC = *u8;
    type PFNGLBUFFERSUBDATAPROC = *u8;
    type PFNGLGETBUFFERSUBDATAPROC = *u8;
    type PFNGLMAPBUFFERPROC = *u8;
    type PFNGLUNMAPBUFFERPROC = *u8;
    type PFNGLGETBUFFERPARAMETERIVPROC = *u8;
    type PFNGLGETBUFFERPOINTERVPROC = *u8;
}

pub mod version_2_0 {
    #[nolink]
    extern {
        fn glBlendEquationSeparate(++modeRGB: GLenum, ++modeAlpha: GLenum);
        fn glDrawBuffers(++n: GLsizei, ++bufs: *GLenum);
        fn glStencilOpSeparate(++face: GLenum, ++sfail: GLenum, ++dpfail: GLenum, ++dppass: GLenum);
        fn glStencilFuncSeparate(++face: GLenum, ++func: GLenum, ++glref: GLint, ++mask: GLuint);
        fn glStencilMaskSeparate(++face: GLenum, ++mask: GLuint);
        fn glAttachShader(++program: GLuint, ++shader: GLuint);
        fn glBindAttribLocation(++program: GLuint, ++index: GLuint, ++name: *GLchar);
        fn glCompileShader(++shader: GLuint);
        fn glCreateProgram() -> GLuint;
        fn glCreateShader(++gltype: GLenum) -> GLuint;
        fn glDeleteProgram(++program: GLuint);
        fn glDeleteShader(++shader: GLuint);
        fn glDetachShader(++program: GLuint, ++shader: GLuint);
        fn glDisableVertexAttribArray(++index: GLuint);
        fn glEnableVertexAttribArray(++index: GLuint);
        fn glGetActiveAttrib(++program: GLuint, ++index: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++size: *GLint, ++gltype: *GLenum, ++name: *GLchar);
        fn glGetActiveUniform(++program: GLuint, ++index: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++size: *GLint, ++gltype: *GLenum, ++name: *GLchar);
        fn glGetAttachedShaders(++program: GLuint, ++maxCount: GLsizei, ++count: *GLsizei, ++obj: *GLuint);
        fn glGetAttribLocation(++program: GLuint, ++name: *GLchar) -> GLint;
        fn glGetProgramiv(++program: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetProgramInfoLog(++program: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++infoLog: *GLchar);
        fn glGetShaderiv(++shader: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetShaderInfoLog(++shader: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++infoLog: *GLchar);
        fn glGetShaderSource(++shader: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++source: *GLchar);
        fn glGetUniformLocation(++program: GLuint, ++name: *GLchar) -> GLint;
        fn glGetUniformfv(++program: GLuint, ++location: GLint, ++params: *GLfloat);
        fn glGetUniformiv(++program: GLuint, ++location: GLint, ++params: *GLint);
        fn glGetVertexAttribdv(++index: GLuint, ++pname: GLenum, ++params: *GLdouble);
        fn glGetVertexAttribfv(++index: GLuint, ++pname: GLenum, ++params: *GLfloat);
        fn glGetVertexAttribiv(++index: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetVertexAttribPointerv(++index: GLuint, ++pname: GLenum, ++pointer: **GLvoid);
        fn glIsProgram(++program: GLuint) -> GLboolean;
        fn glIsShader(++shader: GLuint) -> GLboolean;
        fn glLinkProgram(++program: GLuint);
        fn glShaderSource(++shader: GLuint, ++count: GLsizei, ++string: **GLchar, ++length: *GLint);
        fn glUseProgram(++program: GLuint);
        fn glUniform1f(++location: GLint, ++v0: GLfloat);
        fn glUniform2f(++location: GLint, ++v0: GLfloat, ++v1: GLfloat);
        fn glUniform3f(++location: GLint, ++v0: GLfloat, ++v1: GLfloat, ++v2: GLfloat);
        fn glUniform4f(++location: GLint, ++v0: GLfloat, ++v1: GLfloat, ++v2: GLfloat, ++v3: GLfloat);
        fn glUniform1i(++location: GLint, ++v0: GLint);
        fn glUniform2i(++location: GLint, ++v0: GLint, ++v1: GLint);
        fn glUniform3i(++location: GLint, ++v0: GLint, ++v1: GLint, ++v2: GLint);
        fn glUniform4i(++location: GLint, ++v0: GLint, ++v1: GLint, ++v2: GLint, ++v3: GLint);
        fn glUniform1fv(++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        fn glUniform2fv(++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        fn glUniform3fv(++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        fn glUniform4fv(++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        fn glUniform1iv(++location: GLint, ++count: GLsizei, ++value: *GLint);
        fn glUniform2iv(++location: GLint, ++count: GLsizei, ++value: *GLint);
        fn glUniform3iv(++location: GLint, ++count: GLsizei, ++value: *GLint);
        fn glUniform4iv(++location: GLint, ++count: GLsizei, ++value: *GLint);
        fn glUniformMatrix2fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix3fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix4fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glValidateProgram(++program: GLuint);
        fn glVertexAttrib1d(++index: GLuint, ++x: GLdouble);
        fn glVertexAttrib1dv(++index: GLuint, ++v: *GLdouble);
        fn glVertexAttrib1f(++index: GLuint, ++x: GLfloat);
        fn glVertexAttrib1fv(++index: GLuint, ++v: *GLfloat);
        fn glVertexAttrib1s(++index: GLuint, ++x: GLshort);
        fn glVertexAttrib1sv(++index: GLuint, ++v: *GLshort);
        fn glVertexAttrib2d(++index: GLuint, ++x: GLdouble, ++y: GLdouble);
        fn glVertexAttrib2dv(++index: GLuint, ++v: *GLdouble);
        fn glVertexAttrib2f(++index: GLuint, ++x: GLfloat, ++y: GLfloat);
        fn glVertexAttrib2fv(++index: GLuint, ++v: *GLfloat);
        fn glVertexAttrib2s(++index: GLuint, ++x: GLshort, ++y: GLshort);
        fn glVertexAttrib2sv(++index: GLuint, ++v: *GLshort);
        fn glVertexAttrib3d(++index: GLuint, ++x: GLdouble, ++y: GLdouble, ++z: GLdouble);
        fn glVertexAttrib3dv(++index: GLuint, ++v: *GLdouble);
        fn glVertexAttrib3f(++index: GLuint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat);
        fn glVertexAttrib3fv(++index: GLuint, ++v: *GLfloat);
        fn glVertexAttrib3s(++index: GLuint, ++x: GLshort, ++y: GLshort, ++z: GLshort);
        fn glVertexAttrib3sv(++index: GLuint, ++v: *GLshort);
        fn glVertexAttrib4Nbv(++index: GLuint, ++v: *GLbyte);
        fn glVertexAttrib4Niv(++index: GLuint, ++v: *GLint);
        fn glVertexAttrib4Nsv(++index: GLuint, ++v: *GLshort);
        fn glVertexAttrib4Nub(++index: GLuint, ++x: GLubyte, ++y: GLubyte, ++z: GLubyte, ++w: GLubyte);
        fn glVertexAttrib4Nubv(++index: GLuint, ++v: *GLubyte);
        fn glVertexAttrib4Nuiv(++index: GLuint, ++v: *GLuint);
        fn glVertexAttrib4Nusv(++index: GLuint, ++v: *GLushort);
        fn glVertexAttrib4bv(++index: GLuint, ++v: *GLbyte);
        fn glVertexAttrib4d(++index: GLuint, ++x: GLdouble, ++y: GLdouble, ++z: GLdouble, ++w: GLdouble);
        fn glVertexAttrib4dv(++index: GLuint, ++v: *GLdouble);
        fn glVertexAttrib4f(++index: GLuint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat, ++w: GLfloat);
        fn glVertexAttrib4fv(++index: GLuint, ++v: *GLfloat);
        fn glVertexAttrib4iv(++index: GLuint, ++v: *GLint);
        fn glVertexAttrib4s(++index: GLuint, ++x: GLshort, ++y: GLshort, ++z: GLshort, ++w: GLshort);
        fn glVertexAttrib4sv(++index: GLuint, ++v: *GLshort);
        fn glVertexAttrib4ubv(++index: GLuint, ++v: *GLubyte);
        fn glVertexAttrib4uiv(++index: GLuint, ++v: *GLuint);
        fn glVertexAttrib4usv(++index: GLuint, ++v: *GLushort);
        fn glVertexAttribPointer(++index: GLuint, ++size: GLint, ++gltype: GLenum, ++normalized: GLboolean, ++stride: GLsizei, ++pointer: *GLvoid);
    }
    type PFNGLBLENDEQUATIONSEPARATEPROC = *u8;
    type PFNGLDRAWBUFFERSPROC = *u8;
    type PFNGLSTENCILOPSEPARATEPROC = *u8;
    type PFNGLSTENCILFUNCSEPARATEPROC = *u8;
    type PFNGLSTENCILMASKSEPARATEPROC = *u8;
    type PFNGLATTACHSHADERPROC = *u8;
    type PFNGLBINDATTRIBLOCATIONPROC = *u8;
    type PFNGLCOMPILESHADERPROC = *u8;
    type PFNGLCREATEPROGRAMPROC = *u8;
    type PFNGLCREATESHADERPROC = *u8;
    type PFNGLDELETEPROGRAMPROC = *u8;
    type PFNGLDELETESHADERPROC = *u8;
    type PFNGLDETACHSHADERPROC = *u8;
    type PFNGLDISABLEVERTEXATTRIBARRAYPROC = *u8;
    type PFNGLENABLEVERTEXATTRIBARRAYPROC = *u8;
    type PFNGLGETACTIVEATTRIBPROC = *u8;
    type PFNGLGETACTIVEUNIFORMPROC = *u8;
    type PFNGLGETATTACHEDSHADERSPROC = *u8;
    type PFNGLGETATTRIBLOCATIONPROC = *u8;
    type PFNGLGETPROGRAMIVPROC = *u8;
    type PFNGLGETPROGRAMINFOLOGPROC = *u8;
    type PFNGLGETSHADERIVPROC = *u8;
    type PFNGLGETSHADERINFOLOGPROC = *u8;
    type PFNGLGETSHADERSOURCEPROC = *u8;
    type PFNGLGETUNIFORMLOCATIONPROC = *u8;
    type PFNGLGETUNIFORMFVPROC = *u8;
    type PFNGLGETUNIFORMIVPROC = *u8;
    type PFNGLGETVERTEXATTRIBDVPROC = *u8;
    type PFNGLGETVERTEXATTRIBFVPROC = *u8;
    type PFNGLGETVERTEXATTRIBIVPROC = *u8;
    type PFNGLGETVERTEXATTRIBPOINTERVPROC = *u8;
    type PFNGLISPROGRAMPROC = *u8;
    type PFNGLISSHADERPROC = *u8;
    type PFNGLLINKPROGRAMPROC = *u8;
    type PFNGLSHADERSOURCEPROC = *u8;
    type PFNGLUSEPROGRAMPROC = *u8;
    type PFNGLUNIFORM1FPROC = *u8;
    type PFNGLUNIFORM2FPROC = *u8;
    type PFNGLUNIFORM3FPROC = *u8;
    type PFNGLUNIFORM4FPROC = *u8;
    type PFNGLUNIFORM1IPROC = *u8;
    type PFNGLUNIFORM2IPROC = *u8;
    type PFNGLUNIFORM3IPROC = *u8;
    type PFNGLUNIFORM4IPROC = *u8;
    type PFNGLUNIFORM1FVPROC = *u8;
    type PFNGLUNIFORM2FVPROC = *u8;
    type PFNGLUNIFORM3FVPROC = *u8;
    type PFNGLUNIFORM4FVPROC = *u8;
    type PFNGLUNIFORM1IVPROC = *u8;
    type PFNGLUNIFORM2IVPROC = *u8;
    type PFNGLUNIFORM3IVPROC = *u8;
    type PFNGLUNIFORM4IVPROC = *u8;
    type PFNGLUNIFORMMATRIX2FVPROC = *u8;
    type PFNGLUNIFORMMATRIX3FVPROC = *u8;
    type PFNGLUNIFORMMATRIX4FVPROC = *u8;
    type PFNGLVALIDATEPROGRAMPROC = *u8;
    type PFNGLVERTEXATTRIB1DPROC = *u8;
    type PFNGLVERTEXATTRIB1DVPROC = *u8;
    type PFNGLVERTEXATTRIB1FPROC = *u8;
    type PFNGLVERTEXATTRIB1FVPROC = *u8;
    type PFNGLVERTEXATTRIB1SPROC = *u8;
    type PFNGLVERTEXATTRIB1SVPROC = *u8;
    type PFNGLVERTEXATTRIB2DPROC = *u8;
    type PFNGLVERTEXATTRIB2DVPROC = *u8;
    type PFNGLVERTEXATTRIB2FPROC = *u8;
    type PFNGLVERTEXATTRIB2FVPROC = *u8;
    type PFNGLVERTEXATTRIB2SPROC = *u8;
    type PFNGLVERTEXATTRIB2SVPROC = *u8;
    type PFNGLVERTEXATTRIB3DPROC = *u8;
    type PFNGLVERTEXATTRIB3DVPROC = *u8;
    type PFNGLVERTEXATTRIB3FPROC = *u8;
    type PFNGLVERTEXATTRIB3FVPROC = *u8;
    type PFNGLVERTEXATTRIB3SPROC = *u8;
    type PFNGLVERTEXATTRIB3SVPROC = *u8;
    type PFNGLVERTEXATTRIB4NBVPROC = *u8;
    type PFNGLVERTEXATTRIB4NIVPROC = *u8;
    type PFNGLVERTEXATTRIB4NSVPROC = *u8;
    type PFNGLVERTEXATTRIB4NUBPROC = *u8;
    type PFNGLVERTEXATTRIB4NUBVPROC = *u8;
    type PFNGLVERTEXATTRIB4NUIVPROC = *u8;
    type PFNGLVERTEXATTRIB4NUSVPROC = *u8;
    type PFNGLVERTEXATTRIB4BVPROC = *u8;
    type PFNGLVERTEXATTRIB4DPROC = *u8;
    type PFNGLVERTEXATTRIB4DVPROC = *u8;
    type PFNGLVERTEXATTRIB4FPROC = *u8;
    type PFNGLVERTEXATTRIB4FVPROC = *u8;
    type PFNGLVERTEXATTRIB4IVPROC = *u8;
    type PFNGLVERTEXATTRIB4SPROC = *u8;
    type PFNGLVERTEXATTRIB4SVPROC = *u8;
    type PFNGLVERTEXATTRIB4UBVPROC = *u8;
    type PFNGLVERTEXATTRIB4UIVPROC = *u8;
    type PFNGLVERTEXATTRIB4USVPROC = *u8;
    type PFNGLVERTEXATTRIBPOINTERPROC = *u8;
}

pub mod version_2_1 {
    #[nolink]
    extern {
        fn glUniformMatrix2x3fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix3x2fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix2x4fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix4x2fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix3x4fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        fn glUniformMatrix4x3fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
    }
    type PFNGLUNIFORMMATRIX2X3FVPROC = *u8;
    type PFNGLUNIFORMMATRIX3X2FVPROC = *u8;
    type PFNGLUNIFORMMATRIX2X4FVPROC = *u8;
    type PFNGLUNIFORMMATRIX4X2FVPROC = *u8;
    type PFNGLUNIFORMMATRIX3X4FVPROC = *u8;
    type PFNGLUNIFORMMATRIX4X3FVPROC = *u8;
}

pub mod version_3_0 {
    /* OpenGL 3.0 also reuses entry points from these extensions: */
    /* ARB_framebuffer_object */
    /* ARB_map_buffer_range */
    /* ARB_vertex_array_object */
    #[nolink]
    extern {
        fn glColorMaski(++index: GLuint, ++r: GLboolean, ++g: GLboolean, ++b: GLboolean, ++a: GLboolean);
        fn glGetBooleani_v(++target: GLenum, ++index: GLuint, ++data: *GLboolean);
        fn glGetIntegeri_v(++target: GLenum, ++index: GLuint, ++data: *GLint);
        fn glEnablei(++target: GLenum, ++index: GLuint);
        fn glDisablei(++target: GLenum, ++index: GLuint);
        fn glIsEnabledi(++target: GLenum, ++index: GLuint) -> GLboolean;
        fn glBeginTransformFeedback(++primitiveMode: GLenum);
        fn glEndTransformFeedback();
        fn glBindBufferRange(++target: GLenum, ++index: GLuint, ++buffer: GLuint, ++offset: GLintptr, ++size: GLsizeiptr);
        fn glBindBufferBase(++target: GLenum, ++index: GLuint, ++buffer: GLuint);
        fn glTransformFeedbackVaryings(++program: GLuint, ++count: GLsizei, ++varyings: **GLchar, ++bufferMode: GLenum);
        fn glGetTransformFeedbackVarying(++program: GLuint, ++index: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++size: *GLsizei, ++gltype: *GLenum, ++name: *GLchar);
        fn glClampColor(++target: GLenum, ++clamp: GLenum);
        fn glBeginConditionalRender(++id: GLuint, ++mode: GLenum);
        fn glEndConditionalRender();
        fn glVertexAttribIPointer(++index: GLuint, ++size: GLint, ++gltype: GLenum, ++stride: GLsizei, ++pointer: *GLvoid);
        fn glGetVertexAttribIiv(++index: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetVertexAttribIuiv(++index: GLuint, ++pname: GLenum, ++params: *GLuint);
        fn glVertexAttribI1i(++index: GLuint, ++x: GLint);
        fn glVertexAttribI2i(++index: GLuint, ++x: GLint, ++y: GLint);
        fn glVertexAttribI3i(++index: GLuint, ++x: GLint, ++y: GLint, ++z: GLint);
        fn glVertexAttribI4i(++index: GLuint, ++x: GLint, ++y: GLint, ++z: GLint, ++w: GLint);
        fn glVertexAttribI1ui(++index: GLuint, ++x: GLuint);
        fn glVertexAttribI2ui(++index: GLuint, ++x: GLuint, ++y: GLuint);
        fn glVertexAttribI3ui(++index: GLuint, ++x: GLuint, ++y: GLuint, ++z: GLuint);
        fn glVertexAttribI4ui(++index: GLuint, ++x: GLuint, ++y: GLuint, ++z: GLuint, ++w: GLuint);
        fn glVertexAttribI1iv(++index: GLuint, ++v: *GLint);
        fn glVertexAttribI2iv(++index: GLuint, ++v: *GLint);
        fn glVertexAttribI3iv(++index: GLuint, ++v: *GLint);
        fn glVertexAttribI4iv(++index: GLuint, ++v: *GLint);
        fn glVertexAttribI1uiv(++index: GLuint, ++v: *GLuint);
        fn glVertexAttribI2uiv(++index: GLuint, ++v: *GLuint);
        fn glVertexAttribI3uiv(++index: GLuint, ++v: *GLuint);
        fn glVertexAttribI4uiv(++index: GLuint, ++v: *GLuint);
        fn glVertexAttribI4bv(++index: GLuint, ++v: *GLbyte);
        fn glVertexAttribI4sv(++index: GLuint, ++v: *GLshort);
        fn glVertexAttribI4ubv(++index: GLuint, ++v: *GLubyte);
        fn glVertexAttribI4usv(++index: GLuint, ++v: *GLushort);
        fn glGetUniformuiv(++program: GLuint, ++location: GLint, ++params: *GLuint);
        fn glBindFragDataLocation(++program: GLuint, ++color: GLuint, ++name: *GLchar);
        fn glGetFragDataLocation(++program: GLuint, ++name: *GLchar) -> GLint;
        fn glUniform1ui(++location: GLint, ++v0: GLuint);
        fn glUniform2ui(++location: GLint, ++v0: GLuint, ++v1: GLuint);
        fn glUniform3ui(++location: GLint, ++v0: GLuint, ++v1: GLuint, ++v2: GLuint);
        fn glUniform4ui(++location: GLint, ++v0: GLuint, ++v1: GLuint, ++v2: GLuint, ++v3: GLuint);
        fn glUniform1uiv(++location: GLint, ++count: GLsizei, ++value: *GLuint);
        fn glUniform2uiv(++location: GLint, ++count: GLsizei, ++value: *GLuint);
        fn glUniform3uiv(++location: GLint, ++count: GLsizei, ++value: *GLuint);
        fn glUniform4uiv(++location: GLint, ++count: GLsizei, ++value: *GLuint);
        fn glTexParameterIiv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glTexParameterIuiv(++target: GLenum, ++pname: GLenum, ++params: *GLuint);
        fn glGetTexParameterIiv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glGetTexParameterIuiv(++target: GLenum, ++pname: GLenum, ++params: *GLuint);
        fn glClearBufferiv(++buffer: GLenum, ++drawbuffer: GLint, ++value: *GLint);
        fn glClearBufferuiv(++buffer: GLenum, ++drawbuffer: GLint, ++value: *GLuint);
        fn glClearBufferfv(++buffer: GLenum, ++drawbuffer: GLint, ++value: *GLfloat);
        fn glClearBufferfi(++buffer: GLenum, ++drawbuffer: GLint, ++depth: GLfloat, ++stencil: GLint);
        fn glGetStringi(++name: GLenum, ++index: GLuint) -> *GLubyte;
    }
    type PFNGLCOLORMASKIPROC = *u8;
    type PFNGLGETBOOLEANI_VPROC = *u8;
    type PFNGLGETINTEGERI_VPROC = *u8;
    type PFNGLENABLEIPROC = *u8;
    type PFNGLDISABLEIPROC = *u8;
    type PFNGLISENABLEDIPROC = *u8;
    type PFNGLBEGINTRANSFORMFEEDBACKPROC = *u8;
    type PFNGLENDTRANSFORMFEEDBACKPROC = *u8;
    type PFNGLBINDBUFFERRANGEPROC = *u8;
    type PFNGLBINDBUFFERBASEPROC = *u8;
    type PFNGLTRANSFORMFEEDBACKVARYINGSPROC = *u8;
    type PFNGLGETTRANSFORMFEEDBACKVARYINGPROC = *u8;
    type PFNGLCLAMPCOLORPROC = *u8;
    type PFNGLBEGINCONDITIONALRENDERPROC = *u8;
    type PFNGLENDCONDITIONALRENDERPROC = *u8;
    type PFNGLVERTEXATTRIBIPOINTERPROC = *u8;
    type PFNGLGETVERTEXATTRIBIIVPROC = *u8;
    type PFNGLGETVERTEXATTRIBIUIVPROC = *u8;
    type PFNGLVERTEXATTRIBI1IPROC = *u8;
    type PFNGLVERTEXATTRIBI2IPROC = *u8;
    type PFNGLVERTEXATTRIBI3IPROC = *u8;
    type PFNGLVERTEXATTRIBI4IPROC = *u8;
    type PFNGLVERTEXATTRIBI1UIPROC = *u8;
    type PFNGLVERTEXATTRIBI2UIPROC = *u8;
    type PFNGLVERTEXATTRIBI3UIPROC = *u8;
    type PFNGLVERTEXATTRIBI4UIPROC = *u8;
    type PFNGLVERTEXATTRIBI1IVPROC = *u8;
    type PFNGLVERTEXATTRIBI2IVPROC = *u8;
    type PFNGLVERTEXATTRIBI3IVPROC = *u8;
    type PFNGLVERTEXATTRIBI4IVPROC = *u8;
    type PFNGLVERTEXATTRIBI1UIVPROC = *u8;
    type PFNGLVERTEXATTRIBI2UIVPROC = *u8;
    type PFNGLVERTEXATTRIBI3UIVPROC = *u8;
    type PFNGLVERTEXATTRIBI4UIVPROC = *u8;
    type PFNGLVERTEXATTRIBI4BVPROC = *u8;
    type PFNGLVERTEXATTRIBI4SVPROC = *u8;
    type PFNGLVERTEXATTRIBI4UBVPROC = *u8;
    type PFNGLVERTEXATTRIBI4USVPROC = *u8;
    type PFNGLGETUNIFORMUIVPROC = *u8;
    type PFNGLBINDFRAGDATALOCATIONPROC = *u8;
    type PFNGLGETFRAGDATALOCATIONPROC = *u8;
    type PFNGLUNIFORM1UIPROC = *u8;
    type PFNGLUNIFORM2UIPROC = *u8;
    type PFNGLUNIFORM3UIPROC = *u8;
    type PFNGLUNIFORM4UIPROC = *u8;
    type PFNGLUNIFORM1UIVPROC = *u8;
    type PFNGLUNIFORM2UIVPROC = *u8;
    type PFNGLUNIFORM3UIVPROC = *u8;
    type PFNGLUNIFORM4UIVPROC = *u8;
    type PFNGLTEXPARAMETERIIVPROC = *u8;
    type PFNGLTEXPARAMETERIUIVPROC = *u8;
    type PFNGLGETTEXPARAMETERIIVPROC = *u8;
    type PFNGLGETTEXPARAMETERIUIVPROC = *u8;
    type PFNGLCLEARBUFFERIVPROC = *u8;
    type PFNGLCLEARBUFFERUIVPROC = *u8;
    type PFNGLCLEARBUFFERFVPROC = *u8;
    type PFNGLCLEARBUFFERFIPROC = *u8;
    type PFNGLGETSTRINGIPROC = *u8;
}

pub mod version_3_1 {
    /* OpenGL 3.1 also reuses entry points from these extensions: */
    /* ARB_copy_buffer */
    /* ARB_uniform_buffer_object */
    #[nolink]
    extern {
        fn glDrawArraysInstanced(++mode: GLenum, ++first: GLint, ++count: GLsizei, ++instancecount: GLsizei);
        fn glDrawElementsInstanced(++mode: GLenum, ++count: GLsizei, ++gltype: GLenum, ++indices: *GLvoid, ++instancecount: GLsizei);
        fn glTexBuffer(++target: GLenum, ++internalformat: GLenum, ++buffer: GLuint);
        fn glPrimitiveRestartIndex(++index: GLuint);
    }
    type PFNGLDRAWARRAYSINSTANCEDPROC = *u8;
    type PFNGLDRAWELEMENTSINSTANCEDPROC = *u8;
    type PFNGLTEXBUFFERPROC = *u8;
    type PFNGLPRIMITIVERESTARTINDEXPROC = *u8;
}

pub mod version_3_2 {
    /* OpenGL 3.2 also reuses entry points from these extensions: */
    /* ARB_draw_elements_base_vertex */
    /* ARB_provoking_vertex */
    /* ARB_sync */
    /* ARB_texture_multisample */
    #[nolink]
    extern {
        fn glGetInteger64i_v(++target: GLenum, ++index: GLuint, ++data: *GLint64);
        fn glGetBufferParameteri64v(++target: GLenum, ++pname: GLenum, ++params: *GLint64);
        fn glFramebufferTexture(++target: GLenum, ++attachment: GLenum, ++texture: GLuint, ++level: GLint);
    }
    type PFNGLGETINTEGER64I_VPROC = *u8;
    type PFNGLGETBUFFERPARAMETERI64VPROC = *u8;
    type PFNGLFRAMEBUFFERTEXTUREPROC = *u8;
}

pub mod version_3_3 {
    /* OpenGL 3.3 also reuses entry points from these extensions: */
    /* ARB_blend_func_extended */
    /* ARB_sampler_objects */
    /* ARB_explicit_attrib_location, but it has none */
    /* ARB_occlusion_query2 (no entry points) */
    /* ARB_shader_bit_encoding (no entry points) */
    /* ARB_texture_rgb10_a2ui (no entry points) */
    /* ARB_texture_swizzle (no entry points) */
    /* ARB_timer_query */
    /* ARB_vertex_type_2_10_10_10_rev */
    #[nolink]
    extern {
        // fn glVertexAttribDivisor(++index: GLuint, ++divisor: GLuint);
    }
    type PFNGLVERTEXATTRIBDIVISORPROC = *u8;
}

pub mod version_4_0 {
    /* OpenGL 4.0 also reuses entry points from these extensions: */
    /* ARB_texture_query_lod (no entry points) */
    /* ARB_draw_indirect */
    /* ARB_gpu_shader5 (no entry points) */
    /* ARB_gpu_shader_fp64 */
    /* ARB_shader_subroutine */
    /* ARB_tessellation_shader */
    /* ARB_texture_buffer_object_rgb32 (no entry points) */
    /* ARB_texture_cube_map_array (no entry points) */
    /* ARB_texture_gather (no entry points) */
    /* ARB_transform_feedback2 */
    /* ARB_transform_feedback3 */
    #[nolink]
    extern {
        // fn glMinSampleShading(++value: GLfloat);
        // fn glBlendEquationi(++buf: GLuint, ++mode: GLenum);
        // fn glBlendEquationSeparatei(++buf: GLuint, ++modeRGB: GLenum, ++modeAlpha: GLenum);
        // fn glBlendFunci(++buf: GLuint, ++src: GLenum, ++dst: GLenum);
        // fn glBlendFuncSeparatei(++buf: GLuint, ++srcRGB: GLenum, ++dstRGB: GLenum, ++srcAlpha: GLenum, ++dstAlpha: GLenum);
    }
    type PFNGLMINSAMPLESHADINGPROC = *u8;
    type PFNGLBLENDEQUATIONIPROC = *u8;
    type PFNGLBLENDEQUATIONSEPARATEIPROC = *u8;
    type PFNGLBLENDFUNCIPROC = *u8;
    type PFNGLBLENDFUNCSEPARATEIPROC = *u8;
}

pub mod version_4_1 {
    /* OpenGL 4.1 reuses entry points from these extensions: */
    /* ARB_ES2_compatibility */
    /* ARB_get_program_binary */
    /* ARB_separate_shader_objects */
    /* ARB_shader_precision (no entry points) */
    /* ARB_vertex_attrib_64bit */
    /* ARB_viewport_array */
}

pub mod version_4_2 {
    /* OpenGL 4.2 reuses entry points from these extensions: */
    /* ARB_base_instance */
    /* ARB_shading_language_420pack (no entry points) */
    /* ARB_transform_feedback_instanced */
    /* ARB_compressed_texture_pixel_storage (no entry points) */
    /* ARB_conservative_depth (no entry points) */
    /* ARB_internalformat_query */
    /* ARB_map_buffer_alignment (no entry points) */
    /* ARB_shader_atomic_counters */
    /* ARB_shader_image_load_store */
    /* ARB_shading_language_packing (no entry points) */
    /* ARB_texture_storage */
}

pub mod version_4_3 {
    /* OpenGL 4.3 reuses entry points from these extensions: */
    /* ARB_arrays_of_arrays (no entry points, GLSL only) */
    /* ARB_fragment_layer_viewport (no entry points, GLSL only) */
    /* ARB_shader_image_size (no entry points, GLSL only) */
    /* ARB_ES3_compatibility (no entry points) */
    /* ARB_clear_buffer_object */
    /* ARB_compute_shader */
    /* ARB_copy_image */
    /* KHR_debug (includes ARB_debug_output commands promoted to KHR without suffixes) */
    /* ARB_explicit_uniform_location (no entry points) */
    /* ARB_framebuffer_no_attachments */
    /* ARB_internalformat_query2 */
    /* ARB_invalidate_subdata */
    /* ARB_multi_draw_indirect */
    /* ARB_program_interface_query */
    /* ARB_robust_buffer_access_behavior (no entry points) */
    /* ARB_shader_storage_buffer_object */
    /* ARB_stencil_texturing (no entry points) */
    /* ARB_texture_buffer_range */
    /* ARB_texture_query_levels (no entry points) */
    /* ARB_texture_storage_multisample */
    /* ARB_texture_view */
    /* ARB_vertex_attrib_binding */
}

pub mod arb_depth_buffer_float {
}

pub mod arb_framebuffer_object {
    #[nolink]
    extern {
        fn glIsRenderbuffer(++renderbuffer: GLuint) -> GLboolean;
        fn glBindRenderbuffer(++target: GLenum, ++renderbuffer: GLuint);
        fn glDeleteRenderbuffers(++n: GLsizei, ++renderbuffers: *GLuint);
        fn glGenRenderbuffers(++n: GLsizei, ++renderbuffers: *GLuint);
        fn glRenderbufferStorage(++target: GLenum, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei);
        fn glGetRenderbufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glIsFramebuffer(++framebuffer: GLuint) -> GLboolean;
        fn glBindFramebuffer(++target: GLenum, ++framebuffer: GLuint);
        fn glDeleteFramebuffers(++n: GLsizei, ++framebuffers: *GLuint);
        fn glGenFramebuffers(++n: GLsizei, ++framebuffers: *GLuint);
        fn glCheckFramebufferStatus(++target: GLenum) -> GLenum;
        fn glFramebufferTexture1D(++target: GLenum, ++attachment: GLenum, ++textarget: GLenum, ++texture: GLuint, ++level: GLint);
        fn glFramebufferTexture2D(++target: GLenum, ++attachment: GLenum, ++textarget: GLenum, ++texture: GLuint, ++level: GLint);
        fn glFramebufferTexture3D(++target: GLenum, ++attachment: GLenum, ++textarget: GLenum, ++texture: GLuint, ++level: GLint, ++zoffset: GLint);
        fn glFramebufferRenderbuffer(++target: GLenum, ++attachment: GLenum, ++renderbuffertarget: GLenum, ++renderbuffer: GLuint);
        fn glGetFramebufferAttachmentParameteriv(++target: GLenum, ++attachment: GLenum, ++pname: GLenum, ++params: *GLint);
        fn glGenerateMipmap(++target: GLenum);
        fn glBlitFramebuffer(++srcX0: GLint, ++srcY0: GLint, ++srcX1: GLint, ++srcY1: GLint, ++dstX0: GLint, ++dstY0: GLint, ++dstX1: GLint, ++dstY1: GLint, ++mask: GLbitfield, ++filter: GLenum);
        fn glRenderbufferStorageMultisample(++target: GLenum, ++samples: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei);
        fn glFramebufferTextureLayer(++target: GLenum, ++attachment: GLenum, ++texture: GLuint, ++level: GLint, ++layer: GLint);
    }
    type PFNGLISRENDERBUFFERPROC = *u8;
    type PFNGLBINDRENDERBUFFERPROC = *u8;
    type PFNGLDELETERENDERBUFFERSPROC = *u8;
    type PFNGLGENRENDERBUFFERSPROC = *u8;
    type PFNGLRENDERBUFFERSTORAGEPROC = *u8;
    type PFNGLGETRENDERBUFFERPARAMETERIVPROC = *u8;
    type PFNGLISFRAMEBUFFERPROC = *u8;
    type PFNGLBINDFRAMEBUFFERPROC = *u8;
    type PFNGLDELETEFRAMEBUFFERSPROC = *u8;
    type PFNGLGENFRAMEBUFFERSPROC = *u8;
    type PFNGLCHECKFRAMEBUFFERSTATUSPROC = *u8;
    type PFNGLFRAMEBUFFERTEXTURE1DPROC = *u8;
    type PFNGLFRAMEBUFFERTEXTURE2DPROC = *u8;
    type PFNGLFRAMEBUFFERTEXTURE3DPROC = *u8;
    type PFNGLFRAMEBUFFERRENDERBUFFERPROC = *u8;
    type PFNGLGETFRAMEBUFFERATTACHMENTPARAMETERIVPROC = *u8;
    type PFNGLGENERATEMIPMAPPROC = *u8;
    type PFNGLBLITFRAMEBUFFERPROC = *u8;
    type PFNGLRENDERBUFFERSTORAGEMULTISAMPLEPROC = *u8;
    type PFNGLFRAMEBUFFERTEXTURELAYERPROC = *u8;
}

pub mod arb_framebuffer_srgb {
}

pub mod arb_half_float_vertex {
}

pub mod arb_map_buffer_range {
    #[nolink]
    extern {
        fn glMapBufferRange(++target: GLenum, ++offset: GLintptr, ++length: GLsizeiptr, ++access: GLbitfield) -> *GLvoid;
        fn glFlushMappedBufferRange(++target: GLenum, ++offset: GLintptr, ++length: GLsizeiptr);
    }
    type PFNGLMAPBUFFERRANGEPROC = *u8;
    type PFNGLFLUSHMAPPEDBUFFERRANGEPROC = *u8;
}

pub mod arb_texture_compression_rgtc {
}

pub mod arb_texture_rg {
}

pub mod arb_vertex_array_object {
    #[nolink]
    extern {
        fn glBindVertexArray(++array: GLuint);
        fn glDeleteVertexArrays(++n: GLsizei, ++arrays: *GLuint);
        fn glGenVertexArrays(++n: GLsizei, ++arrays: *GLuint);
        fn glIsVertexArray(++array: GLuint) -> GLboolean;
    }
    type PFNGLBINDVERTEXARRAYPROC = *u8;
    type PFNGLDELETEVERTEXARRAYSPROC = *u8;
    type PFNGLGENVERTEXARRAYSPROC = *u8;
    type PFNGLISVERTEXARRAYPROC = *u8;
}

pub mod arb_uniform_buffer_object {
    #[nolink]
    extern {
        fn glGetUniformIndices(++program: GLuint, ++uniformCount: GLsizei, ++uniformNames: **GLchar, ++uniformIndices: *GLuint);
        fn glGetActiveUniformsiv(++program: GLuint, ++uniformCount: GLsizei, ++uniformIndices: *GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetActiveUniformName(++program: GLuint, ++uniformIndex: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++uniformName: *GLchar);
        fn glGetUniformBlockIndex(++program: GLuint, ++uniformBlockName: *GLchar) -> GLuint;
        fn glGetActiveUniformBlockiv(++program: GLuint, ++uniformBlockIndex: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetActiveUniformBlockName(++program: GLuint, ++uniformBlockIndex: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++uniformBlockName: *GLchar);
        fn glUniformBlockBinding(++program: GLuint, ++uniformBlockIndex: GLuint, ++uniformBlockBinding: GLuint);
    }
    type PFNGLGETUNIFORMINDICESPROC = *u8;
    type PFNGLGETACTIVEUNIFORMSIVPROC = *u8;
    type PFNGLGETACTIVEUNIFORMNAMEPROC = *u8;
    type PFNGLGETUNIFORMBLOCKINDEXPROC = *u8;
    type PFNGLGETACTIVEUNIFORMBLOCKIVPROC = *u8;
    type PFNGLGETACTIVEUNIFORMBLOCKNAMEPROC = *u8;
    type PFNGLUNIFORMBLOCKBINDINGPROC = *u8;
}

pub mod arb_copy_buffer {
    #[nolink]
    extern {
        fn glCopyBufferSubData(++readTarget: GLenum, ++writeTarget: GLenum, ++readOffset: GLintptr, ++writeOffset: GLintptr, ++size: GLsizeiptr);
    }
    type PFNGLCOPYBUFFERSUBDATAPROC = *u8;
}

pub mod arb_depth_clamp {
}

pub mod arb_draw_elements_base_vertex {
    #[nolink]
    extern {
        fn glDrawElementsBaseVertex(++mode: GLenum, ++count: GLsizei, ++gltype: GLenum, ++indices: *GLvoid, ++basevertex: GLint);
        fn glDrawRangeElementsBaseVertex(++mode: GLenum, ++start: GLuint, ++end: GLuint, ++count: GLsizei, ++gltype: GLenum, ++indices: *GLvoid, ++basevertex: GLint);
        fn glDrawElementsInstancedBaseVertex(++mode: GLenum, ++count: GLsizei, ++gltype: GLenum, ++indices: *GLvoid, ++instancecount: GLsizei, ++basevertex: GLint);
        fn glMultiDrawElementsBaseVertex(++mode: GLenum, ++count: *GLsizei, ++gltype: GLenum, ++indices: **GLvoid, ++drawcount: GLsizei, ++basevertex: *GLint);
    }
    type PFNGLDRAWELEMENTSBASEVERTEXPROC = *u8;
    type PFNGLDRAWRANGEELEMENTSBASEVERTEXPROC = *u8;
    type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXPROC = *u8;
    type PFNGLMULTIDRAWELEMENTSBASEVERTEXPROC = *u8;
}

pub mod arb_fragment_coord_conventions {
}

pub mod arb_provoking_vertex {
    #[nolink]
    extern {
        fn glProvokingVertex(++mode: GLenum);
    }
    type PFNGLPROVOKINGVERTEXPROC = *u8;
}

pub mod arb_seamless_cube_map {
}

pub mod arb_sync {
    #[nolink]
    extern {
        fn glFenceSync(++condition: GLenum, ++flags: GLbitfield) -> GLsync;
        fn glIsSync(++sync: GLsync) -> GLboolean;
        fn glDeleteSync(++sync: GLsync);
        fn glClientWaitSync(++sync: GLsync, ++flags: GLbitfield, ++timeout: GLuint64) -> GLenum;
        fn glWaitSync(++sync: GLsync, ++flags: GLbitfield, ++timeout: GLuint64);
        fn glGetInteger64v(++pname: GLenum, ++params: *GLint64);
        fn glGetSynciv(++sync: GLsync, ++pname: GLenum, ++bufSize: GLsizei, ++length: *GLsizei, ++values: *GLint);
    }
    type PFNGLFENCESYNCPROC = *u8;
    type PFNGLISSYNCPROC = *u8;
    type PFNGLDELETESYNCPROC = *u8;
    type PFNGLCLIENTWAITSYNCPROC = *u8;
    type PFNGLWAITSYNCPROC = *u8;
    type PFNGLGETINTEGER64VPROC = *u8;
    type PFNGLGETSYNCIVPROC = *u8;
}

pub mod arb_texture_multisample {
    #[nolink]
    extern {
        fn glTexImage2DMultisample(++target: GLenum, ++samples: GLsizei, ++internalformat: GLint, ++width: GLsizei, ++height: GLsizei, ++fixedsamplelocations: GLboolean);
        fn glTexImage3DMultisample(++target: GLenum, ++samples: GLsizei, ++internalformat: GLint, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++fixedsamplelocations: GLboolean);
        fn glGetMultisamplefv(++pname: GLenum, ++index: GLuint, ++val: *GLfloat);
        fn glSampleMaski(++index: GLuint, ++mask: GLbitfield);
    }
    type PFNGLTEXIMAGE2DMULTISAMPLEPROC = *u8;
    type PFNGLTEXIMAGE3DMULTISAMPLEPROC = *u8;
    type PFNGLGETMULTISAMPLEFVPROC = *u8;
    type PFNGLSAMPLEMASKIPROC = *u8;
}

pub mod arb_vertex_array_bgra {
}

pub mod arb_draw_buffers_blend {
    #[nolink]
    extern {
        // fn glBlendEquationiARB(++buf: GLuint, ++mode: GLenum);
        // fn glBlendEquationSeparateiARB(++buf: GLuint, ++modeRGB: GLenum, ++modeAlpha: GLenum);
        // fn glBlendFunciARB(++buf: GLuint, ++src: GLenum, ++dst: GLenum);
        // fn glBlendFuncSeparateiARB(++buf: GLuint, ++srcRGB: GLenum, ++dstRGB: GLenum, ++srcAlpha: GLenum, ++dstAlpha: GLenum);
    }
    type PFNGLBLENDEQUATIONIARBPROC = *u8;
    type PFNGLBLENDEQUATIONSEPARATEIARBPROC = *u8;
    type PFNGLBLENDFUNCIARBPROC = *u8;
    type PFNGLBLENDFUNCSEPARATEIARBPROC = *u8;
}

pub mod arb_sample_shading {
    #[nolink]
    extern {
        // fn glMinSampleShadingARB(++value: GLfloat);
    }
    type PFNGLMINSAMPLESHADINGARBPROC = *u8;
}

pub mod arb_texture_cube_map_array {
}

pub mod arb_texture_gather {
}

pub mod arb_texture_query_lod {
}

pub mod arb_shading_language_include {
    #[nolink]
    extern {
        // fn glNamedStringARB(++gltype: GLenum, ++namelen: GLint, ++name: *GLchar, ++stringlen: GLint, ++string: *GLchar);
        // fn glDeleteNamedStringARB(++namelen: GLint, ++name: *GLchar);
        // fn glCompileShaderIncludeARB(++shader: GLuint, ++count: GLsizei, ++path: **GLchar, ++length: *GLint);
        // fn glIsNamedStringARB(++namelen: GLint, ++name: *GLchar) -> GLboolean;
        // fn glGetNamedStringARB(++namelen: GLint, ++name: *GLchar, ++bufSize: GLsizei, ++stringlen: *GLint, ++string: *GLchar);
        // fn glGetNamedStringivARB(++namelen: GLint, ++name: *GLchar, ++pname: GLenum, ++params: *GLint);
    }
    type PFNGLNAMEDSTRINGARBPROC = *u8;
    type PFNGLDELETENAMEDSTRINGARBPROC = *u8;
    type PFNGLCOMPILESHADERINCLUDEARBPROC = *u8;
    type PFNGLISNAMEDSTRINGARBPROC = *u8;
    type PFNGLGETNAMEDSTRINGARBPROC = *u8;
    type PFNGLGETNAMEDSTRINGIVARBPROC = *u8;
}

pub mod arb_texture_compression_bptc {
}

pub mod arb_blend_func_extended {
    #[nolink]
    extern {
        // fn glBindFragDataLocationIndexed(++program: GLuint, ++colorNumber: GLuint, ++index: GLuint, ++name: *GLchar);
        // fn glGetFragDataIndex(++program: GLuint, ++name: *GLchar) -> GLint;
    }
    type PFNGLBINDFRAGDATALOCATIONINDEXEDPROC = *u8;
    type PFNGLGETFRAGDATAINDEXPROC = *u8;
}

pub mod arb_explicit_attrib_location {
}

pub mod arb_occlusion_query2 {
}

pub mod arb_sampler_objects {
    #[nolink]
    extern {
        fn glGenSamplers(++count: GLsizei, ++samplers: *GLuint);
        fn glDeleteSamplers(++count: GLsizei, ++samplers: *GLuint);
        fn glIsSampler(++sampler: GLuint) -> GLboolean;
        fn glBindSampler(++unit: GLuint, ++sampler: GLuint);
        fn glSamplerParameteri(++sampler: GLuint, ++pname: GLenum, ++param: GLint);
        fn glSamplerParameteriv(++sampler: GLuint, ++pname: GLenum, ++param: *GLint);
        fn glSamplerParameterf(++sampler: GLuint, ++pname: GLenum, ++param: GLfloat);
        fn glSamplerParameterfv(++sampler: GLuint, ++pname: GLenum, ++param: *GLfloat);
        fn glSamplerParameterIiv(++sampler: GLuint, ++pname: GLenum, ++param: *GLint);
        fn glSamplerParameterIuiv(++sampler: GLuint, ++pname: GLenum, ++param: *GLuint);
        fn glGetSamplerParameteriv(++sampler: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetSamplerParameterIiv(++sampler: GLuint, ++pname: GLenum, ++params: *GLint);
        fn glGetSamplerParameterfv(++sampler: GLuint, ++pname: GLenum, ++params: *GLfloat);
        fn glGetSamplerParameterIuiv(++sampler: GLuint, ++pname: GLenum, ++params: *GLuint);
    }
    type PFNGLGENSAMPLERSPROC = *u8;
    type PFNGLDELETESAMPLERSPROC = *u8;
    type PFNGLISSAMPLERPROC = *u8;
    type PFNGLBINDSAMPLERPROC = *u8;
    type PFNGLSAMPLERPARAMETERIPROC = *u8;
    type PFNGLSAMPLERPARAMETERIVPROC = *u8;
    type PFNGLSAMPLERPARAMETERFPROC = *u8;
    type PFNGLSAMPLERPARAMETERFVPROC = *u8;
    type PFNGLSAMPLERPARAMETERIIVPROC = *u8;
    type PFNGLSAMPLERPARAMETERIUIVPROC = *u8;
    type PFNGLGETSAMPLERPARAMETERIVPROC = *u8;
    type PFNGLGETSAMPLERPARAMETERIIVPROC = *u8;
    type PFNGLGETSAMPLERPARAMETERFVPROC = *u8;
    type PFNGLGETSAMPLERPARAMETERIUIVPROC = *u8;
}

pub mod arb_shader_bit_encoding {
}

pub mod arb_texture_rgb10_a2ui {
}

pub mod arb_texture_swizzle {
}

pub mod arb_timer_query {
    #[nolink]
    extern {
        fn glQueryCounter(++id: GLuint, ++target: GLenum);
        fn glGetQueryObjecti64v(++id: GLuint, ++pname: GLenum, ++params: *GLint64);
        fn glGetQueryObjectui64v(++id: GLuint, ++pname: GLenum, ++params: *GLuint64);
    }
    type PFNGLQUERYCOUNTERPROC = *u8;
    type PFNGLGETQUERYOBJECTI64VPROC = *u8;
    type PFNGLGETQUERYOBJECTUI64VPROC = *u8;
}

pub mod arb_vertex_type_2_10_10_10_rev {
    #[nolink]
    extern {
        // fn glVertexP2ui(++gltype: GLenum, ++value: GLuint);
        // fn glVertexP2uiv(++gltype: GLenum, ++value: *GLuint);
        // fn glVertexP3ui(++gltype: GLenum, ++value: GLuint);
        // fn glVertexP3uiv(++gltype: GLenum, ++value: *GLuint);
        // fn glVertexP4ui(++gltype: GLenum, ++value: GLuint);
        // fn glVertexP4uiv(++gltype: GLenum, ++value: *GLuint);
        // fn glTexCoordP1ui(++gltype: GLenum, ++coords: GLuint);
        // fn glTexCoordP1uiv(++gltype: GLenum, ++coords: *GLuint);
        // fn glTexCoordP2ui(++gltype: GLenum, ++coords: GLuint);
        // fn glTexCoordP2uiv(++gltype: GLenum, ++coords: *GLuint);
        // fn glTexCoordP3ui(++gltype: GLenum, ++coords: GLuint);
        // fn glTexCoordP3uiv(++gltype: GLenum, ++coords: *GLuint);
        // fn glTexCoordP4ui(++gltype: GLenum, ++coords: GLuint);
        // fn glTexCoordP4uiv(++gltype: GLenum, ++coords: *GLuint);
        // fn glMultiTexCoordP1ui(++texture: GLenum, ++gltype: GLenum, ++coords: GLuint);
        // fn glMultiTexCoordP1uiv(++texture: GLenum, ++gltype: GLenum, ++coords: *GLuint);
        // fn glMultiTexCoordP2ui(++texture: GLenum, ++gltype: GLenum, ++coords: GLuint);
        // fn glMultiTexCoordP2uiv(++texture: GLenum, ++gltype: GLenum, ++coords: *GLuint);
        // fn glMultiTexCoordP3ui(++texture: GLenum, ++gltype: GLenum, ++coords: GLuint);
        // fn glMultiTexCoordP3uiv(++texture: GLenum, ++gltype: GLenum, ++coords: *GLuint);
        // fn glMultiTexCoordP4ui(++texture: GLenum, ++gltype: GLenum, ++coords: GLuint);
        // fn glMultiTexCoordP4uiv(++texture: GLenum, ++gltype: GLenum, ++coords: *GLuint);
        // fn glNormalP3ui(++gltype: GLenum, ++coords: GLuint);
        // fn glNormalP3uiv(++gltype: GLenum, ++coords: *GLuint);
        // fn glColorP3ui(++gltype: GLenum, ++color: GLuint);
        // fn glColorP3uiv(++gltype: GLenum, ++color: *GLuint);
        // fn glColorP4ui(++gltype: GLenum, ++color: GLuint);
        // fn glColorP4uiv(++gltype: GLenum, ++color: *GLuint);
        // fn glSecondaryColorP3ui(++gltype: GLenum, ++color: GLuint);
        // fn glSecondaryColorP3uiv(++gltype: GLenum, ++color: *GLuint);
        // fn glVertexAttribP1ui(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: GLuint);
        // fn glVertexAttribP1uiv(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: *GLuint);
        // fn glVertexAttribP2ui(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: GLuint);
        // fn glVertexAttribP2uiv(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: *GLuint);
        // fn glVertexAttribP3ui(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: GLuint);
        // fn glVertexAttribP3uiv(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: *GLuint);
        // fn glVertexAttribP4ui(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: GLuint);
        // fn glVertexAttribP4uiv(++index: GLuint, ++gltype: GLenum, ++normalized: GLboolean, ++value: *GLuint);
    }
    type PFNGLVERTEXP2UIPROC = *u8;
    type PFNGLVERTEXP2UIVPROC = *u8;
    type PFNGLVERTEXP3UIPROC = *u8;
    type PFNGLVERTEXP3UIVPROC = *u8;
    type PFNGLVERTEXP4UIPROC = *u8;
    type PFNGLVERTEXP4UIVPROC = *u8;
    type PFNGLTEXCOORDP1UIPROC = *u8;
    type PFNGLTEXCOORDP1UIVPROC = *u8;
    type PFNGLTEXCOORDP2UIPROC = *u8;
    type PFNGLTEXCOORDP2UIVPROC = *u8;
    type PFNGLTEXCOORDP3UIPROC = *u8;
    type PFNGLTEXCOORDP3UIVPROC = *u8;
    type PFNGLTEXCOORDP4UIPROC = *u8;
    type PFNGLTEXCOORDP4UIVPROC = *u8;
    type PFNGLMULTITEXCOORDP1UIPROC = *u8;
    type PFNGLMULTITEXCOORDP1UIVPROC = *u8;
    type PFNGLMULTITEXCOORDP2UIPROC = *u8;
    type PFNGLMULTITEXCOORDP2UIVPROC = *u8;
    type PFNGLMULTITEXCOORDP3UIPROC = *u8;
    type PFNGLMULTITEXCOORDP3UIVPROC = *u8;
    type PFNGLMULTITEXCOORDP4UIPROC = *u8;
    type PFNGLMULTITEXCOORDP4UIVPROC = *u8;
    type PFNGLNORMALP3UIPROC = *u8;
    type PFNGLNORMALP3UIVPROC = *u8;
    type PFNGLCOLORP3UIPROC = *u8;
    type PFNGLCOLORP3UIVPROC = *u8;
    type PFNGLCOLORP4UIPROC = *u8;
    type PFNGLCOLORP4UIVPROC = *u8;
    type PFNGLSECONDARYCOLORP3UIPROC = *u8;
    type PFNGLSECONDARYCOLORP3UIVPROC = *u8;
    type PFNGLVERTEXATTRIBP1UIPROC = *u8;
    type PFNGLVERTEXATTRIBP1UIVPROC = *u8;
    type PFNGLVERTEXATTRIBP2UIPROC = *u8;
    type PFNGLVERTEXATTRIBP2UIVPROC = *u8;
    type PFNGLVERTEXATTRIBP3UIPROC = *u8;
    type PFNGLVERTEXATTRIBP3UIVPROC = *u8;
    type PFNGLVERTEXATTRIBP4UIPROC = *u8;
    type PFNGLVERTEXATTRIBP4UIVPROC = *u8;
}

pub mod arb_draw_indirect {
    #[nolink]
    extern {
        // fn glDrawArraysIndirect(++mode: GLenum, ++indirect: *GLvoid);
        // fn glDrawElementsIndirect(++mode: GLenum, ++gltype: GLenum, ++indirect: *GLvoid);
    }
    type PFNGLDRAWARRAYSINDIRECTPROC = *u8;
    type PFNGLDRAWELEMENTSINDIRECTPROC = *u8;
}

pub mod arb_gpu_shader5 {
}

pub mod arb_gpu_shader_fp64 {
    #[nolink]
    extern {
        // fn glUniform1d(++location: GLint, ++x: GLdouble);
        // fn glUniform2d(++location: GLint, ++x: GLdouble, ++y: GLdouble);
        // fn glUniform3d(++location: GLint, ++x: GLdouble, ++y: GLdouble, ++z: GLdouble);
        // fn glUniform4d(++location: GLint, ++x: GLdouble, ++y: GLdouble, ++z: GLdouble, ++w: GLdouble);
        // fn glUniform1dv(++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glUniform2dv(++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glUniform3dv(++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glUniform4dv(++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glUniformMatrix2dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix3dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix4dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix2x3dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix2x4dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix3x2dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix3x4dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix4x2dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glUniformMatrix4x3dv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glGetUniformdv(++program: GLuint, ++location: GLint, ++params: *GLdouble);
    }
    type PFNGLUNIFORM1DPROC = *u8;
    type PFNGLUNIFORM2DPROC = *u8;
    type PFNGLUNIFORM3DPROC = *u8;
    type PFNGLUNIFORM4DPROC = *u8;
    type PFNGLUNIFORM1DVPROC = *u8;
    type PFNGLUNIFORM2DVPROC = *u8;
    type PFNGLUNIFORM3DVPROC = *u8;
    type PFNGLUNIFORM4DVPROC = *u8;
    type PFNGLUNIFORMMATRIX2DVPROC = *u8;
    type PFNGLUNIFORMMATRIX3DVPROC = *u8;
    type PFNGLUNIFORMMATRIX4DVPROC = *u8;
    type PFNGLUNIFORMMATRIX2X3DVPROC = *u8;
    type PFNGLUNIFORMMATRIX2X4DVPROC = *u8;
    type PFNGLUNIFORMMATRIX3X2DVPROC = *u8;
    type PFNGLUNIFORMMATRIX3X4DVPROC = *u8;
    type PFNGLUNIFORMMATRIX4X2DVPROC = *u8;
    type PFNGLUNIFORMMATRIX4X3DVPROC = *u8;
    type PFNGLGETUNIFORMDVPROC = *u8;
}

pub mod arb_shader_subroutine {
    #[nolink]
    extern {
        // fn glGetSubroutineUniformLocation(++program: GLuint, ++shadertype: GLenum, ++name: *GLchar) -> GLint;
        // fn glGetSubroutineIndex(++program: GLuint, ++shadertype: GLenum, ++name: *GLchar) -> GLuint;
        // fn glGetActiveSubroutineUniformiv(++program: GLuint, ++shadertype: GLenum, ++index: GLuint, ++pname: GLenum, ++values: *GLint);
        // fn glGetActiveSubroutineUniformName(++program: GLuint, ++shadertype: GLenum, ++index: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++name: *GLchar);
        // fn glGetActiveSubroutineName(++program: GLuint, ++shadertype: GLenum, ++index: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++name: *GLchar);
        // fn glUniformSubroutinesuiv(++shadertype: GLenum, ++count: GLsizei, ++indices: *GLuint);
        // fn glGetUniformSubroutineuiv(++shadertype: GLenum, ++location: GLint, ++params: *GLuint);
        // fn glGetProgramStageiv(++program: GLuint, ++shadertype: GLenum, ++pname: GLenum, ++values: *GLint);
    }
    type PFNGLGETSUBROUTINEUNIFORMLOCATIONPROC = *u8;
    type PFNGLGETSUBROUTINEINDEXPROC = *u8;
    type PFNGLGETACTIVESUBROUTINEUNIFORMIVPROC = *u8;
    type PFNGLGETACTIVESUBROUTINEUNIFORMNAMEPROC = *u8;
    type PFNGLGETACTIVESUBROUTINENAMEPROC = *u8;
    type PFNGLUNIFORMSUBROUTINESUIVPROC = *u8;
    type PFNGLGETUNIFORMSUBROUTINEUIVPROC = *u8;
    type PFNGLGETPROGRAMSTAGEIVPROC = *u8;
}

pub mod arb_tessellation_shader {
    #[nolink]
    extern {
        // fn glPatchParameteri(++pname: GLenum, ++value: GLint);
        // fn glPatchParameterfv(++pname: GLenum, ++values: *GLfloat);
    }
    type PFNGLPATCHPARAMETERIPROC = *u8;
    type PFNGLPATCHPARAMETERFVPROC = *u8;
}

pub mod arb_texture_buffer_object_rgb32 {
}

pub mod arb_transform_feedback2 {
    #[nolink]
    extern {
        // fn glBindTransformFeedback(++target: GLenum, ++id: GLuint);
        // fn glDeleteTransformFeedbacks(++n: GLsizei, ++ids: *GLuint);
        // fn glGenTransformFeedbacks(++n: GLsizei, ++ids: *GLuint);
        // fn glIsTransformFeedback(++id: GLuint) -> GLboolean;
        // fn glPauseTransformFeedback();
        // fn glResumeTransformFeedback();
        // fn glDrawTransformFeedback(++mode: GLenum, ++id: GLuint);
    }
    type PFNGLBINDTRANSFORMFEEDBACKPROC = *u8;
    type PFNGLDELETETRANSFORMFEEDBACKSPROC = *u8;
    type PFNGLGENTRANSFORMFEEDBACKSPROC = *u8;
    type PFNGLISTRANSFORMFEEDBACKPROC = *u8;
    type PFNGLPAUSETRANSFORMFEEDBACKPROC = *u8;
    type PFNGLRESUMETRANSFORMFEEDBACKPROC = *u8;
    type PFNGLDRAWTRANSFORMFEEDBACKPROC = *u8;
}

pub mod arb_transform_feedback3 {
    #[nolink]
    extern {
        // fn glDrawTransformFeedbackStream(++mode: GLenum, ++id: GLuint, ++stream: GLuint);
        // fn glBeginQueryIndexed(++target: GLenum, ++index: GLuint, ++id: GLuint);
        // fn glEndQueryIndexed(++target: GLenum, ++index: GLuint);
        // fn glGetQueryIndexediv(++target: GLenum, ++index: GLuint, ++pname: GLenum, ++params: *GLint);
    }
    type PFNGLDRAWTRANSFORMFEEDBACKSTREAMPROC = *u8;
    type PFNGLBEGINQUERYINDEXEDPROC = *u8;
    type PFNGLENDQUERYINDEXEDPROC = *u8;
    type PFNGLGETQUERYINDEXEDIVPROC = *u8;
}

pub mod arb_es2_compatibility {
    #[nolink]
    extern {
        // fn glReleaseShaderCompiler();
        // fn glShaderBinary(++count: GLsizei, ++shaders: *GLuint, ++binaryformat: GLenum, ++binary: *GLvoid, ++length: GLsizei);
        // fn glGetShaderPrecisionFormat(++shadertype: GLenum, ++precisiontype: GLenum, ++range: *GLint, ++precision: *GLint);
        // fn glDepthRangef(++n: GLfloat, ++f: GLfloat);
        // fn glClearDepthf(++d: GLfloat);
    }
    type PFNGLRELEASESHADERCOMPILERPROC = *u8;
    type PFNGLSHADERBINARYPROC = *u8;
    type PFNGLGETSHADERPRECISIONFORMATPROC = *u8;
    type PFNGLDEPTHRANGEFPROC = *u8;
    type PFNGLCLEARDEPTHFPROC = *u8;
}

pub mod arb_get_program_binary {
    #[nolink]
    extern {
        // fn glGetProgramBinary(++program: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++binaryFormat: *GLenum, ++binary: *GLvoid);
        // fn glProgramBinary(++program: GLuint, ++binaryFormat: GLenum, ++binary: *GLvoid, ++length: GLsizei);
        // fn glProgramParameteri(++program: GLuint, ++pname: GLenum, ++value: GLint);
    }
    type PFNGLGETPROGRAMBINARYPROC = *u8;
    type PFNGLPROGRAMBINARYPROC = *u8;
    type PFNGLPROGRAMPARAMETERIPROC = *u8;
}

pub mod arb_separate_shader_objects {
    #[nolink]
    extern {
        // fn glUseProgramStages(++pipeline: GLuint, ++stages: GLbitfield, ++program: GLuint);
        // fn glActiveShaderProgram(++pipeline: GLuint, ++program: GLuint);
        // fn glCreateShaderProgramv(++gltype: GLenum, ++count: GLsizei, ++strings: **GLchar) -> GLuint;
        // fn glBindProgramPipeline(++pipeline: GLuint);
        // fn glDeleteProgramPipelines(++n: GLsizei, ++pipelines: *GLuint);
        // fn glGenProgramPipelines(++n: GLsizei, ++pipelines: *GLuint);
        // fn glIsProgramPipeline(++pipeline: GLuint) -> GLboolean;
        // fn glGetProgramPipelineiv(++pipeline: GLuint, ++pname: GLenum, ++params: *GLint);
        // fn glProgramUniform1i(++program: GLuint, ++location: GLint, ++v0: GLint);
        // fn glProgramUniform1iv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLint);
        // fn glProgramUniform1f(++program: GLuint, ++location: GLint, ++v0: GLfloat);
        // fn glProgramUniform1fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        // fn glProgramUniform1d(++program: GLuint, ++location: GLint, ++v0: GLdouble);
        // fn glProgramUniform1dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glProgramUniform1ui(++program: GLuint, ++location: GLint, ++v0: GLuint);
        // fn glProgramUniform1uiv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLuint);
        // fn glProgramUniform2i(++program: GLuint, ++location: GLint, ++v0: GLint, ++v1: GLint);
        // fn glProgramUniform2iv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLint);
        // fn glProgramUniform2f(++program: GLuint, ++location: GLint, ++v0: GLfloat, ++v1: GLfloat);
        // fn glProgramUniform2fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        // fn glProgramUniform2d(++program: GLuint, ++location: GLint, ++v0: GLdouble, ++v1: GLdouble);
        // fn glProgramUniform2dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glProgramUniform2ui(++program: GLuint, ++location: GLint, ++v0: GLuint, ++v1: GLuint);
        // fn glProgramUniform2uiv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLuint);
        // fn glProgramUniform3i(++program: GLuint, ++location: GLint, ++v0: GLint, ++v1: GLint, ++v2: GLint);
        // fn glProgramUniform3iv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLint);
        // fn glProgramUniform3f(++program: GLuint, ++location: GLint, ++v0: GLfloat, ++v1: GLfloat, ++v2: GLfloat);
        // fn glProgramUniform3fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        // fn glProgramUniform3d(++program: GLuint, ++location: GLint, ++v0: GLdouble, ++v1: GLdouble, ++v2: GLdouble);
        // fn glProgramUniform3dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glProgramUniform3ui(++program: GLuint, ++location: GLint, ++v0: GLuint, ++v1: GLuint, ++v2: GLuint);
        // fn glProgramUniform3uiv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLuint);
        // fn glProgramUniform4i(++program: GLuint, ++location: GLint, ++v0: GLint, ++v1: GLint, ++v2: GLint, ++v3: GLint);
        // fn glProgramUniform4iv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLint);
        // fn glProgramUniform4f(++program: GLuint, ++location: GLint, ++v0: GLfloat, ++v1: GLfloat, ++v2: GLfloat, ++v3: GLfloat);
        // fn glProgramUniform4fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLfloat);
        // fn glProgramUniform4d(++program: GLuint, ++location: GLint, ++v0: GLdouble, ++v1: GLdouble, ++v2: GLdouble, ++v3: GLdouble);
        // fn glProgramUniform4dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLdouble);
        // fn glProgramUniform4ui(++program: GLuint, ++location: GLint, ++v0: GLuint, ++v1: GLuint, ++v2: GLuint, ++v3: GLuint);
        // fn glProgramUniform4uiv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++value: *GLuint);
        // fn glProgramUniformMatrix2fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix3fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix4fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix2dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix3dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix4dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix2x3fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix3x2fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix2x4fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix4x2fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix3x4fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix4x3fv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);
        // fn glProgramUniformMatrix2x3dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix3x2dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix2x4dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix4x2dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix3x4dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glProgramUniformMatrix4x3dv(++program: GLuint, ++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLdouble);
        // fn glValidateProgramPipeline(++pipeline: GLuint);
        // fn glGetProgramPipelineInfoLog(++pipeline: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++infoLog: *GLchar);
    }
    type PFNGLUSEPROGRAMSTAGESPROC = *u8;
    type PFNGLACTIVESHADERPROGRAMPROC = *u8;
    type PFNGLCREATESHADERPROGRAMVPROC = *u8;
    type PFNGLBINDPROGRAMPIPELINEPROC = *u8;
    type PFNGLDELETEPROGRAMPIPELINESPROC = *u8;
    type PFNGLGENPROGRAMPIPELINESPROC = *u8;
    type PFNGLISPROGRAMPIPELINEPROC = *u8;
    type PFNGLGETPROGRAMPIPELINEIVPROC = *u8;
    type PFNGLPROGRAMUNIFORM1IPROC = *u8;
    type PFNGLPROGRAMUNIFORM1IVPROC = *u8;
    type PFNGLPROGRAMUNIFORM1FPROC = *u8;
    type PFNGLPROGRAMUNIFORM1FVPROC = *u8;
    type PFNGLPROGRAMUNIFORM1DPROC = *u8;
    type PFNGLPROGRAMUNIFORM1DVPROC = *u8;
    type PFNGLPROGRAMUNIFORM1UIPROC = *u8;
    type PFNGLPROGRAMUNIFORM1UIVPROC = *u8;
    type PFNGLPROGRAMUNIFORM2IPROC = *u8;
    type PFNGLPROGRAMUNIFORM2IVPROC = *u8;
    type PFNGLPROGRAMUNIFORM2FPROC = *u8;
    type PFNGLPROGRAMUNIFORM2FVPROC = *u8;
    type PFNGLPROGRAMUNIFORM2DPROC = *u8;
    type PFNGLPROGRAMUNIFORM2DVPROC = *u8;
    type PFNGLPROGRAMUNIFORM2UIPROC = *u8;
    type PFNGLPROGRAMUNIFORM2UIVPROC = *u8;
    type PFNGLPROGRAMUNIFORM3IPROC = *u8;
    type PFNGLPROGRAMUNIFORM3IVPROC = *u8;
    type PFNGLPROGRAMUNIFORM3FPROC = *u8;
    type PFNGLPROGRAMUNIFORM3FVPROC = *u8;
    type PFNGLPROGRAMUNIFORM3DPROC = *u8;
    type PFNGLPROGRAMUNIFORM3DVPROC = *u8;
    type PFNGLPROGRAMUNIFORM3UIPROC = *u8;
    type PFNGLPROGRAMUNIFORM3UIVPROC = *u8;
    type PFNGLPROGRAMUNIFORM4IPROC = *u8;
    type PFNGLPROGRAMUNIFORM4IVPROC = *u8;
    type PFNGLPROGRAMUNIFORM4FPROC = *u8;
    type PFNGLPROGRAMUNIFORM4FVPROC = *u8;
    type PFNGLPROGRAMUNIFORM4DPROC = *u8;
    type PFNGLPROGRAMUNIFORM4DVPROC = *u8;
    type PFNGLPROGRAMUNIFORM4UIPROC = *u8;
    type PFNGLPROGRAMUNIFORM4UIVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX2FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX3FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX4FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX2DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX3DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX4DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX2X3FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX3X2FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX2X4FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX4X2FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX3X4FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX4X3FVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX2X3DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX3X2DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX2X4DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX4X2DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX3X4DVPROC = *u8;
    type PFNGLPROGRAMUNIFORMMATRIX4X3DVPROC = *u8;
    type PFNGLVALIDATEPROGRAMPIPELINEPROC = *u8;
    type PFNGLGETPROGRAMPIPELINEINFOLOGPROC = *u8;
}

pub mod arb_vertex_attrib_64bit {
    #[nolink]
    extern {
        // fn glVertexAttribL1d(++index: GLuint, ++x: GLdouble);
        // fn glVertexAttribL2d(++index: GLuint, ++x: GLdouble, ++y: GLdouble);
        // fn glVertexAttribL3d(++index: GLuint, ++x: GLdouble, ++y: GLdouble, ++z: GLdouble);
        // fn glVertexAttribL4d(++index: GLuint, ++x: GLdouble, ++y: GLdouble, ++z: GLdouble, ++w: GLdouble);
        // fn glVertexAttribL1dv(++index: GLuint, ++v: *GLdouble);
        // fn glVertexAttribL2dv(++index: GLuint, ++v: *GLdouble);
        // fn glVertexAttribL3dv(++index: GLuint, ++v: *GLdouble);
        // fn glVertexAttribL4dv(++index: GLuint, ++v: *GLdouble);
        // fn glVertexAttribLPointer(++index: GLuint, ++size: GLint, ++gltype: GLenum, ++stride: GLsizei, ++pointer: *GLvoid);
        // fn glGetVertexAttribLdv(++index: GLuint, ++pname: GLenum, ++params: *GLdouble);
    }
    type PFNGLVERTEXATTRIBL1DPROC = *u8;
    type PFNGLVERTEXATTRIBL2DPROC = *u8;
    type PFNGLVERTEXATTRIBL3DPROC = *u8;
    type PFNGLVERTEXATTRIBL4DPROC = *u8;
    type PFNGLVERTEXATTRIBL1DVPROC = *u8;
    type PFNGLVERTEXATTRIBL2DVPROC = *u8;
    type PFNGLVERTEXATTRIBL3DVPROC = *u8;
    type PFNGLVERTEXATTRIBL4DVPROC = *u8;
    type PFNGLVERTEXATTRIBLPOINTERPROC = *u8;
    type PFNGLGETVERTEXATTRIBLDVPROC = *u8;
}

pub mod arb_viewport_array {
    #[nolink]
    extern {
        // fn glViewportArrayv(++first: GLuint, ++count: GLsizei, ++v: *GLfloat);
        // fn glViewportIndexedf(++index: GLuint, ++x: GLfloat, ++y: GLfloat, ++w: GLfloat, ++h: GLfloat);
        // fn glViewportIndexedfv(++index: GLuint, ++v: *GLfloat);
        // fn glScissorArrayv(++first: GLuint, ++count: GLsizei, ++v: *GLint);
        // fn glScissorIndexed(++index: GLuint, ++left: GLint, ++bottom: GLint, ++width: GLsizei, ++height: GLsizei);
        // fn glScissorIndexedv(++index: GLuint, ++v: *GLint);
        // fn glDepthRangeArrayv(++first: GLuint, ++count: GLsizei, ++v: *GLdouble);
        // fn glDepthRangeIndexed(++index: GLuint, ++n: GLdouble, ++f: GLdouble);
        // fn glGetFloati_v(++target: GLenum, ++index: GLuint, ++data: *GLfloat);
        // fn glGetDoublei_v(++target: GLenum, ++index: GLuint, ++data: *GLdouble);
    }
    type PFNGLVIEWPORTARRAYVPROC = *u8;
    type PFNGLVIEWPORTINDEXEDFPROC = *u8;
    type PFNGLVIEWPORTINDEXEDFVPROC = *u8;
    type PFNGLSCISSORARRAYVPROC = *u8;
    type PFNGLSCISSORINDEXEDPROC = *u8;
    type PFNGLSCISSORINDEXEDVPROC = *u8;
    type PFNGLDEPTHRANGEARRAYVPROC = *u8;
    type PFNGLDEPTHRANGEINDEXEDPROC = *u8;
    type PFNGLGETFLOATI_VPROC = *u8;
    type PFNGLGETDOUBLEI_VPROC = *u8;
}

pub mod arb_cl_event {
    #[nolink]
    extern {
        // fn glCreateSyncFromCLeventARB(++context: *Struct__cl_context, ++event: *Struct__cl_event, ++flags: GLbitfield) -> GLsync;
    }
    type PFNGLCREATESYNCFROMCLEVENTARBPROC = *u8;
}

pub mod arb_debug_output {
    #[nolink]
    extern {
        // fn glDebugMessageControlARB(++source: GLenum, ++gltype: GLenum, ++severity: GLenum, ++count: GLsizei, ++ids: *GLuint, ++enabled: GLboolean);
        // fn glDebugMessageInsertARB(++source: GLenum, ++gltype: GLenum, ++id: GLuint, ++severity: GLenum, ++length: GLsizei, ++buf: *GLchar);
        // fn glDebugMessageCallbackARB(++callback: GLDEBUGPROCARB, ++userParam: *GLvoid);
        // fn glGetDebugMessageLogARB(++count: GLuint, ++bufsize: GLsizei, ++sources: *GLenum, ++types: *GLenum, ++ids: *GLuint, ++severities: *GLenum, ++lengths: *GLsizei, ++messageLog: *GLchar) -> GLuint;
    }
    type PFNGLDEBUGMESSAGECONTROLARBPROC = *u8;
    type PFNGLDEBUGMESSAGEINSERTARBPROC = *u8;
    type PFNGLDEBUGMESSAGECALLBACKARBPROC = *u8;
    type PFNGLGETDEBUGMESSAGELOGARBPROC = *u8;
}

pub mod arb_robustness {
    #[nolink]
    extern {
        // fn glGetGraphicsResetStatusARB() -> GLenum;
        // fn glGetnTexImageARB(++target: GLenum, ++level: GLint, ++format: GLenum, ++gltype: GLenum, ++bufSize: GLsizei, ++img: *GLvoid);
        // fn glReadnPixelsARB(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++gltype: GLenum, ++bufSize: GLsizei, ++data: *GLvoid);
        // fn glGetnCompressedTexImageARB(++target: GLenum, ++lod: GLint, ++bufSize: GLsizei, ++img: *GLvoid);
        // fn glGetnUniformfvARB(++program: GLuint, ++location: GLint, ++bufSize: GLsizei, ++params: *GLfloat);
        // fn glGetnUniformivARB(++program: GLuint, ++location: GLint, ++bufSize: GLsizei, ++params: *GLint);
        // fn glGetnUniformuivARB(++program: GLuint, ++location: GLint, ++bufSize: GLsizei, ++params: *GLuint);
        // fn glGetnUniformdvARB(++program: GLuint, ++location: GLint, ++bufSize: GLsizei, ++params: *GLdouble);
    }
    type PFNGLGETGRAPHICSRESETSTATUSARBPROC = *u8;
    type PFNGLGETNTEXIMAGEARBPROC = *u8;
    type PFNGLREADNPIXELSARBPROC = *u8;
    type PFNGLGETNCOMPRESSEDTEXIMAGEARBPROC = *u8;
    type PFNGLGETNUNIFORMFVARBPROC = *u8;
    type PFNGLGETNUNIFORMIVARBPROC = *u8;
    type PFNGLGETNUNIFORMUIVARBPROC = *u8;
    type PFNGLGETNUNIFORMDVARBPROC = *u8;
}

pub mod arb_shader_stencil_export {
}

pub mod arb_base_instance {
    #[nolink]
    extern {
        // fn glDrawArraysInstancedBaseInstance(++mode: GLenum, ++first: GLint, ++count: GLsizei, ++instancecount: GLsizei, ++baseinstance: GLuint);
        // fn glDrawElementsInstancedBaseInstance(++mode: GLenum, ++count: GLsizei, ++gltype: GLenum, ++indices: *libc::c_void, ++instancecount: GLsizei, ++baseinstance: GLuint);
        // fn glDrawElementsInstancedBaseVertexBaseInstance(++mode: GLenum, ++count: GLsizei, ++gltype: GLenum, ++indices: *libc::c_void, ++instancecount: GLsizei, ++basevertex: GLint, ++baseinstance: GLuint);
    }
    type PFNGLDRAWARRAYSINSTANCEDBASEINSTANCEPROC = *u8;
    type PFNGLDRAWELEMENTSINSTANCEDBASEINSTANCEPROC = *u8;
    type PFNGLDRAWELEMENTSINSTANCEDBASEVERTEXBASEINSTANCEPROC = *u8;
}

pub mod arb_shading_language_420pack {
}

pub mod arb_transform_feedback_instanced {
    #[nolink]
    extern {
        // fn glDrawTransformFeedbackInstanced(++mode: GLenum, ++id: GLuint, ++instancecount: GLsizei);
        // fn glDrawTransformFeedbackStreamInstanced(++mode: GLenum, ++id: GLuint, ++stream: GLuint, ++instancecount: GLsizei);
    }
    type PFNGLDRAWTRANSFORMFEEDBACKINSTANCEDPROC = *u8;
    type PFNGLDRAWTRANSFORMFEEDBACKSTREAMINSTANCEDPROC = *u8;
}

pub mod arb_compressed_texture_pixel_storage {
}

pub mod arb_conservative_depth {
}

pub mod arb_internalformat_query {
    #[nolink]
    extern {
        // fn glGetInternalformativ(++target: GLenum, ++internalformat: GLenum, ++pname: GLenum, ++bufSize: GLsizei, ++params: *GLint);
    }
    type PFNGLGETINTERNALFORMATIVPROC = *u8;
}

pub mod arb_map_buffer_alignment {
}

pub mod arb_shader_atomic_counters {
    #[nolink]
    extern {
        // fn glGetActiveAtomicCounterBufferiv(++program: GLuint, ++bufferIndex: GLuint, ++pname: GLenum, ++params: *GLint);
    }
    type PFNGLGETACTIVEATOMICCOUNTERBUFFERIVPROC = *u8;
}

pub mod arb_shader_image_load_store {
    #[nolink]
    extern {
        // fn glBindImageTexture(++unit: GLuint, ++texture: GLuint, ++level: GLint, ++layered: GLboolean, ++layer: GLint, ++access: GLenum, ++format: GLenum);
        // fn glMemoryBarrier(++barriers: GLbitfield);
    }
    type PFNGLBINDIMAGETEXTUREPROC = *u8;
    type PFNGLMEMORYBARRIERPROC = *u8;
}

pub mod arb_shading_language_packing {
}

pub mod arb_texture_storage {
    #[nolink]
    extern {
        // fn glTexStorage1D(++target: GLenum, ++levels: GLsizei, ++internalformat: GLenum, ++width: GLsizei);
        // fn glTexStorage2D(++target: GLenum, ++levels: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei);
        // fn glTexStorage3D(++target: GLenum, ++levels: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei);
        // fn glTextureStorage1DEXT(++texture: GLuint, ++target: GLenum, ++levels: GLsizei, ++internalformat: GLenum, ++width: GLsizei);
        // fn glTextureStorage2DEXT(++texture: GLuint, ++target: GLenum, ++levels: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei);
        // fn glTextureStorage3DEXT(++texture: GLuint, ++target: GLenum, ++levels: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei);
    }
    type PFNGLTEXSTORAGE1DPROC = *u8;
    type PFNGLTEXSTORAGE2DPROC = *u8;
    type PFNGLTEXSTORAGE3DPROC = *u8;
    type PFNGLTEXTURESTORAGE1DEXTPROC = *u8;
    type PFNGLTEXTURESTORAGE2DEXTPROC = *u8;
    type PFNGLTEXTURESTORAGE3DEXTPROC = *u8;
}

pub mod khr_texture_compression_astc_ldr {
}

pub mod khr_debug {
    #[nolink]
    extern {
        // fn glDebugMessageControl(++source: GLenum, ++gltype: GLenum, ++severity: GLenum, ++count: GLsizei, ++ids: *GLuint, ++enabled: GLboolean);
        // fn glDebugMessageInsert(++source: GLenum, ++gltype: GLenum, ++id: GLuint, ++severity: GLenum, ++length: GLsizei, ++buf: *GLchar);
        // fn glDebugMessageCallback(++callback: GLDEBUGPROC, ++userParam: *libc::c_void);
        // fn glGetDebugMessageLog(++count: GLuint, ++bufsize: GLsizei, ++sources: *GLenum, ++types: *GLenum, ++ids: *GLuint, ++severities: *GLenum, ++lengths: *GLsizei, ++messageLog: *GLchar) -> GLuint;
        // fn glPushDebugGroup(++source: GLenum, ++id: GLuint, ++length: GLsizei, ++message: *GLchar);
        // fn glPopDebugGroup();
        // fn glObjectLabel(++identifier: GLenum, ++name: GLuint, ++length: GLsizei, ++label: *GLchar);
        // fn glGetObjectLabel(++identifier: GLenum, ++name: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++label: *GLchar);
        // fn glObjectPtrLabel(++ptr: *libc::c_void, ++length: GLsizei, ++label: *GLchar);
        // fn glGetObjectPtrLabel(++ptr: *libc::c_void, ++bufSize: GLsizei, ++length: *GLsizei, ++label: *GLchar);
    }
    type PFNGLDEBUGMESSAGECONTROLPROC = *u8;
    type PFNGLDEBUGMESSAGEINSERTPROC = *u8;
    type PFNGLDEBUGMESSAGECALLBACKPROC = *u8;
    type PFNGLGETDEBUGMESSAGELOGPROC = *u8;
    type PFNGLPUSHDEBUGGROUPPROC = *u8;
    type PFNGLPOPDEBUGGROUPPROC = *u8;
    type PFNGLOBJECTLABELPROC = *u8;
    type PFNGLGETOBJECTLABELPROC = *u8;
    type PFNGLOBJECTPTRLABELPROC = *u8;
    type PFNGLGETOBJECTPTRLABELPROC = *u8;
}

pub mod arb_arrays_of_arrays {
}

pub mod arb_clear_buffer_object {
    #[nolink]
    extern {
        // fn glClearBufferData(++target: GLenum, ++internalformat: GLenum, ++format: GLenum, ++gltype: GLenum, ++data: *libc::c_void);
        // fn glClearBufferSubData(++target: GLenum, ++internalformat: GLenum, ++offset: GLintptr, ++size: GLsizeiptr, ++format: GLenum, ++gltype: GLenum, ++data: *libc::c_void);
        // fn glClearNamedBufferDataEXT(++buffer: GLuint, ++internalformat: GLenum, ++format: GLenum, ++gltype: GLenum, ++data: *libc::c_void);
        // fn glClearNamedBufferSubDataEXT(++buffer: GLuint, ++internalformat: GLenum, ++format: GLenum, ++gltype: GLenum, ++offset: GLsizeiptr, ++size: GLsizeiptr, ++data: *libc::c_void);
    }
    type PFNGLCLEARBUFFERDATAPROC = *u8;
    type PFNGLCLEARBUFFERSUBDATAPROC = *u8;
    type PFNGLCLEARNAMEDBUFFERDATAEXTPROC = *u8;
    type PFNGLCLEARNAMEDBUFFERSUBDATAEXTPROC = *u8;
}

pub mod arb_compute_shader {
    #[nolink]
    extern {
        // fn glDispatchCompute(++num_groups_x: GLuint, ++num_groups_y: GLuint, ++num_groups_z: GLuint);
        // fn glDispatchComputeIndirect(++indirect: GLintptr);
    }
    type PFNGLDISPATCHCOMPUTEPROC = *u8;
    type PFNGLDISPATCHCOMPUTEINDIRECTPROC = *u8;
}

pub mod arb_copy_image {
    #[nolink]
    extern {
        // fn glCopyImageSubData(++srcName: GLuint, ++srcTarget: GLenum, ++srcLevel: GLint, ++srcX: GLint, ++srcY: GLint, ++srcZ: GLint, ++dstName: GLuint, ++dstTarget: GLenum, ++dstLevel: GLint, ++dstX: GLint, ++dstY: GLint, ++dstZ: GLint, ++srcWidth: GLsizei, ++srcHeight: GLsizei, ++srcDepth: GLsizei);
    }
    type PFNGLCOPYIMAGESUBDATAPROC = *u8;
}

pub mod arb_texture_view {
    #[nolink]
    extern {
        // fn glTextureView(++texture: GLuint, ++target: GLenum, ++origtexture: GLuint, ++internalformat: GLenum, ++minlevel: GLuint, ++numlevels: GLuint, ++minlayer: GLuint, ++numlayers: GLuint);
    }
    type PFNGLTEXTUREVIEWPROC = *u8;
}

pub mod arb_vertex_attrib_binding {
    #[nolink]
    extern {
        // fn glBindVertexBuffer(++bindingindex: GLuint, ++buffer: GLuint, ++offset: GLintptr, ++stride: GLsizei);
        // fn glVertexAttribFormat(++attribindex: GLuint, ++size: GLint, ++gltype: GLenum, ++normalized: GLboolean, ++relativeoffset: GLuint);
        // fn glVertexAttribIFormat(++attribindex: GLuint, ++size: GLint, ++gltype: GLenum, ++relativeoffset: GLuint);
        // fn glVertexAttribLFormat(++attribindex: GLuint, ++size: GLint, ++gltype: GLenum, ++relativeoffset: GLuint);
        // fn glVertexAttribBinding(++attribindex: GLuint, ++bindingindex: GLuint);
        // fn glVertexBindingDivisor(++bindingindex: GLuint, ++divisor: GLuint);
        // fn glVertexArrayBindVertexBufferEXT(++vaobj: GLuint, ++bindingindex: GLuint, ++buffer: GLuint, ++offset: GLintptr, ++stride: GLsizei);
        // fn glVertexArrayVertexAttribFormatEXT(++vaobj: GLuint, ++attribindex: GLuint, ++size: GLint, ++gltype: GLenum, ++normalized: GLboolean, ++relativeoffset: GLuint);
        // fn glVertexArrayVertexAttribIFormatEXT(++vaobj: GLuint, ++attribindex: GLuint, ++size: GLint, ++gltype: GLenum, ++relativeoffset: GLuint);
        // fn glVertexArrayVertexAttribLFormatEXT(++vaobj: GLuint, ++attribindex: GLuint, ++size: GLint, ++gltype: GLenum, ++relativeoffset: GLuint);
        // fn glVertexArrayVertexAttribBindingEXT(++vaobj: GLuint, ++attribindex: GLuint, ++bindingindex: GLuint);
        // fn glVertexArrayVertexBindingDivisorEXT(++vaobj: GLuint, ++bindingindex: GLuint, ++divisor: GLuint);
    }
    type PFNGLBINDVERTEXBUFFERPROC = *u8;
    type PFNGLVERTEXATTRIBFORMATPROC = *u8;
    type PFNGLVERTEXATTRIBIFORMATPROC = *u8;
    type PFNGLVERTEXATTRIBLFORMATPROC = *u8;
    type PFNGLVERTEXATTRIBBINDINGPROC = *u8;
    type PFNGLVERTEXBINDINGDIVISORPROC = *u8;
    type PFNGLVERTEXARRAYBINDVERTEXBUFFEREXTPROC = *u8;
    type PFNGLVERTEXARRAYVERTEXATTRIBFORMATEXTPROC = *u8;
    type PFNGLVERTEXARRAYVERTEXATTRIBIFORMATEXTPROC = *u8;
    type PFNGLVERTEXARRAYVERTEXATTRIBLFORMATEXTPROC = *u8;
    type PFNGLVERTEXARRAYVERTEXATTRIBBINDINGEXTPROC = *u8;
    type PFNGLVERTEXARRAYVERTEXBINDINGDIVISOREXTPROC = *u8;
}

pub mod arb_robustness_isolation {
}

pub mod arb_es3_compatibility {
}

pub mod arb_explicit_uniform_location {
}

pub mod arb_fragment_layer_viewport {
}

pub mod arb_framebuffer_no_attachments {
    #[nolink]
    extern {
        // fn glFramebufferParameteri(++target: GLenum, ++pname: GLenum, ++param: GLint);
        // fn glGetFramebufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);
        // fn glNamedFramebufferParameteriEXT(++framebuffer: GLuint, ++pname: GLenum, ++param: GLint);
        // fn glGetNamedFramebufferParameterivEXT(++framebuffer: GLuint, ++pname: GLenum, ++params: *GLint);
    }
    type PFNGLFRAMEBUFFERPARAMETERIPROC = *u8;
    type PFNGLGETFRAMEBUFFERPARAMETERIVPROC = *u8;
    type PFNGLNAMEDFRAMEBUFFERPARAMETERIEXTPROC = *u8;
    type PFNGLGETNAMEDFRAMEBUFFERPARAMETERIVEXTPROC = *u8;
}

pub mod arb_internalformat_query2 {
    #[nolink]
    extern {
        // fn glGetInternalformati64v(++target: GLenum, ++internalformat: GLenum, ++pname: GLenum, ++bufSize: GLsizei, ++params: *GLint64);
    }
    type PFNGLGETINTERNALFORMATI64VPROC = *u8;
}

pub mod arb_invalidate_subdata {
    #[nolink]
    extern {
        // fn glInvalidateTexSubImage(++texture: GLuint, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++zoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei);
        // fn glInvalidateTexImage(++texture: GLuint, ++level: GLint);
        // fn glInvalidateBufferSubData(++buffer: GLuint, ++offset: GLintptr, ++length: GLsizeiptr);
        // fn glInvalidateBufferData(++buffer: GLuint);
        // fn glInvalidateFramebuffer(++target: GLenum, ++numAttachments: GLsizei, ++attachments: *GLenum);
        // fn glInvalidateSubFramebuffer(++target: GLenum, ++numAttachments: GLsizei, ++attachments: *GLenum, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);
    }
    type PFNGLINVALIDATETEXSUBIMAGEPROC = *u8;
    type PFNGLINVALIDATETEXIMAGEPROC = *u8;
    type PFNGLINVALIDATEBUFFERSUBDATAPROC = *u8;
    type PFNGLINVALIDATEBUFFERDATAPROC = *u8;
    type PFNGLINVALIDATEFRAMEBUFFERPROC = *u8;
    type PFNGLINVALIDATESUBFRAMEBUFFERPROC = *u8;
}

pub mod arb_multi_draw_indirect {
    #[nolink]
    extern {
        // fn glMultiDrawArraysIndirect(++mode: GLenum, ++indirect: *libc::c_void, ++drawcount: GLsizei, ++stride: GLsizei);
        // fn glMultiDrawElementsIndirect(++mode: GLenum, ++gltype: GLenum, ++indirect: *libc::c_void, ++drawcount: GLsizei, ++stride: GLsizei);
    }
    type PFNGLMULTIDRAWARRAYSINDIRECTPROC = *u8;
    type PFNGLMULTIDRAWELEMENTSINDIRECTPROC = *u8;
}

pub mod arb_program_interface_query {
    #[nolink]
    extern {
        // fn glGetProgramInterfaceiv(++program: GLuint, ++programInterface: GLenum, ++pname: GLenum, ++params: *GLint);
        // fn glGetProgramResourceIndex(++program: GLuint, ++programInterface: GLenum, ++name: *GLchar) -> GLuint;
        // fn glGetProgramResourceName(++program: GLuint, ++programInterface: GLenum, ++index: GLuint, ++bufSize: GLsizei, ++length: *GLsizei, ++name: *GLchar);
        // fn glGetProgramResourceiv(++program: GLuint, ++programInterface: GLenum, ++index: GLuint, ++propCount: GLsizei, ++props: *GLenum, ++bufSize: GLsizei, ++length: *GLsizei, ++params: *GLint);
        // fn glGetProgramResourceLocation(++program: GLuint, ++programInterface: GLenum, ++name: *GLchar) -> GLint;
        // fn glGetProgramResourceLocationIndex(++program: GLuint, ++programInterface: GLenum, ++name: *GLchar) -> GLint;
    }
    type PFNGLGETPROGRAMINTERFACEIVPROC = *u8;
    type PFNGLGETPROGRAMRESOURCEINDEXPROC = *u8;
    type PFNGLGETPROGRAMRESOURCENAMEPROC = *u8;
    type PFNGLGETPROGRAMRESOURCEIVPROC = *u8;
    type PFNGLGETPROGRAMRESOURCELOCATIONPROC = *u8;
    type PFNGLGETPROGRAMRESOURCELOCATIONINDEXPROC = *u8;
}

pub mod arb_robust_buffer_access_behavior {
}

pub mod arb_shader_image_size {
}

pub mod arb_shader_storage_buffer_object {
    #[nolink]
    extern {
        // fn glShaderStorageBlockBinding(++program: GLuint, ++storageBlockIndex: GLuint, ++storageBlockBinding: GLuint);
    }
    type PFNGLSHADERSTORAGEBLOCKBINDINGPROC = *u8;
}

pub mod arb_stencil_texturing {
}

pub mod arb_texture_buffer_range {
    #[nolink]
    extern {
        // fn glTexBufferRange(++target: GLenum, ++internalformat: GLenum, ++buffer: GLuint, ++offset: GLintptr, ++size: GLsizeiptr);
        // fn glTextureBufferRangeEXT(++texture: GLuint, ++target: GLenum, ++internalformat: GLenum, ++buffer: GLuint, ++offset: GLintptr, ++size: GLsizeiptr);
    }
    type PFNGLTEXBUFFERRANGEPROC = *u8;
    type PFNGLTEXTUREBUFFERRANGEEXTPROC = *u8;
}

pub mod arb_texture_query_levels {
}

pub mod arb_texture_storage_multisample {
    #[nolink]
    extern {
        // fn glTexStorage2DMultisample(++target: GLenum, ++samples: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++fixedsamplelocations: GLboolean);
        // fn glTexStorage3DMultisample(++target: GLenum, ++samples: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++fixedsamplelocations: GLboolean);
        // fn glTextureStorage2DMultisampleEXT(++texture: GLuint, ++target: GLenum, ++samples: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++fixedsamplelocations: GLboolean);
        // fn glTextureStorage3DMultisampleEXT(++texture: GLuint, ++target: GLenum, ++samples: GLsizei, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++depth: GLsizei, ++fixedsamplelocations: GLboolean);
    }
    type PFNGLTEXSTORAGE2DMULTISAMPLEPROC = *u8;
    type PFNGLTEXSTORAGE3DMULTISAMPLEPROC = *u8;
    type PFNGLTEXTURESTORAGE2DMULTISAMPLEEXTPROC = *u8;
    type PFNGLTEXTURESTORAGE3DMULTISAMPLEEXTPROC = *u8;
}