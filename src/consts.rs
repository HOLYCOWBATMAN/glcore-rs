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

#[cfg(version_1_1)]
pub mod version_1_1 {
    /* AttribMask */
    pub const GL_DEPTH_BUFFER_BIT               : GLenum = 0x00000100;
    pub const GL_STENCIL_BUFFER_BIT             : GLenum = 0x00000400;
    pub const GL_COLOR_BUFFER_BIT               : GLenum = 0x00004000;
    /* Boolean */
    pub const GL_FALSE                          : GLenum = 0;
    pub const GL_TRUE                           : GLenum = 1;
    /* BeginMode */
    pub const GL_POINTS                         : GLenum = 0x0000;
    pub const GL_LINES                          : GLenum = 0x0001;
    pub const GL_LINE_LOOP                      : GLenum = 0x0002;
    pub const GL_LINE_STRIP                     : GLenum = 0x0003;
    pub const GL_TRIANGLES                      : GLenum = 0x0004;
    pub const GL_TRIANGLE_STRIP                 : GLenum = 0x0005;
    pub const GL_TRIANGLE_FAN                   : GLenum = 0x0006;
    pub const GL_QUADS                          : GLenum = 0x0007;
    /* AlphaFunction */
    pub const GL_NEVER                          : GLenum = 0x0200;
    pub const GL_LESS                           : GLenum = 0x0201;
    pub const GL_EQUAL                          : GLenum = 0x0202;
    pub const GL_LEQUAL                         : GLenum = 0x0203;
    pub const GL_GREATER                        : GLenum = 0x0204;
    pub const GL_NOTEQUAL                       : GLenum = 0x0205;
    pub const GL_GEQUAL                         : GLenum = 0x0206;
    pub const GL_ALWAYS                         : GLenum = 0x0207;
    /* BlendingFactorDest */
    pub const GL_ZERO                           : GLenum = 0;
    pub const GL_ONE                            : GLenum = 1;
    pub const GL_SRC_COLOR                      : GLenum = 0x0300;
    pub const GL_ONE_MINUS_SRC_COLOR            : GLenum = 0x0301;
    pub const GL_SRC_ALPHA                      : GLenum = 0x0302;
    pub const GL_ONE_MINUS_SRC_ALPHA            : GLenum = 0x0303;
    pub const GL_DST_ALPHA                      : GLenum = 0x0304;
    pub const GL_ONE_MINUS_DST_ALPHA            : GLenum = 0x0305;
    /* BlendingFactorSrc */
    pub const GL_DST_COLOR                      : GLenum = 0x0306;
    pub const GL_ONE_MINUS_DST_COLOR            : GLenum = 0x0307;
    pub const GL_SRC_ALPHA_SATURATE             : GLenum = 0x0308;
    /* DrawBufferMode */
    pub const GL_NONE                           : GLenum = 0;
    pub const GL_FRONT_LEFT                     : GLenum = 0x0400;
    pub const GL_FRONT_RIGHT                    : GLenum = 0x0401;
    pub const GL_BACK_LEFT                      : GLenum = 0x0402;
    pub const GL_BACK_RIGHT                     : GLenum = 0x0403;
    pub const GL_FRONT                          : GLenum = 0x0404;
    pub const GL_BACK                           : GLenum = 0x0405;
    pub const GL_LEFT                           : GLenum = 0x0406;
    pub const GL_RIGHT                          : GLenum = 0x0407;
    pub const GL_FRONT_AND_BACK                 : GLenum = 0x0408;
    /* ErrorCode */
    pub const GL_NO_ERROR                       : GLenum = 0;
    pub const GL_INVALID_ENUM                   : GLenum = 0x0500;
    pub const GL_INVALID_VALUE                  : GLenum = 0x0501;
    pub const GL_INVALID_OPERATION              : GLenum = 0x0502;
    pub const GL_OUT_OF_MEMORY                  : GLenum = 0x0505;
    /* FrontFaceDirection */
    pub const GL_CW                             : GLenum = 0x0900;
    pub const GL_CCW                            : GLenum = 0x0901;
    /* GetPName */
    pub const GL_POINT_SIZE                     : GLenum = 0x0B11;
    pub const GL_POINT_SIZE_RANGE               : GLenum = 0x0B12;
    pub const GL_POINT_SIZE_GRANULARITY         : GLenum = 0x0B13;
    pub const GL_LINE_SMOOTH                    : GLenum = 0x0B20;
    pub const GL_LINE_WIDTH                     : GLenum = 0x0B21;
    pub const GL_LINE_WIDTH_RANGE               : GLenum = 0x0B22;
    pub const GL_LINE_WIDTH_GRANULARITY         : GLenum = 0x0B23;
    pub const GL_POLYGON_MODE                   : GLenum = 0x0B40;
    pub const GL_POLYGON_SMOOTH                 : GLenum = 0x0B41;
    pub const GL_CULL_FACE                      : GLenum = 0x0B44;
    pub const GL_CULL_FACE_MODE                 : GLenum = 0x0B45;
    pub const GL_FRONT_FACE                     : GLenum = 0x0B46;
    pub const GL_DEPTH_RANGE                    : GLenum = 0x0B70;
    pub const GL_DEPTH_TEST                     : GLenum = 0x0B71;
    pub const GL_DEPTH_WRITEMASK                : GLenum = 0x0B72;
    pub const GL_DEPTH_CLEAR_VALUE              : GLenum = 0x0B73;
    pub const GL_DEPTH_FUNC                     : GLenum = 0x0B74;
    pub const GL_STENCIL_TEST                   : GLenum = 0x0B90;
    pub const GL_STENCIL_CLEAR_VALUE            : GLenum = 0x0B91;
    pub const GL_STENCIL_FUNC                   : GLenum = 0x0B92;
    pub const GL_STENCIL_VALUE_MASK             : GLenum = 0x0B93;
    pub const GL_STENCIL_FAIL                   : GLenum = 0x0B94;
    pub const GL_STENCIL_PASS_DEPTH_FAIL        : GLenum = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS        : GLenum = 0x0B96;
    pub const GL_STENCIL_REF                    : GLenum = 0x0B97;
    pub const GL_STENCIL_WRITEMASK              : GLenum = 0x0B98;
    pub const GL_VIEWPORT                       : GLenum = 0x0BA2;
    pub const GL_DITHER                         : GLenum = 0x0BD0;
    pub const GL_BLEND_DST                      : GLenum = 0x0BE0;
    pub const GL_BLEND_SRC                      : GLenum = 0x0BE1;
    pub const GL_BLEND                          : GLenum = 0x0BE2;
    pub const GL_LOGIC_OP_MODE                  : GLenum = 0x0BF0;
    pub const GL_COLOR_LOGIC_OP                 : GLenum = 0x0BF2;
    pub const GL_DRAW_BUFFER                    : GLenum = 0x0C01;
    pub const GL_READ_BUFFER                    : GLenum = 0x0C02;
    pub const GL_SCISSOR_BOX                    : GLenum = 0x0C10;
    pub const GL_SCISSOR_TEST                   : GLenum = 0x0C11;
    pub const GL_COLOR_CLEAR_VALUE              : GLenum = 0x0C22;
    pub const GL_COLOR_WRITEMASK                : GLenum = 0x0C23;
    pub const GL_DOUBLEBUFFER                   : GLenum = 0x0C32;
    pub const GL_STEREO                         : GLenum = 0x0C33;
    pub const GL_LINE_SMOOTH_HINT               : GLenum = 0x0C52;
    pub const GL_POLYGON_SMOOTH_HINT            : GLenum = 0x0C53;
    pub const GL_UNPACK_SWAP_BYTES              : GLenum = 0x0CF0;
    pub const GL_UNPACK_LSB_FIRST               : GLenum = 0x0CF1;
    pub const GL_UNPACK_ROW_LENGTH              : GLenum = 0x0CF2;
    pub const GL_UNPACK_SKIP_ROWS               : GLenum = 0x0CF3;
    pub const GL_UNPACK_SKIP_PIXELS             : GLenum = 0x0CF4;
    pub const GL_UNPACK_ALIGNMENT               : GLenum = 0x0CF5;
    pub const GL_PACK_SWAP_BYTES                : GLenum = 0x0D00;
    pub const GL_PACK_LSB_FIRST                 : GLenum = 0x0D01;
    pub const GL_PACK_ROW_LENGTH                : GLenum = 0x0D02;
    pub const GL_PACK_SKIP_ROWS                 : GLenum = 0x0D03;
    pub const GL_PACK_SKIP_PIXELS               : GLenum = 0x0D04;
    pub const GL_PACK_ALIGNMENT                 : GLenum = 0x0D05;
    pub const GL_MAX_TEXTURE_SIZE               : GLenum = 0x0D33;
    pub const GL_MAX_VIEWPORT_DIMS              : GLenum = 0x0D3A;
    pub const GL_SUBPIXEL_BITS                  : GLenum = 0x0D50;
    pub const GL_TEXTURE_1D                     : GLenum = 0x0DE0;
    pub const GL_TEXTURE_2D                     : GLenum = 0x0DE1;
    pub const GL_POLYGON_OFFSET_UNITS           : GLenum = 0x2A00;
    pub const GL_POLYGON_OFFSET_POINT           : GLenum = 0x2A01;
    pub const GL_POLYGON_OFFSET_LINE            : GLenum = 0x2A02;
    pub const GL_POLYGON_OFFSET_FILL            : GLenum = 0x8037;
    pub const GL_POLYGON_OFFSET_FACTOR          : GLenum = 0x8038;
    pub const GL_TEXTURE_BINDING_1D             : GLenum = 0x8068;
    pub const GL_TEXTURE_BINDING_2D             : GLenum = 0x8069;
    /* GetTextureParameter */
    pub const GL_TEXTURE_WIDTH                  : GLenum = 0x1000;
    pub const GL_TEXTURE_HEIGHT                 : GLenum = 0x1001;
    pub const GL_TEXTURE_INTERNAL_FORMAT        : GLenum = 0x1003;
    pub const GL_TEXTURE_BORDER_COLOR           : GLenum = 0x1004;
    pub const GL_TEXTURE_RED_SIZE               : GLenum = 0x805C;
    pub const GL_TEXTURE_GREEN_SIZE             : GLenum = 0x805D;
    pub const GL_TEXTURE_BLUE_SIZE              : GLenum = 0x805E;
    pub const GL_TEXTURE_ALPHA_SIZE             : GLenum = 0x805F;
    /* HintMode */
    pub const GL_DONT_CARE                      : GLenum = 0x1100;
    pub const GL_FASTEST                        : GLenum = 0x1101;
    pub const GL_NICEST                         : GLenum = 0x1102;
    /* DataType */
    pub const GL_BYTE                           : GLenum = 0x1400;
    pub const GL_UNSIGNED_BYTE                  : GLenum = 0x1401;
    pub const GL_SHORT                          : GLenum = 0x1402;
    pub const GL_UNSIGNED_SHORT                 : GLenum = 0x1403;
    pub const GL_INT                            : GLenum = 0x1404;
    pub const GL_UNSIGNED_INT                   : GLenum = 0x1405;
    pub const GL_FLOAT                          : GLenum = 0x1406;
    pub const GL_DOUBLE                         : GLenum = 0x140A;
    /* ErrorCode */
    pub const GL_STACK_OVERFLOW                 : GLenum = 0x0503;
    pub const GL_STACK_UNDERFLOW                : GLenum = 0x0504;
    /* LogicOp */
    pub const GL_CLEAR                          : GLenum = 0x1500;
    pub const GL_AND                            : GLenum = 0x1501;
    pub const GL_AND_REVERSE                    : GLenum = 0x1502;
    pub const GL_COPY                           : GLenum = 0x1503;
    pub const GL_AND_INVERTED                   : GLenum = 0x1504;
    pub const GL_NOOP                           : GLenum = 0x1505;
    pub const GL_XOR                            : GLenum = 0x1506;
    pub const GL_OR                             : GLenum = 0x1507;
    pub const GL_NOR                            : GLenum = 0x1508;
    pub const GL_EQUIV                          : GLenum = 0x1509;
    pub const GL_INVERT                         : GLenum = 0x150A;
    pub const GL_OR_REVERSE                     : GLenum = 0x150B;
    pub const GL_COPY_INVERTED                  : GLenum = 0x150C;
    pub const GL_OR_INVERTED                    : GLenum = 0x150D;
    pub const GL_NAND                           : GLenum = 0x150E;
    pub const GL_SET                            : GLenum = 0x150F;
    /* MatrixMode (for gl3.h, FBO attachment type) */
    pub const GL_TEXTURE                        : GLenum = 0x1702;
    /* PixelCopyType */
    pub const GL_COLOR                          : GLenum = 0x1800;
    pub const GL_DEPTH                          : GLenum = 0x1801;
    pub const GL_STENCIL                        : GLenum = 0x1802;
    /* PixelFormat */
    pub const GL_STENCIL_INDEX                  : GLenum = 0x1901;
    pub const GL_DEPTH_COMPONENT                : GLenum = 0x1902;
    pub const GL_RED                            : GLenum = 0x1903;
    pub const GL_GREEN                          : GLenum = 0x1904;
    pub const GL_BLUE                           : GLenum = 0x1905;
    pub const GL_ALPHA                          : GLenum = 0x1906;
    pub const GL_RGB                            : GLenum = 0x1907;
    pub const GL_RGBA                           : GLenum = 0x1908;
    /* PolygonMode */
    pub const GL_POINT                          : GLenum = 0x1B00;
    pub const GL_LINE                           : GLenum = 0x1B01;
    pub const GL_FILL                           : GLenum = 0x1B02;
    /* StencilOp */
    pub const GL_KEEP                           : GLenum = 0x1E00;
    pub const GL_REPLACE                        : GLenum = 0x1E01;
    pub const GL_INCR                           : GLenum = 0x1E02;
    pub const GL_DECR                           : GLenum = 0x1E03;
    /* StringName */
    pub const GL_VENDOR                         : GLenum = 0x1F00;
    pub const GL_RENDERER                       : GLenum = 0x1F01;
    pub const GL_VERSION                        : GLenum = 0x1F02;
    pub const GL_EXTENSIONS                     : GLenum = 0x1F03;
    /* TextureMagFilter */
    pub const GL_NEAREST                        : GLenum = 0x2600;
    pub const GL_LINEAR                         : GLenum = 0x2601;
    /* TextureMinFilter */
    pub const GL_NEAREST_MIPMAP_NEAREST         : GLenum = 0x2700;
    pub const GL_LINEAR_MIPMAP_NEAREST          : GLenum = 0x2701;
    pub const GL_NEAREST_MIPMAP_LINEAR          : GLenum = 0x2702;
    pub const GL_LINEAR_MIPMAP_LINEAR           : GLenum = 0x2703;
    /* TextureParameterName */
    pub const GL_TEXTURE_MAG_FILTER             : GLenum = 0x2800;
    pub const GL_TEXTURE_MIN_FILTER             : GLenum = 0x2801;
    pub const GL_TEXTURE_WRAP_S                 : GLenum = 0x2802;
    pub const GL_TEXTURE_WRAP_T                 : GLenum = 0x2803;
    /* TextureTarget */
    pub const GL_PROXY_TEXTURE_1D               : GLenum = 0x8063;
    pub const GL_PROXY_TEXTURE_2D               : GLenum = 0x8064;
    /* TextureWrapMode */
    pub const GL_REPEAT                         : GLenum = 0x2901;
    /* PixelInternalFormat */
    pub const GL_R3_G3_B2                       : GLenum = 0x2A10;
    pub const GL_RGB4                           : GLenum = 0x804F;
    pub const GL_RGB5                           : GLenum = 0x8050;
    pub const GL_RGB8                           : GLenum = 0x8051;
    pub const GL_RGB10                          : GLenum = 0x8052;
    pub const GL_RGB12                          : GLenum = 0x8053;
    pub const GL_RGB16                          : GLenum = 0x8054;
    pub const GL_RGBA2                          : GLenum = 0x8055;
    pub const GL_RGBA4                          : GLenum = 0x8056;
    pub const GL_RGB5_A1                        : GLenum = 0x8057;
    pub const GL_RGBA8                          : GLenum = 0x8058;
    pub const GL_RGB10_A2                       : GLenum = 0x8059;
    pub const GL_RGBA12                         : GLenum = 0x805A;
    pub const GL_RGBA16                         : GLenum = 0x805B;
}

#[cfg(version_1_2)]
pub mod version_1_2 {
    pub const GL_UNSIGNED_BYTE_3_3_2            : GLenum = 0x8032;
    pub const GL_UNSIGNED_SHORT_4_4_4_4         : GLenum = 0x8033;
    pub const GL_UNSIGNED_SHORT_5_5_5_1         : GLenum = 0x8034;
    pub const GL_UNSIGNED_INT_8_8_8_8           : GLenum = 0x8035;
    pub const GL_UNSIGNED_INT_10_10_10_2        : GLenum = 0x8036;
    pub const GL_TEXTURE_BINDING_3D             : GLenum = 0x806A;
    pub const GL_PACK_SKIP_IMAGES               : GLenum = 0x806B;
    pub const GL_PACK_IMAGE_HEIGHT              : GLenum = 0x806C;
    pub const GL_UNPACK_SKIP_IMAGES             : GLenum = 0x806D;
    pub const GL_UNPACK_IMAGE_HEIGHT            : GLenum = 0x806E;
    pub const GL_TEXTURE_3D                     : GLenum = 0x806F;
    pub const GL_PROXY_TEXTURE_3D               : GLenum = 0x8070;
    pub const GL_TEXTURE_DEPTH                  : GLenum = 0x8071;
    pub const GL_TEXTURE_WRAP_R                 : GLenum = 0x8072;
    pub const GL_MAX_3D_TEXTURE_SIZE            : GLenum = 0x8073;
    pub const GL_UNSIGNED_BYTE_2_3_3_REV        : GLenum = 0x8362;
    pub const GL_UNSIGNED_SHORT_5_6_5           : GLenum = 0x8363;
    pub const GL_UNSIGNED_SHORT_5_6_5_REV       : GLenum = 0x8364;
    pub const GL_UNSIGNED_SHORT_4_4_4_4_REV     : GLenum = 0x8365;
    pub const GL_UNSIGNED_SHORT_1_5_5_5_REV     : GLenum = 0x8366;
    pub const GL_UNSIGNED_INT_8_8_8_8_REV       : GLenum = 0x8367;
    pub const GL_UNSIGNED_INT_2_10_10_10_REV    : GLenum = 0x8368;
    pub const GL_BGR                            : GLenum = 0x80E0;
    pub const GL_BGRA                           : GLenum = 0x80E1;
    pub const GL_MAX_ELEMENTS_VERTICES          : GLenum = 0x80E8;
    pub const GL_MAX_ELEMENTS_INDICES           : GLenum = 0x80E9;
    pub const GL_CLAMP_TO_EDGE                  : GLenum = 0x812F;
    pub const GL_TEXTURE_MIN_LOD                : GLenum = 0x813A;
    pub const GL_TEXTURE_MAX_LOD                : GLenum = 0x813B;
    pub const GL_TEXTURE_BASE_LEVEL             : GLenum = 0x813C;
    pub const GL_TEXTURE_MAX_LEVEL              : GLenum = 0x813D;
    pub const GL_SMOOTH_POINT_SIZE_RANGE        : GLenum = 0x0B12;
    pub const GL_SMOOTH_POINT_SIZE_GRANULARITY  : GLenum = 0x0B13;
    pub const GL_SMOOTH_LINE_WIDTH_RANGE        : GLenum = 0x0B22;
    pub const GL_SMOOTH_LINE_WIDTH_GRANULARITY  : GLenum = 0x0B23;
    pub const GL_ALIASED_LINE_WIDTH_RANGE       : GLenum = 0x846E;
}

#[cfg(arb_imaging)]
pub mod arb_imaging {
    pub const GL_CONSTANT_COLOR                 : GLenum = 0x8001;
    pub const GL_ONE_MINUS_CONSTANT_COLOR       : GLenum = 0x8002;
    pub const GL_CONSTANT_ALPHA                 : GLenum = 0x8003;
    pub const GL_ONE_MINUS_CONSTANT_ALPHA       : GLenum = 0x8004;
    pub const GL_BLEND_COLOR                    : GLenum = 0x8005;
    pub const GL_FUNC_ADD                       : GLenum = 0x8006;
    pub const GL_MIN                            : GLenum = 0x8007;
    pub const GL_MAX                            : GLenum = 0x8008;
    pub const GL_BLEND_EQUATION                 : GLenum = 0x8009;
    pub const GL_FUNC_SUBTRACT                  : GLenum = 0x800A;
    pub const GL_FUNC_REVERSE_SUBTRACT          : GLenum = 0x800B;
}

#[cfg(version_1_3)]
pub mod version_1_3 {
    pub const GL_TEXTURE0                       : GLenum = 0x84C0;
    pub const GL_TEXTURE1                       : GLenum = 0x84C1;
    pub const GL_TEXTURE2                       : GLenum = 0x84C2;
    pub const GL_TEXTURE3                       : GLenum = 0x84C3;
    pub const GL_TEXTURE4                       : GLenum = 0x84C4;
    pub const GL_TEXTURE5                       : GLenum = 0x84C5;
    pub const GL_TEXTURE6                       : GLenum = 0x84C6;
    pub const GL_TEXTURE7                       : GLenum = 0x84C7;
    pub const GL_TEXTURE8                       : GLenum = 0x84C8;
    pub const GL_TEXTURE9                       : GLenum = 0x84C9;
    pub const GL_TEXTURE10                      : GLenum = 0x84CA;
    pub const GL_TEXTURE11                      : GLenum = 0x84CB;
    pub const GL_TEXTURE12                      : GLenum = 0x84CC;
    pub const GL_TEXTURE13                      : GLenum = 0x84CD;
    pub const GL_TEXTURE14                      : GLenum = 0x84CE;
    pub const GL_TEXTURE15                      : GLenum = 0x84CF;
    pub const GL_TEXTURE16                      : GLenum = 0x84D0;
    pub const GL_TEXTURE17                      : GLenum = 0x84D1;
    pub const GL_TEXTURE18                      : GLenum = 0x84D2;
    pub const GL_TEXTURE19                      : GLenum = 0x84D3;
    pub const GL_TEXTURE20                      : GLenum = 0x84D4;
    pub const GL_TEXTURE21                      : GLenum = 0x84D5;
    pub const GL_TEXTURE22                      : GLenum = 0x84D6;
    pub const GL_TEXTURE23                      : GLenum = 0x84D7;
    pub const GL_TEXTURE24                      : GLenum = 0x84D8;
    pub const GL_TEXTURE25                      : GLenum = 0x84D9;
    pub const GL_TEXTURE26                      : GLenum = 0x84DA;
    pub const GL_TEXTURE27                      : GLenum = 0x84DB;
    pub const GL_TEXTURE28                      : GLenum = 0x84DC;
    pub const GL_TEXTURE29                      : GLenum = 0x84DD;
    pub const GL_TEXTURE30                      : GLenum = 0x84DE;
    pub const GL_TEXTURE31                      : GLenum = 0x84DF;
    pub const GL_ACTIVE_TEXTURE                 : GLenum = 0x84E0;
    pub const GL_MULTISAMPLE                    : GLenum = 0x809D;
    pub const GL_SAMPLE_ALPHA_TO_COVERAGE       : GLenum = 0x809E;
    pub const GL_SAMPLE_ALPHA_TO_ONE            : GLenum = 0x809F;
    pub const GL_SAMPLE_COVERAGE                : GLenum = 0x80A0;
    pub const GL_SAMPLE_BUFFERS                 : GLenum = 0x80A8;
    pub const GL_SAMPLES                        : GLenum = 0x80A9;
    pub const GL_SAMPLE_COVERAGE_VALUE          : GLenum = 0x80AA;
    pub const GL_SAMPLE_COVERAGE_INVERT         : GLenum = 0x80AB;
    pub const GL_TEXTURE_CUBE_MAP               : GLenum = 0x8513;
    pub const GL_TEXTURE_BINDING_CUBE_MAP       : GLenum = 0x8514;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_X    : GLenum = 0x8515;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_X    : GLenum = 0x8516;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Y    : GLenum = 0x8517;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Y    : GLenum = 0x8518;
    pub const GL_TEXTURE_CUBE_MAP_POSITIVE_Z    : GLenum = 0x8519;
    pub const GL_TEXTURE_CUBE_MAP_NEGATIVE_Z    : GLenum = 0x851A;
    pub const GL_PROXY_TEXTURE_CUBE_MAP         : GLenum = 0x851B;
    pub const GL_MAX_CUBE_MAP_TEXTURE_SIZE      : GLenum = 0x851C;
    pub const GL_COMPRESSED_RGB                 : GLenum = 0x84ED;
    pub const GL_COMPRESSED_RGBA                : GLenum = 0x84EE;
    pub const GL_TEXTURE_COMPRESSION_HINT       : GLenum = 0x84EF;
    pub const GL_TEXTURE_COMPRESSED_IMAGE_SIZE  : GLenum = 0x86A0;
    pub const GL_TEXTURE_COMPRESSED             : GLenum = 0x86A1;
    pub const GL_NUM_COMPRESSED_TEXTURE_FORMATS : GLenum = 0x86A2;
    pub const GL_COMPRESSED_TEXTURE_FORMATS     : GLenum = 0x86A3;
    pub const GL_CLAMP_TO_BORDER                : GLenum = 0x812D;
}

#[cfg(version_1_4)]
pub mod version_1_4 {
    pub const GL_BLEND_DST_RGB                  : GLenum = 0x80C8;
    pub const GL_BLEND_SRC_RGB                  : GLenum = 0x80C9;
    pub const GL_BLEND_DST_ALPHA                : GLenum = 0x80CA;
    pub const GL_BLEND_SRC_ALPHA                : GLenum = 0x80CB;
    pub const GL_POINT_FADE_THRESHOLD_SIZE      : GLenum = 0x8128;
    pub const GL_DEPTH_COMPONENT16              : GLenum = 0x81A5;
    pub const GL_DEPTH_COMPONENT24              : GLenum = 0x81A6;
    pub const GL_DEPTH_COMPONENT32              : GLenum = 0x81A7;
    pub const GL_MIRRORED_REPEAT                : GLenum = 0x8370;
    pub const GL_MAX_TEXTURE_LOD_BIAS           : GLenum = 0x84FD;
    pub const GL_TEXTURE_LOD_BIAS               : GLenum = 0x8501;
    pub const GL_INCR_WRAP                      : GLenum = 0x8507;
    pub const GL_DECR_WRAP                      : GLenum = 0x8508;
    pub const GL_TEXTURE_DEPTH_SIZE             : GLenum = 0x884A;
    pub const GL_TEXTURE_COMPARE_MODE           : GLenum = 0x884C;
    pub const GL_TEXTURE_COMPARE_FUNC           : GLenum = 0x884D;
}

#[cfg(version_1_5)]
pub mod version_1_5 {
    pub const GL_BUFFER_SIZE                    : GLenum = 0x8764;
    pub const GL_BUFFER_USAGE                   : GLenum = 0x8765;
    pub const GL_QUERY_COUNTER_BITS             : GLenum = 0x8864;
    pub const GL_CURRENT_QUERY                  : GLenum = 0x8865;
    pub const GL_QUERY_RESULT                   : GLenum = 0x8866;
    pub const GL_QUERY_RESULT_AVAILABLE         : GLenum = 0x8867;
    pub const GL_ARRAY_BUFFER                   : GLenum = 0x8892;
    pub const GL_ELEMENT_ARRAY_BUFFER           : GLenum = 0x8893;
    pub const GL_ARRAY_BUFFER_BINDING           : GLenum = 0x8894;
    pub const GL_ELEMENT_ARRAY_BUFFER_BINDING   : GLenum = 0x8895;
    pub const GL_VERTEX_ATTRIB_ARRAY_BUFFER_BINDING : GLenum = 0x889F;
    pub const GL_READ_ONLY                      : GLenum = 0x88B8;
    pub const GL_WRITE_ONLY                     : GLenum = 0x88B9;
    pub const GL_READ_WRITE                     : GLenum = 0x88BA;
    pub const GL_BUFFER_ACCESS                  : GLenum = 0x88BB;
    pub const GL_BUFFER_MAPPED                  : GLenum = 0x88BC;
    pub const GL_BUFFER_MAP_POINTER             : GLenum = 0x88BD;
    pub const GL_STREAM_DRAW                    : GLenum = 0x88E0;
    pub const GL_STREAM_READ                    : GLenum = 0x88E1;
    pub const GL_STREAM_COPY                    : GLenum = 0x88E2;
    pub const GL_STATIC_DRAW                    : GLenum = 0x88E4;
    pub const GL_STATIC_READ                    : GLenum = 0x88E5;
    pub const GL_STATIC_COPY                    : GLenum = 0x88E6;
    pub const GL_DYNAMIC_DRAW                   : GLenum = 0x88E8;
    pub const GL_DYNAMIC_READ                   : GLenum = 0x88E9;
    pub const GL_DYNAMIC_COPY                   : GLenum = 0x88EA;
    pub const GL_SAMPLES_PASSED                 : GLenum = 0x8914;
    pub const GL_SRC1_ALPHA                     : GLenum = 0x8589;
}

#[cfg(version_2_0)]
pub mod version_2_0 {
    pub const GL_BLEND_EQUATION_RGB             : GLenum = 0x8009;
    pub const GL_VERTEX_ATTRIB_ARRAY_ENABLED    : GLenum = 0x8622;
    pub const GL_VERTEX_ATTRIB_ARRAY_SIZE       : GLenum = 0x8623;
    pub const GL_VERTEX_ATTRIB_ARRAY_STRIDE     : GLenum = 0x8624;
    pub const GL_VERTEX_ATTRIB_ARRAY_TYPE       : GLenum = 0x8625;
    pub const GL_CURRENT_VERTEX_ATTRIB          : GLenum = 0x8626;
    pub const GL_VERTEX_PROGRAM_POINT_SIZE      : GLenum = 0x8642;
    pub const GL_VERTEX_ATTRIB_ARRAY_POINTER    : GLenum = 0x8645;
    pub const GL_STENCIL_BACK_FUNC              : GLenum = 0x8800;
    pub const GL_STENCIL_BACK_FAIL              : GLenum = 0x8801;
    pub const GL_STENCIL_BACK_PASS_DEPTH_FAIL   : GLenum = 0x8802;
    pub const GL_STENCIL_BACK_PASS_DEPTH_PASS   : GLenum = 0x8803;
    pub const GL_MAX_DRAW_BUFFERS               : GLenum = 0x8824;
    pub const GL_DRAW_BUFFER0                   : GLenum = 0x8825;
    pub const GL_DRAW_BUFFER1                   : GLenum = 0x8826;
    pub const GL_DRAW_BUFFER2                   : GLenum = 0x8827;
    pub const GL_DRAW_BUFFER3                   : GLenum = 0x8828;
    pub const GL_DRAW_BUFFER4                   : GLenum = 0x8829;
    pub const GL_DRAW_BUFFER5                   : GLenum = 0x882A;
    pub const GL_DRAW_BUFFER6                   : GLenum = 0x882B;
    pub const GL_DRAW_BUFFER7                   : GLenum = 0x882C;
    pub const GL_DRAW_BUFFER8                   : GLenum = 0x882D;
    pub const GL_DRAW_BUFFER9                   : GLenum = 0x882E;
    pub const GL_DRAW_BUFFER10                  : GLenum = 0x882F;
    pub const GL_DRAW_BUFFER11                  : GLenum = 0x8830;
    pub const GL_DRAW_BUFFER12                  : GLenum = 0x8831;
    pub const GL_DRAW_BUFFER13                  : GLenum = 0x8832;
    pub const GL_DRAW_BUFFER14                  : GLenum = 0x8833;
    pub const GL_DRAW_BUFFER15                  : GLenum = 0x8834;
    pub const GL_BLEND_EQUATION_ALPHA           : GLenum = 0x883D;
    pub const GL_MAX_VERTEX_ATTRIBS             : GLenum = 0x8869;
    pub const GL_VERTEX_ATTRIB_ARRAY_NORMALIZED : GLenum = 0x886A;
    pub const GL_MAX_TEXTURE_IMAGE_UNITS        : GLenum = 0x8872;
    pub const GL_FRAGMENT_SHADER                : GLenum = 0x8B30;
    pub const GL_VERTEX_SHADER                  : GLenum = 0x8B31;
    pub const GL_MAX_FRAGMENT_UNIFORM_COMPONENTS : GLenum = 0x8B49;
    pub const GL_MAX_VERTEX_UNIFORM_COMPONENTS  : GLenum = 0x8B4A;
    pub const GL_MAX_VARYING_FLOATS             : GLenum = 0x8B4B;
    pub const GL_MAX_VERTEX_TEXTURE_IMAGE_UNITS : GLenum = 0x8B4C;
    pub const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS : GLenum = 0x8B4D;
    pub const GL_SHADER_TYPE                    : GLenum = 0x8B4F;
    pub const GL_FLOAT_VEC2                     : GLenum = 0x8B50;
    pub const GL_FLOAT_VEC3                     : GLenum = 0x8B51;
    pub const GL_FLOAT_VEC4                     : GLenum = 0x8B52;
    pub const GL_INT_VEC2                       : GLenum = 0x8B53;
    pub const GL_INT_VEC3                       : GLenum = 0x8B54;
    pub const GL_INT_VEC4                       : GLenum = 0x8B55;
    pub const GL_BOOL                           : GLenum = 0x8B56;
    pub const GL_BOOL_VEC2                      : GLenum = 0x8B57;
    pub const GL_BOOL_VEC3                      : GLenum = 0x8B58;
    pub const GL_BOOL_VEC4                      : GLenum = 0x8B59;
    pub const GL_FLOAT_MAT2                     : GLenum = 0x8B5A;
    pub const GL_FLOAT_MAT3                     : GLenum = 0x8B5B;
    pub const GL_FLOAT_MAT4                     : GLenum = 0x8B5C;
    pub const GL_SAMPLER_1D                     : GLenum = 0x8B5D;
    pub const GL_SAMPLER_2D                     : GLenum = 0x8B5E;
    pub const GL_SAMPLER_3D                     : GLenum = 0x8B5F;
    pub const GL_SAMPLER_CUBE                   : GLenum = 0x8B60;
    pub const GL_SAMPLER_1D_SHADOW              : GLenum = 0x8B61;
    pub const GL_SAMPLER_2D_SHADOW              : GLenum = 0x8B62;
    pub const GL_DELETE_STATUS                  : GLenum = 0x8B80;
    pub const GL_COMPILE_STATUS                 : GLenum = 0x8B81;
    pub const GL_LINK_STATUS                    : GLenum = 0x8B82;
    pub const GL_VALIDATE_STATUS                : GLenum = 0x8B83;
    pub const GL_INFO_LOG_LENGTH                : GLenum = 0x8B84;
    pub const GL_ATTACHED_SHADERS               : GLenum = 0x8B85;
    pub const GL_ACTIVE_UNIFORMS                : GLenum = 0x8B86;
    pub const GL_ACTIVE_UNIFORM_MAX_LENGTH      : GLenum = 0x8B87;
    pub const GL_SHADER_SOURCE_LENGTH           : GLenum = 0x8B88;
    pub const GL_ACTIVE_ATTRIBUTES              : GLenum = 0x8B89;
    pub const GL_ACTIVE_ATTRIBUTE_MAX_LENGTH    : GLenum = 0x8B8A;
    pub const GL_FRAGMENT_SHADER_DERIVATIVE_HINT : GLenum = 0x8B8B;
    pub const GL_SHADING_LANGUAGE_VERSION       : GLenum = 0x8B8C;
    pub const GL_CURRENT_PROGRAM                : GLenum = 0x8B8D;
    pub const GL_POINT_SPRITE_COORD_ORIGIN      : GLenum = 0x8CA0;
    pub const GL_LOWER_LEFT                     : GLenum = 0x8CA1;
    pub const GL_UPPER_LEFT                     : GLenum = 0x8CA2;
    pub const GL_STENCIL_BACK_REF               : GLenum = 0x8CA3;
    pub const GL_STENCIL_BACK_VALUE_MASK        : GLenum = 0x8CA4;
    pub const GL_STENCIL_BACK_WRITEMASK         : GLenum = 0x8CA5;
}

#[cfg(version_2_1)]
pub mod version_2_1 {
    pub const GL_PIXEL_PACK_BUFFER              : GLenum = 0x88EB;
    pub const GL_PIXEL_UNPACK_BUFFER            : GLenum = 0x88EC;
    pub const GL_PIXEL_PACK_BUFFER_BINDING      : GLenum = 0x88ED;
    pub const GL_PIXEL_UNPACK_BUFFER_BINDING    : GLenum = 0x88EF;
    pub const GL_FLOAT_MAT2x3                   : GLenum = 0x8B65;
    pub const GL_FLOAT_MAT2x4                   : GLenum = 0x8B66;
    pub const GL_FLOAT_MAT3x2                   : GLenum = 0x8B67;
    pub const GL_FLOAT_MAT3x4                   : GLenum = 0x8B68;
    pub const GL_FLOAT_MAT4x2                   : GLenum = 0x8B69;
    pub const GL_FLOAT_MAT4x3                   : GLenum = 0x8B6A;
    pub const GL_SRGB                           : GLenum = 0x8C40;
    pub const GL_SRGB8                          : GLenum = 0x8C41;
    pub const GL_SRGB_ALPHA                     : GLenum = 0x8C42;
    pub const GL_SRGB8_ALPHA8                   : GLenum = 0x8C43;
    pub const GL_COMPRESSED_SRGB                : GLenum = 0x8C48;
    pub const GL_COMPRESSED_SRGB_ALPHA          : GLenum = 0x8C49;
}

#[cfg(version_3_0)]
pub mod version_3_0 {
    pub const GL_COMPARE_REF_TO_TEXTURE         : GLenum = 0x884E;
    pub const GL_CLIP_DISTANCE0                 : GLenum = 0x3000;
    pub const GL_CLIP_DISTANCE1                 : GLenum = 0x3001;
    pub const GL_CLIP_DISTANCE2                 : GLenum = 0x3002;
    pub const GL_CLIP_DISTANCE3                 : GLenum = 0x3003;
    pub const GL_CLIP_DISTANCE4                 : GLenum = 0x3004;
    pub const GL_CLIP_DISTANCE5                 : GLenum = 0x3005;
    pub const GL_CLIP_DISTANCE6                 : GLenum = 0x3006;
    pub const GL_CLIP_DISTANCE7                 : GLenum = 0x3007;
    pub const GL_MAX_CLIP_DISTANCES             : GLenum = 0x0D32;
    pub const GL_MAJOR_VERSION                  : GLenum = 0x821B;
    pub const GL_MINOR_VERSION                  : GLenum = 0x821C;
    pub const GL_NUM_EXTENSIONS                 : GLenum = 0x821D;
    pub const GL_CONTEXT_FLAGS                  : GLenum = 0x821E;
    pub const GL_COMPRESSED_RED                 : GLenum = 0x8225;
    pub const GL_COMPRESSED_RG                  : GLenum = 0x8226;
    pub const GL_CONTEXT_FLAG_FORWARD_COMPATIBLE_BIT : GLenum = 0x0001;
    pub const GL_RGBA32F                        : GLenum = 0x8814;
    pub const GL_RGB32F                         : GLenum = 0x8815;
    pub const GL_RGBA16F                        : GLenum = 0x881A;
    pub const GL_RGB16F                         : GLenum = 0x881B;
    pub const GL_VERTEX_ATTRIB_ARRAY_INTEGER    : GLenum = 0x88FD;
    pub const GL_MAX_ARRAY_TEXTURE_LAYERS       : GLenum = 0x88FF;
    pub const GL_MIN_PROGRAM_TEXEL_OFFSET       : GLenum = 0x8904;
    pub const GL_MAX_PROGRAM_TEXEL_OFFSET       : GLenum = 0x8905;
    pub const GL_CLAMP_READ_COLOR               : GLenum = 0x891C;
    pub const GL_FIXED_ONLY                     : GLenum = 0x891D;
    pub const GL_MAX_VARYING_COMPONENTS         : GLenum = 0x8B4B;
    pub const GL_TEXTURE_1D_ARRAY               : GLenum = 0x8C18;
    pub const GL_PROXY_TEXTURE_1D_ARRAY         : GLenum = 0x8C19;
    pub const GL_TEXTURE_2D_ARRAY               : GLenum = 0x8C1A;
    pub const GL_PROXY_TEXTURE_2D_ARRAY         : GLenum = 0x8C1B;
    pub const GL_TEXTURE_BINDING_1D_ARRAY       : GLenum = 0x8C1C;
    pub const GL_TEXTURE_BINDING_2D_ARRAY       : GLenum = 0x8C1D;
    pub const GL_R11F_G11F_B10F                 : GLenum = 0x8C3A;
    pub const GL_UNSIGNED_INT_10F_11F_11F_REV   : GLenum = 0x8C3B;
    pub const GL_RGB9_E5                        : GLenum = 0x8C3D;
    pub const GL_UNSIGNED_INT_5_9_9_9_REV       : GLenum = 0x8C3E;
    pub const GL_TEXTURE_SHARED_SIZE            : GLenum = 0x8C3F;
    pub const GL_TRANSFORM_FEEDBACK_VARYING_MAX_LENGTH : GLenum = 0x8C76;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_MODE : GLenum = 0x8C7F;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_COMPONENTS : GLenum = 0x8C80;
    pub const GL_TRANSFORM_FEEDBACK_VARYINGS    : GLenum = 0x8C83;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_START : GLenum = 0x8C84;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_SIZE : GLenum = 0x8C85;
    pub const GL_PRIMITIVES_GENERATED           : GLenum = 0x8C87;
    pub const GL_TRANSFORM_FEEDBACK_PRIMITIVES_WRITTEN : GLenum = 0x8C88;
    pub const GL_RASTERIZER_DISCARD             : GLenum = 0x8C89;
    pub const GL_MAX_TRANSFORM_FEEDBACK_INTERLEAVED_COMPONENTS : GLenum = 0x8C8A;
    pub const GL_MAX_TRANSFORM_FEEDBACK_SEPARATE_ATTRIBS : GLenum = 0x8C8B;
    pub const GL_INTERLEAVED_ATTRIBS            : GLenum = 0x8C8C;
    pub const GL_SEPARATE_ATTRIBS               : GLenum = 0x8C8D;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER      : GLenum = 0x8C8E;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_BINDING : GLenum = 0x8C8F;
    pub const GL_RGBA32UI                       : GLenum = 0x8D70;
    pub const GL_RGB32UI                        : GLenum = 0x8D71;
    pub const GL_RGBA16UI                       : GLenum = 0x8D76;
    pub const GL_RGB16UI                        : GLenum = 0x8D77;
    pub const GL_RGBA8UI                        : GLenum = 0x8D7C;
    pub const GL_RGB8UI                         : GLenum = 0x8D7D;
    pub const GL_RGBA32I                        : GLenum = 0x8D82;
    pub const GL_RGB32I                         : GLenum = 0x8D83;
    pub const GL_RGBA16I                        : GLenum = 0x8D88;
    pub const GL_RGB16I                         : GLenum = 0x8D89;
    pub const GL_RGBA8I                         : GLenum = 0x8D8E;
    pub const GL_RGB8I                          : GLenum = 0x8D8F;
    pub const GL_RED_INTEGER                    : GLenum = 0x8D94;
    pub const GL_GREEN_INTEGER                  : GLenum = 0x8D95;
    pub const GL_BLUE_INTEGER                   : GLenum = 0x8D96;
    pub const GL_RGB_INTEGER                    : GLenum = 0x8D98;
    pub const GL_RGBA_INTEGER                   : GLenum = 0x8D99;
    pub const GL_BGR_INTEGER                    : GLenum = 0x8D9A;
    pub const GL_BGRA_INTEGER                   : GLenum = 0x8D9B;
    pub const GL_SAMPLER_1D_ARRAY               : GLenum = 0x8DC0;
    pub const GL_SAMPLER_2D_ARRAY               : GLenum = 0x8DC1;
    pub const GL_SAMPLER_1D_ARRAY_SHADOW        : GLenum = 0x8DC3;
    pub const GL_SAMPLER_2D_ARRAY_SHADOW        : GLenum = 0x8DC4;
    pub const GL_SAMPLER_CUBE_SHADOW            : GLenum = 0x8DC5;
    pub const GL_UNSIGNED_INT_VEC2              : GLenum = 0x8DC6;
    pub const GL_UNSIGNED_INT_VEC3              : GLenum = 0x8DC7;
    pub const GL_UNSIGNED_INT_VEC4              : GLenum = 0x8DC8;
    pub const GL_INT_SAMPLER_1D                 : GLenum = 0x8DC9;
    pub const GL_INT_SAMPLER_2D                 : GLenum = 0x8DCA;
    pub const GL_INT_SAMPLER_3D                 : GLenum = 0x8DCB;
    pub const GL_INT_SAMPLER_CUBE               : GLenum = 0x8DCC;
    pub const GL_INT_SAMPLER_1D_ARRAY           : GLenum = 0x8DCE;
    pub const GL_INT_SAMPLER_2D_ARRAY           : GLenum = 0x8DCF;
    pub const GL_UNSIGNED_INT_SAMPLER_1D        : GLenum = 0x8DD1;
    pub const GL_UNSIGNED_INT_SAMPLER_2D        : GLenum = 0x8DD2;
    pub const GL_UNSIGNED_INT_SAMPLER_3D        : GLenum = 0x8DD3;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE      : GLenum = 0x8DD4;
    pub const GL_UNSIGNED_INT_SAMPLER_1D_ARRAY  : GLenum = 0x8DD6;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_ARRAY  : GLenum = 0x8DD7;
    pub const GL_QUERY_WAIT                     : GLenum = 0x8E13;
    pub const GL_QUERY_NO_WAIT                  : GLenum = 0x8E14;
    pub const GL_QUERY_BY_REGION_WAIT           : GLenum = 0x8E15;
    pub const GL_QUERY_BY_REGION_NO_WAIT        : GLenum = 0x8E16;
    pub const GL_BUFFER_ACCESS_FLAGS            : GLenum = 0x911F;
    pub const GL_BUFFER_MAP_LENGTH              : GLenum = 0x9120;
    pub const GL_BUFFER_MAP_OFFSET              : GLenum = 0x9121;
    
    /* Reuse tokens from ARB_depth_buffer_float */
    /* reuse GL_DEPTH_COMPONENT32F */
    /* reuse GL_DEPTH32F_STENCIL8 */
    /* reuse GL_FLOAT_32_UNSIGNED_INT_24_8_REV */
    /* Reuse tokens from ARB_framebuffer_object */
    /* reuse GL_INVALID_FRAMEBUFFER_OPERATION */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE */
    /* reuse GL_FRAMEBUFFER_DEFAULT */
    /* reuse GL_FRAMEBUFFER_UNDEFINED */
    /* reuse GL_DEPTH_STENCIL_ATTACHMENT */
    /* reuse GL_INDEX */
    /* reuse GL_MAX_RENDERBUFFER_SIZE */
    /* reuse GL_DEPTH_STENCIL */
    /* reuse GL_UNSIGNED_INT_24_8 */
    /* reuse GL_DEPTH24_STENCIL8 */
    /* reuse GL_TEXTURE_STENCIL_SIZE */
    /* reuse GL_TEXTURE_RED_TYPE */
    /* reuse GL_TEXTURE_GREEN_TYPE */
    /* reuse GL_TEXTURE_BLUE_TYPE */
    /* reuse GL_TEXTURE_ALPHA_TYPE */
    /* reuse GL_TEXTURE_DEPTH_TYPE */
    /* reuse GL_UNSIGNED_NORMALIZED */
    /* reuse GL_FRAMEBUFFER_BINDING */
    /* reuse GL_DRAW_FRAMEBUFFER_BINDING */
    /* reuse GL_RENDERBUFFER_BINDING */
    /* reuse GL_READ_FRAMEBUFFER */
    /* reuse GL_DRAW_FRAMEBUFFER */
    /* reuse GL_READ_FRAMEBUFFER_BINDING */
    /* reuse GL_RENDERBUFFER_SAMPLES */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER */
    /* reuse GL_FRAMEBUFFER_COMPLETE */
    /* reuse GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT */
    /* reuse GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT */
    /* reuse GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER */
    /* reuse GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER */
    /* reuse GL_FRAMEBUFFER_UNSUPPORTED */
    /* reuse GL_MAX_COLOR_ATTACHMENTS */
    /* reuse GL_COLOR_ATTACHMENT0 */
    /* reuse GL_COLOR_ATTACHMENT1 */
    /* reuse GL_COLOR_ATTACHMENT2 */
    /* reuse GL_COLOR_ATTACHMENT3 */
    /* reuse GL_COLOR_ATTACHMENT4 */
    /* reuse GL_COLOR_ATTACHMENT5 */
    /* reuse GL_COLOR_ATTACHMENT6 */
    /* reuse GL_COLOR_ATTACHMENT7 */
    /* reuse GL_COLOR_ATTACHMENT8 */
    /* reuse GL_COLOR_ATTACHMENT9 */
    /* reuse GL_COLOR_ATTACHMENT10 */
    /* reuse GL_COLOR_ATTACHMENT11 */
    /* reuse GL_COLOR_ATTACHMENT12 */
    /* reuse GL_COLOR_ATTACHMENT13 */
    /* reuse GL_COLOR_ATTACHMENT14 */
    /* reuse GL_COLOR_ATTACHMENT15 */
    /* reuse GL_DEPTH_ATTACHMENT */
    /* reuse GL_STENCIL_ATTACHMENT */
    /* reuse GL_FRAMEBUFFER */
    /* reuse GL_RENDERBUFFER */
    /* reuse GL_RENDERBUFFER_WIDTH */
    /* reuse GL_RENDERBUFFER_HEIGHT */
    /* reuse GL_RENDERBUFFER_INTERNAL_FORMAT */
    /* reuse GL_STENCIL_INDEX1 */
    /* reuse GL_STENCIL_INDEX4 */
    /* reuse GL_STENCIL_INDEX8 */
    /* reuse GL_STENCIL_INDEX16 */
    /* reuse GL_RENDERBUFFER_RED_SIZE */
    /* reuse GL_RENDERBUFFER_GREEN_SIZE */
    /* reuse GL_RENDERBUFFER_BLUE_SIZE */
    /* reuse GL_RENDERBUFFER_ALPHA_SIZE */
    /* reuse GL_RENDERBUFFER_DEPTH_SIZE */
    /* reuse GL_RENDERBUFFER_STENCIL_SIZE */
    /* reuse GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE */
    /* reuse GL_MAX_SAMPLES */
    /* Reuse tokens from ARB_framebuffer_sRGB */
    /* reuse GL_FRAMEBUFFER_SRGB */
    /* Reuse tokens from ARB_half_float_vertex */
    /* reuse GL_HALF_FLOAT */
    /* Reuse tokens from ARB_map_buffer_range */
    /* reuse GL_MAP_READ_BIT */
    /* reuse GL_MAP_WRITE_BIT */
    /* reuse GL_MAP_INVALIDATE_RANGE_BIT */
    /* reuse GL_MAP_INVALIDATE_BUFFER_BIT */
    /* reuse GL_MAP_FLUSH_EXPLICIT_BIT */
    /* reuse GL_MAP_UNSYNCHRONIZED_BIT */
    /* Reuse tokens from ARB_texture_compression_rgtc */
    /* reuse GL_COMPRESSED_RED_RGTC1 */
    /* reuse GL_COMPRESSED_SIGNED_RED_RGTC1 */
    /* reuse GL_COMPRESSED_RG_RGTC2 */
    /* reuse GL_COMPRESSED_SIGNED_RG_RGTC2 */
    /* Reuse tokens from ARB_texture_rg */
    /* reuse GL_RG */
    /* reuse GL_RG_INTEGER */
    /* reuse GL_R8 */
    /* reuse GL_R16 */
    /* reuse GL_RG8 */
    /* reuse GL_RG16 */
    /* reuse GL_R16F */
    /* reuse GL_R32F */
    /* reuse GL_RG16F */
    /* reuse GL_RG32F */
    /* reuse GL_R8I */
    /* reuse GL_R8UI */
    /* reuse GL_R16I */
    /* reuse GL_R16UI */
    /* reuse GL_R32I */
    /* reuse GL_R32UI */
    /* reuse GL_RG8I */
    /* reuse GL_RG8UI */
    /* reuse GL_RG16I */
    /* reuse GL_RG16UI */
    /* reuse GL_RG32I */
    /* reuse GL_RG32UI */
    /* Reuse tokens from ARB_vertex_array_object */
    /* reuse GL_VERTEX_ARRAY_BINDING */
}

#[cfg(version_3_1)]
pub mod version_3_1 {
    pub const GL_SAMPLER_2D_RECT                : GLenum = 0x8B63;
    pub const GL_SAMPLER_2D_RECT_SHADOW         : GLenum = 0x8B64;
    pub const GL_SAMPLER_BUFFER                 : GLenum = 0x8DC2;
    pub const GL_INT_SAMPLER_2D_RECT            : GLenum = 0x8DCD;
    pub const GL_INT_SAMPLER_BUFFER             : GLenum = 0x8DD0;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_RECT   : GLenum = 0x8DD5;
    pub const GL_UNSIGNED_INT_SAMPLER_BUFFER    : GLenum = 0x8DD8;
    pub const GL_TEXTURE_BUFFER                 : GLenum = 0x8C2A;
    pub const GL_MAX_TEXTURE_BUFFER_SIZE        : GLenum = 0x8C2B;
    pub const GL_TEXTURE_BINDING_BUFFER         : GLenum = 0x8C2C;
    pub const GL_TEXTURE_BUFFER_DATA_STORE_BINDING : GLenum = 0x8C2D;
    pub const GL_TEXTURE_RECTANGLE              : GLenum = 0x84F5;
    pub const GL_TEXTURE_BINDING_RECTANGLE      : GLenum = 0x84F6;
    pub const GL_PROXY_TEXTURE_RECTANGLE        : GLenum = 0x84F7;
    pub const GL_MAX_RECTANGLE_TEXTURE_SIZE     : GLenum = 0x84F8;
    pub const GL_RED_SNORM                      : GLenum = 0x8F90;
    pub const GL_RG_SNORM                       : GLenum = 0x8F91;
    pub const GL_RGB_SNORM                      : GLenum = 0x8F92;
    pub const GL_RGBA_SNORM                     : GLenum = 0x8F93;
    pub const GL_R8_SNORM                       : GLenum = 0x8F94;
    pub const GL_RG8_SNORM                      : GLenum = 0x8F95;
    pub const GL_RGB8_SNORM                     : GLenum = 0x8F96;
    pub const GL_RGBA8_SNORM                    : GLenum = 0x8F97;
    pub const GL_R16_SNORM                      : GLenum = 0x8F98;
    pub const GL_RG16_SNORM                     : GLenum = 0x8F99;
    pub const GL_RGB16_SNORM                    : GLenum = 0x8F9A;
    pub const GL_RGBA16_SNORM                   : GLenum = 0x8F9B;
    pub const GL_SIGNED_NORMALIZED              : GLenum = 0x8F9C;
    pub const GL_PRIMITIVE_RESTART              : GLenum = 0x8F9D;
    pub const GL_PRIMITIVE_RESTART_INDEX        : GLenum = 0x8F9E;
    /* Reuse tokens from ARB_copy_buffer */
    /* reuse GL_COPY_READ_BUFFER */
    /* reuse GL_COPY_WRITE_BUFFER */
    /* Reuse tokens from ARB_draw_instanced (none) */
    /* Reuse tokens from ARB_uniform_buffer_object */
    /* reuse GL_UNIFORM_BUFFER */
    /* reuse GL_UNIFORM_BUFFER_BINDING */
    /* reuse GL_UNIFORM_BUFFER_START */
    /* reuse GL_UNIFORM_BUFFER_SIZE */
    /* reuse GL_MAX_VERTEX_UNIFORM_BLOCKS */
    /* reuse GL_MAX_FRAGMENT_UNIFORM_BLOCKS */
    /* reuse GL_MAX_COMBINED_UNIFORM_BLOCKS */
    /* reuse GL_MAX_UNIFORM_BUFFER_BINDINGS */
    /* reuse GL_MAX_UNIFORM_BLOCK_SIZE */
    /* reuse GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS */
    /* reuse GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS */
    /* reuse GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT */
    /* reuse GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH */
    /* reuse GL_ACTIVE_UNIFORM_BLOCKS */
    /* reuse GL_UNIFORM_TYPE */
    /* reuse GL_UNIFORM_SIZE */
    /* reuse GL_UNIFORM_NAME_LENGTH */
    /* reuse GL_UNIFORM_BLOCK_INDEX */
    /* reuse GL_UNIFORM_OFFSET */
    /* reuse GL_UNIFORM_ARRAY_STRIDE */
    /* reuse GL_UNIFORM_MATRIX_STRIDE */
    /* reuse GL_UNIFORM_IS_ROW_MAJOR */
    /* reuse GL_UNIFORM_BLOCK_BINDING */
    /* reuse GL_UNIFORM_BLOCK_DATA_SIZE */
    /* reuse GL_UNIFORM_BLOCK_NAME_LENGTH */
    /* reuse GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS */
    /* reuse GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES */
    /* reuse GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER */
    /* reuse GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER */
    /* reuse GL_INVALID_INDEX */
}

#[cfg(version_3_2)]
pub mod version_3_2 {
    pub const GL_CONTEXT_CORE_PROFILE_BIT       : GLenum = 0x00000001;
    pub const GL_CONTEXT_COMPATIBILITY_PROFILE_BIT : GLenum = 0x00000002;
    pub const GL_LINES_ADJACENCY                : GLenum = 0x000A;
    pub const GL_LINE_STRIP_ADJACENCY           : GLenum = 0x000B;
    pub const GL_TRIANGLES_ADJACENCY            : GLenum = 0x000C;
    pub const GL_TRIANGLE_STRIP_ADJACENCY       : GLenum = 0x000D;
    pub const GL_PROGRAM_POINT_SIZE             : GLenum = 0x8642;
    pub const GL_MAX_GEOMETRY_TEXTURE_IMAGE_UNITS : GLenum = 0x8C29;
    pub const GL_FRAMEBUFFER_ATTACHMENT_LAYERED : GLenum = 0x8DA7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS : GLenum = 0x8DA8;
    pub const GL_GEOMETRY_SHADER                : GLenum = 0x8DD9;
    pub const GL_GEOMETRY_VERTICES_OUT          : GLenum = 0x8916;
    pub const GL_GEOMETRY_INPUT_TYPE            : GLenum = 0x8917;
    pub const GL_GEOMETRY_OUTPUT_TYPE           : GLenum = 0x8918;
    pub const GL_MAX_GEOMETRY_UNIFORM_COMPONENTS : GLenum = 0x8DDF;
    pub const GL_MAX_GEOMETRY_OUTPUT_VERTICES   : GLenum = 0x8DE0;
    pub const GL_MAX_GEOMETRY_TOTAL_OUTPUT_COMPONENTS : GLenum = 0x8DE1;
    pub const GL_MAX_VERTEX_OUTPUT_COMPONENTS   : GLenum = 0x9122;
    pub const GL_MAX_GEOMETRY_INPUT_COMPONENTS  : GLenum = 0x9123;
    pub const GL_MAX_GEOMETRY_OUTPUT_COMPONENTS : GLenum = 0x9124;
    pub const GL_MAX_FRAGMENT_INPUT_COMPONENTS  : GLenum = 0x9125;
    pub const GL_CONTEXT_PROFILE_MASK           : GLenum = 0x9126;
    /* reuse GL_MAX_VARYING_COMPONENTS */
    /* reuse GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER */
    /* Reuse tokens from ARB_depth_clamp */
    /* reuse GL_DEPTH_CLAMP */
    /* Reuse tokens from ARB_draw_elements_base_vertex (none) */
    /* Reuse tokens from ARB_fragment_coord_conventions (none) */
    /* Reuse tokens from ARB_provoking_vertex */
    /* reuse GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION */
    /* reuse GL_FIRST_VERTEX_CONVENTION */
    /* reuse GL_LAST_VERTEX_CONVENTION */
    /* reuse GL_PROVOKING_VERTEX */
    /* Reuse tokens from ARB_seamless_cube_map */
    /* reuse GL_TEXTURE_CUBE_MAP_SEAMLESS */
    /* Reuse tokens from ARB_sync */
    /* reuse GL_MAX_SERVER_WAIT_TIMEOUT */
    /* reuse GL_OBJECT_TYPE */
    /* reuse GL_SYNC_CONDITION */
    /* reuse GL_SYNC_STATUS */
    /* reuse GL_SYNC_FLAGS */
    /* reuse GL_SYNC_FENCE */
    /* reuse GL_SYNC_GPU_COMMANDS_COMPLETE */
    /* reuse GL_UNSIGNALED */
    /* reuse GL_SIGNALED */
    /* reuse GL_ALREADY_SIGNALED */
    /* reuse GL_TIMEOUT_EXPIRED */
    /* reuse GL_CONDITION_SATISFIED */
    /* reuse GL_WAIT_FAILED */
    /* reuse GL_TIMEOUT_IGNORED */
    /* reuse GL_SYNC_FLUSH_COMMANDS_BIT */
    /* reuse GL_TIMEOUT_IGNORED */
    /* Reuse tokens from ARB_texture_multisample */
    /* reuse GL_SAMPLE_POSITION */
    /* reuse GL_SAMPLE_MASK */
    /* reuse GL_SAMPLE_MASK_VALUE */
    /* reuse GL_MAX_SAMPLE_MASK_WORDS */
    /* reuse GL_TEXTURE_2D_MULTISAMPLE */
    /* reuse GL_PROXY_TEXTURE_2D_MULTISAMPLE */
    /* reuse GL_TEXTURE_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_TEXTURE_BINDING_2D_MULTISAMPLE */
    /* reuse GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_TEXTURE_SAMPLES */
    /* reuse GL_TEXTURE_FIXED_SAMPLE_LOCATIONS */
    /* reuse GL_SAMPLER_2D_MULTISAMPLE */
    /* reuse GL_INT_SAMPLER_2D_MULTISAMPLE */
    /* reuse GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE */
    /* reuse GL_SAMPLER_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_MAX_COLOR_TEXTURE_SAMPLES */
    /* reuse GL_MAX_DEPTH_TEXTURE_SAMPLES */
    /* reuse GL_MAX_INTEGER_SAMPLES */
/* Don't need to reuse tokens from ARB_vertex_array_bgra since they're already in 1.2 core */
}

#[cfg(version_3_3)]
pub mod version_3_3 {
    pub const GL_VERTEX_ATTRIB_ARRAY_DIVISOR    : GLenum = 0x88FE;
    /* Reuse tokens from ARB_blend_func_extended */
    /* reuse GL_SRC1_COLOR */
    /* reuse GL_ONE_MINUS_SRC1_COLOR */
    /* reuse GL_ONE_MINUS_SRC1_ALPHA */
    /* reuse GL_MAX_DUAL_SOURCE_DRAW_BUFFERS */
    /* Reuse tokens from ARB_explicit_attrib_location (none) */
    /* Reuse tokens from ARB_occlusion_query2 */
    /* reuse GL_ANY_SAMPLES_PASSED */
    /* Reuse tokens from ARB_sampler_objects */
    /* reuse GL_SAMPLER_BINDING */
    /* Reuse tokens from ARB_shader_bit_encoding (none) */
    /* Reuse tokens from ARB_texture_rgb10_a2ui */
    /* reuse GL_RGB10_A2UI */
    /* Reuse tokens from ARB_texture_swizzle */
    /* reuse GL_TEXTURE_SWIZZLE_R */
    /* reuse GL_TEXTURE_SWIZZLE_G */
    /* reuse GL_TEXTURE_SWIZZLE_B */
    /* reuse GL_TEXTURE_SWIZZLE_A */
    /* reuse GL_TEXTURE_SWIZZLE_RGBA */
    /* Reuse tokens from ARB_timer_query */
    /* reuse GL_TIME_ELAPSED */
    /* reuse GL_TIMESTAMP */
    /* Reuse tokens from ARB_vertex_type_2_10_10_10_rev */
    /* reuse GL_INT_2_10_10_10_REV */
}

#[cfg(version_4_0)]
pub mod version_4_0 {
    pub const GL_SAMPLE_SHADING                 : GLenum = 0x8C36;
    pub const GL_MIN_SAMPLE_SHADING_VALUE       : GLenum = 0x8C37;
    pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET : GLenum = 0x8E5E;
    pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET : GLenum = 0x8E5F;
    pub const GL_TEXTURE_CUBE_MAP_ARRAY         : GLenum = 0x9009;
    pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY : GLenum = 0x900A;
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY   : GLenum = 0x900B;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY         : GLenum = 0x900C;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW  : GLenum = 0x900D;
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY     : GLenum = 0x900E;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY : GLenum = 0x900F;
    /* Reuse tokens from ARB_texture_query_lod (none) */
    /* Reuse tokens from ARB_draw_buffers_blend (none) */
    /* Reuse tokens from ARB_draw_indirect */
    /* reuse GL_DRAW_INDIRECT_BUFFER */
    /* reuse GL_DRAW_INDIRECT_BUFFER_BINDING */
    /* Reuse tokens from ARB_gpu_shader5 */
    /* reuse GL_GEOMETRY_SHADER_INVOCATIONS */
    /* reuse GL_MAX_GEOMETRY_SHADER_INVOCATIONS */
    /* reuse GL_MIN_FRAGMENT_INTERPOLATION_OFFSET */
    /* reuse GL_MAX_FRAGMENT_INTERPOLATION_OFFSET */
    /* reuse GL_FRAGMENT_INTERPOLATION_OFFSET_BITS */
    /* reuse GL_MAX_VERTEX_STREAMS */
    /* Reuse tokens from ARB_gpu_shader_fp64 */
    /* reuse GL_DOUBLE_VEC2 */
    /* reuse GL_DOUBLE_VEC3 */
    /* reuse GL_DOUBLE_VEC4 */
    /* reuse GL_DOUBLE_MAT2 */
    /* reuse GL_DOUBLE_MAT3 */
    /* reuse GL_DOUBLE_MAT4 */
    /* reuse GL_DOUBLE_MAT2x3 */
    /* reuse GL_DOUBLE_MAT2x4 */
    /* reuse GL_DOUBLE_MAT3x2 */
    /* reuse GL_DOUBLE_MAT3x4 */
    /* reuse GL_DOUBLE_MAT4x2 */
    /* reuse GL_DOUBLE_MAT4x3 */
    /* Reuse tokens from ARB_shader_subroutine */
    /* reuse GL_ACTIVE_SUBROUTINES */
    /* reuse GL_ACTIVE_SUBROUTINE_UNIFORMS */
    /* reuse GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS */
    /* reuse GL_ACTIVE_SUBROUTINE_MAX_LENGTH */
    /* reuse GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH */
    /* reuse GL_MAX_SUBROUTINES */
    /* reuse GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS */
    /* reuse GL_NUM_COMPATIBLE_SUBROUTINES */
    /* reuse GL_COMPATIBLE_SUBROUTINES */
    /* Reuse tokens from ARB_tessellation_shader */
    /* reuse GL_PATCHES */
    /* reuse GL_PATCH_VERTICES */
    /* reuse GL_PATCH_DEFAULT_INNER_LEVEL */
    /* reuse GL_PATCH_DEFAULT_OUTER_LEVEL */
    /* reuse GL_TESS_CONTROL_OUTPUT_VERTICES */
    /* reuse GL_TESS_GEN_MODE */
    /* reuse GL_TESS_GEN_SPACING */
    /* reuse GL_TESS_GEN_VERTEX_ORDER */
    /* reuse GL_TESS_GEN_POINT_MODE */
    /* reuse GL_ISOLINES */
    /* reuse GL_FRACTIONAL_ODD */
    /* reuse GL_FRACTIONAL_EVEN */
    /* reuse GL_MAX_PATCH_VERTICES */
    /* reuse GL_MAX_TESS_GEN_LEVEL */
    /* reuse GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS */
    /* reuse GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS */
    /* reuse GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS */
    /* reuse GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS */
    /* reuse GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS */
    /* reuse GL_MAX_TESS_PATCH_COMPONENTS */
    /* reuse GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS */
    /* reuse GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS */
    /* reuse GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS */
    /* reuse GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS */
    /* reuse GL_MAX_TESS_CONTROL_INPUT_COMPONENTS */
    /* reuse GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS */
    /* reuse GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS */
    /* reuse GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS */
    /* reuse GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER */
    /* reuse GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER */
    /* reuse GL_TESS_EVALUATION_SHADER */
    /* reuse GL_TESS_CONTROL_SHADER */
    /* Reuse tokens from ARB_texture_buffer_object_rgb32 (none) */
    /* Reuse tokens from ARB_transform_feedback2 */
    /* reuse GL_TRANSFORM_FEEDBACK */
    /* reuse GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED */
    /* reuse GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE */
    /* reuse GL_TRANSFORM_FEEDBACK_BINDING */
    /* Reuse tokens from ARB_transform_feedback3 */
    /* reuse GL_MAX_TRANSFORM_FEEDBACK_BUFFERS */
    /* reuse GL_MAX_VERTEX_STREAMS */
}

#[cfg(version_4_1)]
pub mod version_4_1 {
    /* Reuse tokens from ARB_ES2_compatibility */
    /* reuse GL_FIXED */
    /* reuse GL_IMPLEMENTATION_COLOR_READ_TYPE */
    /* reuse GL_IMPLEMENTATION_COLOR_READ_FORMAT */
    /* reuse GL_LOW_FLOAT */
    /* reuse GL_MEDIUM_FLOAT */
    /* reuse GL_HIGH_FLOAT */
    /* reuse GL_LOW_INT */
    /* reuse GL_MEDIUM_INT */
    /* reuse GL_HIGH_INT */
    /* reuse GL_SHADER_COMPILER */
    /* reuse GL_SHADER_BINARY_FORMATS */
    /* reuse GL_NUM_SHADER_BINARY_FORMATS */
    /* reuse GL_MAX_VERTEX_UNIFORM_VECTORS */
    /* reuse GL_MAX_VARYING_VECTORS */
    /* reuse GL_MAX_FRAGMENT_UNIFORM_VECTORS */
    /* reuse GL_RGB565 */
    /* Reuse tokens from ARB_get_program_binary */
    /* reuse GL_PROGRAM_BINARY_RETRIEVABLE_HINT */
    /* reuse GL_PROGRAM_BINARY_LENGTH */
    /* reuse GL_NUM_PROGRAM_BINARY_FORMATS */
    /* reuse GL_PROGRAM_BINARY_FORMATS */
    /* Reuse tokens from ARB_separate_shader_objects */
    /* reuse GL_VERTEX_SHADER_BIT */
    /* reuse GL_FRAGMENT_SHADER_BIT */
    /* reuse GL_GEOMETRY_SHADER_BIT */
    /* reuse GL_TESS_CONTROL_SHADER_BIT */
    /* reuse GL_TESS_EVALUATION_SHADER_BIT */
    /* reuse GL_ALL_SHADER_BITS */
    /* reuse GL_PROGRAM_SEPARABLE */
    /* reuse GL_ACTIVE_PROGRAM */
    /* reuse GL_PROGRAM_PIPELINE_BINDING */
    /* Reuse tokens from ARB_shader_precision (none) */
    /* Reuse tokens from ARB_vertex_attrib_64bit - all are in GL 3.0 and 4.0 already */
    /* Reuse tokens from ARB_viewport_array - some are in GL 1.1 and ARB_provoking_vertex already */
    /* reuse GL_MAX_VIEWPORTS */
    /* reuse GL_VIEWPORT_SUBPIXEL_BITS */
    /* reuse GL_VIEWPORT_BOUNDS_RANGE */
    /* reuse GL_LAYER_PROVOKING_VERTEX */
    /* reuse GL_VIEWPORT_INDEX_PROVOKING_VERTEX */
    /* reuse GL_UNDEFINED_VERTEX */
}

#[cfg(version_4_2)]
pub mod version_4_2 {
    /* Reuse tokens from ARB_base_instance (none) */
    /* Reuse tokens from ARB_shading_language_420pack (none) */
    /* Reuse tokens from ARB_transform_feedback_instanced (none) */
    /* Reuse tokens from ARB_compressed_texture_pixel_storage */
    /* reuse GL_UNPACK_COMPRESSED_BLOCK_WIDTH */
    /* reuse GL_UNPACK_COMPRESSED_BLOCK_HEIGHT */
    /* reuse GL_UNPACK_COMPRESSED_BLOCK_DEPTH */
    /* reuse GL_UNPACK_COMPRESSED_BLOCK_SIZE */
    /* reuse GL_PACK_COMPRESSED_BLOCK_WIDTH */
    /* reuse GL_PACK_COMPRESSED_BLOCK_HEIGHT */
    /* reuse GL_PACK_COMPRESSED_BLOCK_DEPTH */
    /* reuse GL_PACK_COMPRESSED_BLOCK_SIZE */
    /* Reuse tokens from ARB_conservative_depth (none) */
    /* Reuse tokens from ARB_internalformat_query */
    /* reuse GL_NUM_SAMPLE_COUNTS */
    /* Reuse tokens from ARB_map_buffer_alignment */
    /* reuse GL_MIN_MAP_BUFFER_ALIGNMENT */
    /* Reuse tokens from ARB_shader_atomic_counters */
    /* reuse GL_ATOMIC_COUNTER_BUFFER */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_BINDING */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_START */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_SIZE */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER */
    /* reuse GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_VERTEX_ATOMIC_COUNTERS */
    /* reuse GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS */
    /* reuse GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS */
    /* reuse GL_MAX_GEOMETRY_ATOMIC_COUNTERS */
    /* reuse GL_MAX_FRAGMENT_ATOMIC_COUNTERS */
    /* reuse GL_MAX_COMBINED_ATOMIC_COUNTERS */
    /* reuse GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE */
    /* reuse GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS */
    /* reuse GL_ACTIVE_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX */
    /* reuse GL_UNSIGNED_INT_ATOMIC_COUNTER */
    /* Reuse tokens from ARB_shader_image_load_store */
    /* reuse GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT */
    /* reuse GL_ELEMENT_ARRAY_BARRIER_BIT */
    /* reuse GL_UNIFORM_BARRIER_BIT */
    /* reuse GL_TEXTURE_FETCH_BARRIER_BIT */
    /* reuse GL_SHADER_IMAGE_ACCESS_BARRIER_BIT */
    /* reuse GL_COMMAND_BARRIER_BIT */
    /* reuse GL_PIXEL_BUFFER_BARRIER_BIT */
    /* reuse GL_TEXTURE_UPDATE_BARRIER_BIT */
    /* reuse GL_BUFFER_UPDATE_BARRIER_BIT */
    /* reuse GL_FRAMEBUFFER_BARRIER_BIT */
    /* reuse GL_TRANSFORM_FEEDBACK_BARRIER_BIT */
    /* reuse GL_ATOMIC_COUNTER_BARRIER_BIT */
    /* reuse GL_ALL_BARRIER_BITS */
    /* reuse GL_MAX_IMAGE_UNITS */
    /* reuse GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS */
    /* reuse GL_IMAGE_BINDING_NAME */
    /* reuse GL_IMAGE_BINDING_LEVEL */
    /* reuse GL_IMAGE_BINDING_LAYERED */
    /* reuse GL_IMAGE_BINDING_LAYER */
    /* reuse GL_IMAGE_BINDING_ACCESS */
    /* reuse GL_IMAGE_1D */
    /* reuse GL_IMAGE_2D */
    /* reuse GL_IMAGE_3D */
    /* reuse GL_IMAGE_2D_RECT */
    /* reuse GL_IMAGE_CUBE */
    /* reuse GL_IMAGE_BUFFER */
    /* reuse GL_IMAGE_1D_ARRAY */
    /* reuse GL_IMAGE_2D_ARRAY */
    /* reuse GL_IMAGE_CUBE_MAP_ARRAY */
    /* reuse GL_IMAGE_2D_MULTISAMPLE */
    /* reuse GL_IMAGE_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_INT_IMAGE_1D */
    /* reuse GL_INT_IMAGE_2D */
    /* reuse GL_INT_IMAGE_3D */
    /* reuse GL_INT_IMAGE_2D_RECT */
    /* reuse GL_INT_IMAGE_CUBE */
    /* reuse GL_INT_IMAGE_BUFFER */
    /* reuse GL_INT_IMAGE_1D_ARRAY */
    /* reuse GL_INT_IMAGE_2D_ARRAY */
    /* reuse GL_INT_IMAGE_CUBE_MAP_ARRAY */
    /* reuse GL_INT_IMAGE_2D_MULTISAMPLE */
    /* reuse GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_UNSIGNED_INT_IMAGE_1D */
    /* reuse GL_UNSIGNED_INT_IMAGE_2D */
    /* reuse GL_UNSIGNED_INT_IMAGE_3D */
    /* reuse GL_UNSIGNED_INT_IMAGE_2D_RECT */
    /* reuse GL_UNSIGNED_INT_IMAGE_CUBE */
    /* reuse GL_UNSIGNED_INT_IMAGE_BUFFER */
    /* reuse GL_UNSIGNED_INT_IMAGE_1D_ARRAY */
    /* reuse GL_UNSIGNED_INT_IMAGE_2D_ARRAY */
    /* reuse GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY */
    /* reuse GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE */
    /* reuse GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_MAX_IMAGE_SAMPLES */
    /* reuse GL_IMAGE_BINDING_FORMAT */
    /* reuse GL_IMAGE_FORMAT_COMPATIBILITY_TYPE */
    /* reuse GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE */
    /* reuse GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS */
    /* reuse GL_MAX_VERTEX_IMAGE_UNIFORMS */
    /* reuse GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS */
    /* reuse GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS */
    /* reuse GL_MAX_GEOMETRY_IMAGE_UNIFORMS */
    /* reuse GL_MAX_FRAGMENT_IMAGE_UNIFORMS */
    /* reuse GL_MAX_COMBINED_IMAGE_UNIFORMS */
    /* Reuse tokens from ARB_shading_language_packing (none) */
    /* Reuse tokens from ARB_texture_storage */
    /* reuse GL_TEXTURE_IMMUTABLE_FORMAT */
}

#[cfg(version_4_3)]
pub mod version_4_3 {
    pub const GL_NUM_SHADING_LANGUAGE_VERSIONS  : GLenum = 0x82E9;
    pub const GL_VERTEX_ATTRIB_ARRAY_LONG       : GLenum = 0x874E;
    /* Reuse tokens from ARB_arrays_of_arrays (none, GLSL only) */
    /* Reuse tokens from ARB_fragment_layer_viewport (none, GLSL only) */
    /* Reuse tokens from ARB_shader_image_size (none, GLSL only) */
    /* Reuse tokens from ARB_ES3_compatibility */
    /* reuse GL_COMPRESSED_RGB8_ETC2 */
    /* reuse GL_COMPRESSED_SRGB8_ETC2 */
    /* reuse GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 */
    /* reuse GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 */
    /* reuse GL_COMPRESSED_RGBA8_ETC2_EAC */
    /* reuse GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC */
    /* reuse GL_COMPRESSED_R11_EAC */
    /* reuse GL_COMPRESSED_SIGNED_R11_EAC */
    /* reuse GL_COMPRESSED_RG11_EAC */
    /* reuse GL_COMPRESSED_SIGNED_RG11_EAC */
    /* reuse GL_PRIMITIVE_RESTART_FIXED_INDEX */
    /* reuse GL_ANY_SAMPLES_PASSED_CONSERVATIVE */
    /* reuse GL_MAX_ELEMENT_INDEX */
    /* Reuse tokens from ARB_clear_buffer_object (none) */
    /* Reuse tokens from ARB_compute_shader */
    /* reuse GL_COMPUTE_SHADER */
    /* reuse GL_MAX_COMPUTE_UNIFORM_BLOCKS */
    /* reuse GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS */
    /* reuse GL_MAX_COMPUTE_IMAGE_UNIFORMS */
    /* reuse GL_MAX_COMPUTE_SHARED_MEMORY_SIZE */
    /* reuse GL_MAX_COMPUTE_UNIFORM_COMPONENTS */
    /* reuse GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS */
    /* reuse GL_MAX_COMPUTE_ATOMIC_COUNTERS */
    /* reuse GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS */
    /* reuse GL_MAX_COMPUTE_LOCAL_INVOCATIONS */
    /* reuse GL_MAX_COMPUTE_WORK_GROUP_COUNT */
    /* reuse GL_MAX_COMPUTE_WORK_GROUP_SIZE */
    /* reuse GL_COMPUTE_LOCAL_WORK_SIZE */
    /* reuse GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER */
    /* reuse GL_DISPATCH_INDIRECT_BUFFER */
    /* reuse GL_DISPATCH_INDIRECT_BUFFER_BINDING */
    /* Reuse tokens from ARB_copy_image (none) */
    /* Reuse tokens from KHR_debug */
    /* reuse GL_DEBUG_OUTPUT_SYNCHRONOUS */
    /* reuse GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH */
    /* reuse GL_DEBUG_CALLBACK_FUNCTION */
    /* reuse GL_DEBUG_CALLBACK_USER_PARAM */
    /* reuse GL_DEBUG_SOURCE_API */
    /* reuse GL_DEBUG_SOURCE_WINDOW_SYSTEM */
    /* reuse GL_DEBUG_SOURCE_SHADER_COMPILER */
    /* reuse GL_DEBUG_SOURCE_THIRD_PARTY */
    /* reuse GL_DEBUG_SOURCE_APPLICATION */
    /* reuse GL_DEBUG_SOURCE_OTHER */
    /* reuse GL_DEBUG_TYPE_ERROR */
    /* reuse GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR */
    /* reuse GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR */
    /* reuse GL_DEBUG_TYPE_PORTABILITY */
    /* reuse GL_DEBUG_TYPE_PERFORMANCE */
    /* reuse GL_DEBUG_TYPE_OTHER */
    /* reuse GL_MAX_DEBUG_MESSAGE_LENGTH */
    /* reuse GL_MAX_DEBUG_LOGGED_MESSAGES */
    /* reuse GL_DEBUG_LOGGED_MESSAGES */
    /* reuse GL_DEBUG_SEVERITY_HIGH */
    /* reuse GL_DEBUG_SEVERITY_MEDIUM */
    /* reuse GL_DEBUG_SEVERITY_LOW */
    /* reuse GL_DEBUG_TYPE_MARKER */
    /* reuse GL_DEBUG_TYPE_PUSH_GROUP */
    /* reuse GL_DEBUG_TYPE_POP_GROUP */
    /* reuse GL_DEBUG_SEVERITY_NOTIFICATION */
    /* reuse GL_MAX_DEBUG_GROUP_STACK_DEPTH */
    /* reuse GL_DEBUG_GROUP_STACK_DEPTH */
    /* reuse GL_BUFFER */
    /* reuse GL_SHADER */
    /* reuse GL_PROGRAM */
    /* reuse GL_QUERY */
    /* reuse GL_PROGRAM_PIPELINE */
    /* reuse GL_SAMPLER */
    /* reuse GL_DISPLAY_LIST */
    /* reuse GL_MAX_LABEL_LENGTH */
    /* reuse GL_DEBUG_OUTPUT */
    /* reuse GL_CONTEXT_FLAG_DEBUG_BIT */
    /* reuse GL_STACK_UNDERFLOW */
    /* reuse GL_STACK_OVERFLOW */
    /* Reuse tokens from ARB_explicit_uniform_location */
    /* reuse GL_MAX_UNIFORM_LOCATIONS */
    /* Reuse tokens from ARB_framebuffer_no_attachments */
    /* reuse GL_FRAMEBUFFER_DEFAULT_WIDTH */
    /* reuse GL_FRAMEBUFFER_DEFAULT_HEIGHT */
    /* reuse GL_FRAMEBUFFER_DEFAULT_LAYERS */
    /* reuse GL_FRAMEBUFFER_DEFAULT_SAMPLES */
    /* reuse GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS */
    /* reuse GL_MAX_FRAMEBUFFER_WIDTH */
    /* reuse GL_MAX_FRAMEBUFFER_HEIGHT */
    /* reuse GL_MAX_FRAMEBUFFER_LAYERS */
    /* reuse GL_MAX_FRAMEBUFFER_SAMPLES */
    /* Reuse tokens from ARB_internalformat_query2 */
    /* reuse GL_INTERNALFORMAT_SUPPORTED */
    /* reuse GL_INTERNALFORMAT_PREFERRED */
    /* reuse GL_INTERNALFORMAT_RED_SIZE */
    /* reuse GL_INTERNALFORMAT_GREEN_SIZE */
    /* reuse GL_INTERNALFORMAT_BLUE_SIZE */
    /* reuse GL_INTERNALFORMAT_ALPHA_SIZE */
    /* reuse GL_INTERNALFORMAT_DEPTH_SIZE */
    /* reuse GL_INTERNALFORMAT_STENCIL_SIZE */
    /* reuse GL_INTERNALFORMAT_SHARED_SIZE */
    /* reuse GL_INTERNALFORMAT_RED_TYPE */
    /* reuse GL_INTERNALFORMAT_GREEN_TYPE */
    /* reuse GL_INTERNALFORMAT_BLUE_TYPE */
    /* reuse GL_INTERNALFORMAT_ALPHA_TYPE */
    /* reuse GL_INTERNALFORMAT_DEPTH_TYPE */
    /* reuse GL_INTERNALFORMAT_STENCIL_TYPE */
    /* reuse GL_MAX_WIDTH */
    /* reuse GL_MAX_HEIGHT */
    /* reuse GL_MAX_DEPTH */
    /* reuse GL_MAX_LAYERS */
    /* reuse GL_MAX_COMBINED_DIMENSIONS */
    /* reuse GL_COLOR_COMPONENTS */
    /* reuse GL_DEPTH_COMPONENTS */
    /* reuse GL_STENCIL_COMPONENTS */
    /* reuse GL_COLOR_RENDERABLE */
    /* reuse GL_DEPTH_RENDERABLE */
    /* reuse GL_STENCIL_RENDERABLE */
    /* reuse GL_FRAMEBUFFER_RENDERABLE */
    /* reuse GL_FRAMEBUFFER_RENDERABLE_LAYERED */
    /* reuse GL_FRAMEBUFFER_BLEND */
    /* reuse GL_READ_PIXELS */
    /* reuse GL_READ_PIXELS_FORMAT */
    /* reuse GL_READ_PIXELS_TYPE */
    /* reuse GL_TEXTURE_IMAGE_FORMAT */
    /* reuse GL_TEXTURE_IMAGE_TYPE */
    /* reuse GL_GET_TEXTURE_IMAGE_FORMAT */
    /* reuse GL_GET_TEXTURE_IMAGE_TYPE */
    /* reuse GL_MIPMAP */
    /* reuse GL_MANUAL_GENERATE_MIPMAP */
    /* reuse GL_AUTO_GENERATE_MIPMAP */
    /* reuse GL_COLOR_ENCODING */
    /* reuse GL_SRGB_READ */
    /* reuse GL_SRGB_WRITE */
    /* reuse GL_FILTER */
    /* reuse GL_VERTEX_TEXTURE */
    /* reuse GL_TESS_CONTROL_TEXTURE */
    /* reuse GL_TESS_EVALUATION_TEXTURE */
    /* reuse GL_GEOMETRY_TEXTURE */
    /* reuse GL_FRAGMENT_TEXTURE */
    /* reuse GL_COMPUTE_TEXTURE */
    /* reuse GL_TEXTURE_SHADOW */
    /* reuse GL_TEXTURE_GATHER */
    /* reuse GL_TEXTURE_GATHER_SHADOW */
    /* reuse GL_SHADER_IMAGE_LOAD */
    /* reuse GL_SHADER_IMAGE_STORE */
    /* reuse GL_SHADER_IMAGE_ATOMIC */
    /* reuse GL_IMAGE_TEXEL_SIZE */
    /* reuse GL_IMAGE_COMPATIBILITY_CLASS */
    /* reuse GL_IMAGE_PIXEL_FORMAT */
    /* reuse GL_IMAGE_PIXEL_TYPE */
    /* reuse GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST */
    /* reuse GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST */
    /* reuse GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE */
    /* reuse GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE */
    /* reuse GL_TEXTURE_COMPRESSED_BLOCK_WIDTH */
    /* reuse GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT */
    /* reuse GL_TEXTURE_COMPRESSED_BLOCK_SIZE */
    /* reuse GL_CLEAR_BUFFER */
    /* reuse GL_TEXTURE_VIEW */
    /* reuse GL_VIEW_COMPATIBILITY_CLASS */
    /* reuse GL_FULL_SUPPORT */
    /* reuse GL_CAVEAT_SUPPORT */
    /* reuse GL_IMAGE_CLASS_4_X_32 */
    /* reuse GL_IMAGE_CLASS_2_X_32 */
    /* reuse GL_IMAGE_CLASS_1_X_32 */
    /* reuse GL_IMAGE_CLASS_4_X_16 */
    /* reuse GL_IMAGE_CLASS_2_X_16 */
    /* reuse GL_IMAGE_CLASS_1_X_16 */
    /* reuse GL_IMAGE_CLASS_4_X_8 */
    /* reuse GL_IMAGE_CLASS_2_X_8 */
    /* reuse GL_IMAGE_CLASS_1_X_8 */
    /* reuse GL_IMAGE_CLASS_11_11_10 */
    /* reuse GL_IMAGE_CLASS_10_10_10_2 */
    /* reuse GL_VIEW_CLASS_128_BITS */
    /* reuse GL_VIEW_CLASS_96_BITS */
    /* reuse GL_VIEW_CLASS_64_BITS */
    /* reuse GL_VIEW_CLASS_48_BITS */
    /* reuse GL_VIEW_CLASS_32_BITS */
    /* reuse GL_VIEW_CLASS_24_BITS */
    /* reuse GL_VIEW_CLASS_16_BITS */
    /* reuse GL_VIEW_CLASS_8_BITS */
    /* reuse GL_VIEW_CLASS_S3TC_DXT1_RGB */
    /* reuse GL_VIEW_CLASS_S3TC_DXT1_RGBA */
    /* reuse GL_VIEW_CLASS_S3TC_DXT3_RGBA */
    /* reuse GL_VIEW_CLASS_S3TC_DXT5_RGBA */
    /* reuse GL_VIEW_CLASS_RGTC1_RED */
    /* reuse GL_VIEW_CLASS_RGTC2_RG */
    /* reuse GL_VIEW_CLASS_BPTC_UNORM */
    /* reuse GL_VIEW_CLASS_BPTC_FLOAT */
    /* Reuse tokens from ARB_invalidate_subdata (none) */
    /* Reuse tokens from ARB_multi_draw_indirect (none) */
    /* Reuse tokens from ARB_program_interface_query */
    /* reuse GL_UNIFORM */
    /* reuse GL_UNIFORM_BLOCK */
    /* reuse GL_PROGRAM_INPUT */
    /* reuse GL_PROGRAM_OUTPUT */
    /* reuse GL_BUFFER_VARIABLE */
    /* reuse GL_SHADER_STORAGE_BLOCK */
    /* reuse GL_VERTEX_SUBROUTINE */
    /* reuse GL_TESS_CONTROL_SUBROUTINE */
    /* reuse GL_TESS_EVALUATION_SUBROUTINE */
    /* reuse GL_GEOMETRY_SUBROUTINE */
    /* reuse GL_FRAGMENT_SUBROUTINE */
    /* reuse GL_COMPUTE_SUBROUTINE */
    /* reuse GL_VERTEX_SUBROUTINE_UNIFORM */
    /* reuse GL_TESS_CONTROL_SUBROUTINE_UNIFORM */
    /* reuse GL_TESS_EVALUATION_SUBROUTINE_UNIFORM */
    /* reuse GL_GEOMETRY_SUBROUTINE_UNIFORM */
    /* reuse GL_FRAGMENT_SUBROUTINE_UNIFORM */
    /* reuse GL_COMPUTE_SUBROUTINE_UNIFORM */
    /* reuse GL_TRANSFORM_FEEDBACK_VARYING */
    /* reuse GL_ACTIVE_RESOURCES */
    /* reuse GL_MAX_NAME_LENGTH */
    /* reuse GL_MAX_NUM_ACTIVE_VARIABLES */
    /* reuse GL_MAX_NUM_COMPATIBLE_SUBROUTINES */
    /* reuse GL_NAME_LENGTH */
    /* reuse GL_TYPE */
    /* reuse GL_ARRAY_SIZE */
    /* reuse GL_OFFSET */
    /* reuse GL_BLOCK_INDEX */
    /* reuse GL_ARRAY_STRIDE */
    /* reuse GL_MATRIX_STRIDE */
    /* reuse GL_IS_ROW_MAJOR */
    /* reuse GL_ATOMIC_COUNTER_BUFFER_INDEX */
    /* reuse GL_BUFFER_BINDING */
    /* reuse GL_BUFFER_DATA_SIZE */
    /* reuse GL_NUM_ACTIVE_VARIABLES */
    /* reuse GL_ACTIVE_VARIABLES */
    /* reuse GL_REFERENCED_BY_VERTEX_SHADER */
    /* reuse GL_REFERENCED_BY_TESS_CONTROL_SHADER */
    /* reuse GL_REFERENCED_BY_TESS_EVALUATION_SHADER */
    /* reuse GL_REFERENCED_BY_GEOMETRY_SHADER */
    /* reuse GL_REFERENCED_BY_FRAGMENT_SHADER */
    /* reuse GL_REFERENCED_BY_COMPUTE_SHADER */
    /* reuse GL_TOP_LEVEL_ARRAY_SIZE */
    /* reuse GL_TOP_LEVEL_ARRAY_STRIDE */
    /* reuse GL_LOCATION */
    /* reuse GL_LOCATION_INDEX */
    /* reuse GL_IS_PER_PATCH */
    /* Reuse tokens from ARB_robust_buffer_access_behavior (none) */
    /* Reuse tokens from ARB_shader_storage_buffer_object */
    /* reuse GL_SHADER_STORAGE_BUFFER */
    /* reuse GL_SHADER_STORAGE_BUFFER_BINDING */
    /* reuse GL_SHADER_STORAGE_BUFFER_START */
    /* reuse GL_SHADER_STORAGE_BUFFER_SIZE */
    /* reuse GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS */
    /* reuse GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS */
    /* reuse GL_MAX_SHADER_STORAGE_BLOCK_SIZE */
    /* reuse GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT */
    /* reuse GL_SHADER_STORAGE_BARRIER_BIT */
    /* reuse GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES */
    /* Reuse tokens from ARB_stencil_texturing */
    /* reuse GL_DEPTH_STENCIL_TEXTURE_MODE */
    /* Reuse tokens from ARB_texture_buffer_range */
    /* reuse GL_TEXTURE_BUFFER_OFFSET */
    /* reuse GL_TEXTURE_BUFFER_SIZE */
    /* reuse GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT */
    /* Reuse tokens from ARB_texture_query_levels (none) */
    /* Reuse tokens from ARB_texture_storage_multisample (none) */
    /* Reuse tokens from ARB_texture_view */
    /* reuse GL_TEXTURE_VIEW_MIN_LEVEL */
    /* reuse GL_TEXTURE_VIEW_NUM_LEVELS */
    /* reuse GL_TEXTURE_VIEW_MIN_LAYER */
    /* reuse GL_TEXTURE_VIEW_NUM_LAYERS */
    /* reuse GL_TEXTURE_IMMUTABLE_LEVELS */
    /* Reuse tokens from ARB_vertex_attrib_binding */
    /* reuse GL_VERTEX_ATTRIB_BINDING */
    /* reuse GL_VERTEX_ATTRIB_RELATIVE_OFFSET */
    /* reuse GL_VERTEX_BINDING_DIVISOR */
    /* reuse GL_VERTEX_BINDING_OFFSET */
    /* reuse GL_VERTEX_BINDING_STRIDE */
    /* reuse GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET */
    /* reuse GL_MAX_VERTEX_ATTRIB_BINDINGS */
}

#[cfg(arb_depth_buffer_float)]
pub mod arb_depth_buffer_float {
    pub const GL_DEPTH_COMPONENT32F             : GLenum = 0x8CAC;
    pub const GL_DEPTH32F_STENCIL8              : GLenum = 0x8CAD;
    pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV : GLenum = 0x8DAD;
}

#[cfg(arb_framebuffer_object)]
pub mod arb_framebuffer_object {
    pub const GL_INVALID_FRAMEBUFFER_OPERATION  : GLenum = 0x0506;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COLOR_ENCODING : GLenum = 0x8210;
    pub const GL_FRAMEBUFFER_ATTACHMENT_COMPONENT_TYPE : GLenum = 0x8211;
    pub const GL_FRAMEBUFFER_ATTACHMENT_RED_SIZE : GLenum = 0x8212;
    pub const GL_FRAMEBUFFER_ATTACHMENT_GREEN_SIZE : GLenum = 0x8213;
    pub const GL_FRAMEBUFFER_ATTACHMENT_BLUE_SIZE : GLenum = 0x8214;
    pub const GL_FRAMEBUFFER_ATTACHMENT_ALPHA_SIZE : GLenum = 0x8215;
    pub const GL_FRAMEBUFFER_ATTACHMENT_DEPTH_SIZE : GLenum = 0x8216;
    pub const GL_FRAMEBUFFER_ATTACHMENT_STENCIL_SIZE : GLenum = 0x8217;
    pub const GL_FRAMEBUFFER_DEFAULT            : GLenum = 0x8218;
    pub const GL_FRAMEBUFFER_UNDEFINED          : GLenum = 0x8219;
    pub const GL_DEPTH_STENCIL_ATTACHMENT       : GLenum = 0x821A;
    pub const GL_MAX_RENDERBUFFER_SIZE          : GLenum = 0x84E8;
    pub const GL_DEPTH_STENCIL                  : GLenum = 0x84F9;
    pub const GL_UNSIGNED_INT_24_8              : GLenum = 0x84FA;
    pub const GL_DEPTH24_STENCIL8               : GLenum = 0x88F0;
    pub const GL_TEXTURE_STENCIL_SIZE           : GLenum = 0x88F1;
    pub const GL_TEXTURE_RED_TYPE               : GLenum = 0x8C10;
    pub const GL_TEXTURE_GREEN_TYPE             : GLenum = 0x8C11;
    pub const GL_TEXTURE_BLUE_TYPE              : GLenum = 0x8C12;
    pub const GL_TEXTURE_ALPHA_TYPE             : GLenum = 0x8C13;
    pub const GL_TEXTURE_DEPTH_TYPE             : GLenum = 0x8C16;
    pub const GL_UNSIGNED_NORMALIZED            : GLenum = 0x8C17;
    pub const GL_FRAMEBUFFER_BINDING            : GLenum = 0x8CA6;
    pub const GL_DRAW_FRAMEBUFFER_BINDING       : GLenum = GL_FRAMEBUFFER_BINDING;
    pub const GL_RENDERBUFFER_BINDING           : GLenum = 0x8CA7;
    pub const GL_READ_FRAMEBUFFER               : GLenum = 0x8CA8;
    pub const GL_DRAW_FRAMEBUFFER               : GLenum = 0x8CA9;
    pub const GL_READ_FRAMEBUFFER_BINDING       : GLenum = 0x8CAA;
    pub const GL_RENDERBUFFER_SAMPLES           : GLenum = 0x8CAB;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_TYPE : GLenum = 0x8CD0;
    pub const GL_FRAMEBUFFER_ATTACHMENT_OBJECT_NAME : GLenum = 0x8CD1;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LEVEL : GLenum = 0x8CD2;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_CUBE_MAP_FACE : GLenum = 0x8CD3;
    pub const GL_FRAMEBUFFER_ATTACHMENT_TEXTURE_LAYER : GLenum = 0x8CD4;
    pub const GL_FRAMEBUFFER_COMPLETE           : GLenum = 0x8CD5;
    pub const GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT : GLenum = 0x8CD6;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT : GLenum = 0x8CD7;
    pub const GL_FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER : GLenum = 0x8CDB;
    pub const GL_FRAMEBUFFER_INCOMPLETE_READ_BUFFER : GLenum = 0x8CDC;
    pub const GL_FRAMEBUFFER_UNSUPPORTED        : GLenum = 0x8CDD;
    pub const GL_MAX_COLOR_ATTACHMENTS          : GLenum = 0x8CDF;
    pub const GL_COLOR_ATTACHMENT0              : GLenum = 0x8CE0;
    pub const GL_COLOR_ATTACHMENT1              : GLenum = 0x8CE1;
    pub const GL_COLOR_ATTACHMENT2              : GLenum = 0x8CE2;
    pub const GL_COLOR_ATTACHMENT3              : GLenum = 0x8CE3;
    pub const GL_COLOR_ATTACHMENT4              : GLenum = 0x8CE4;
    pub const GL_COLOR_ATTACHMENT5              : GLenum = 0x8CE5;
    pub const GL_COLOR_ATTACHMENT6              : GLenum = 0x8CE6;
    pub const GL_COLOR_ATTACHMENT7              : GLenum = 0x8CE7;
    pub const GL_COLOR_ATTACHMENT8              : GLenum = 0x8CE8;
    pub const GL_COLOR_ATTACHMENT9              : GLenum = 0x8CE9;
    pub const GL_COLOR_ATTACHMENT10             : GLenum = 0x8CEA;
    pub const GL_COLOR_ATTACHMENT11             : GLenum = 0x8CEB;
    pub const GL_COLOR_ATTACHMENT12             : GLenum = 0x8CEC;
    pub const GL_COLOR_ATTACHMENT13             : GLenum = 0x8CED;
    pub const GL_COLOR_ATTACHMENT14             : GLenum = 0x8CEE;
    pub const GL_COLOR_ATTACHMENT15             : GLenum = 0x8CEF;
    pub const GL_DEPTH_ATTACHMENT               : GLenum = 0x8D00;
    pub const GL_STENCIL_ATTACHMENT             : GLenum = 0x8D20;
    pub const GL_FRAMEBUFFER                    : GLenum = 0x8D40;
    pub const GL_RENDERBUFFER                   : GLenum = 0x8D41;
    pub const GL_RENDERBUFFER_WIDTH             : GLenum = 0x8D42;
    pub const GL_RENDERBUFFER_HEIGHT            : GLenum = 0x8D43;
    pub const GL_RENDERBUFFER_INTERNAL_FORMAT   : GLenum = 0x8D44;
    pub const GL_STENCIL_INDEX1                 : GLenum = 0x8D46;
    pub const GL_STENCIL_INDEX4                 : GLenum = 0x8D47;
    pub const GL_STENCIL_INDEX8                 : GLenum = 0x8D48;
    pub const GL_STENCIL_INDEX16                : GLenum = 0x8D49;
    pub const GL_RENDERBUFFER_RED_SIZE          : GLenum = 0x8D50;
    pub const GL_RENDERBUFFER_GREEN_SIZE        : GLenum = 0x8D51;
    pub const GL_RENDERBUFFER_BLUE_SIZE         : GLenum = 0x8D52;
    pub const GL_RENDERBUFFER_ALPHA_SIZE        : GLenum = 0x8D53;
    pub const GL_RENDERBUFFER_DEPTH_SIZE        : GLenum = 0x8D54;
    pub const GL_RENDERBUFFER_STENCIL_SIZE      : GLenum = 0x8D55;
    pub const GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE : GLenum = 0x8D56;
    pub const GL_MAX_SAMPLES                    : GLenum = 0x8D57;
}

#[cfg(arb_framebuffer_srgb)]
pub mod arb_framebuffer_srgb {
    pub const GL_FRAMEBUFFER_SRGB               : GLenum = 0x8DB9;
}

#[cfg(arb_half_float_vertex)]
pub mod arb_half_float_vertex {
    pub const GL_HALF_FLOAT                     : GLenum = 0x140B;
}

#[cfg(arb_map_buffer_range)]
pub mod arb_map_buffer_range {
    pub const GL_MAP_READ_BIT                   : GLenum = 0x0001;
    pub const GL_MAP_WRITE_BIT                  : GLenum = 0x0002;
    pub const GL_MAP_INVALIDATE_RANGE_BIT       : GLenum = 0x0004;
    pub const GL_MAP_INVALIDATE_BUFFER_BIT      : GLenum = 0x0008;
    pub const GL_MAP_FLUSH_EXPLICIT_BIT         : GLenum = 0x0010;
    pub const GL_MAP_UNSYNCHRONIZED_BIT         : GLenum = 0x0020;
}

#[cfg(arb_texture_compression_rgtc)]
pub mod arb_texture_compression_rgtc {
    pub const GL_COMPRESSED_RED_RGTC1           : GLenum = 0x8DBB;
    pub const GL_COMPRESSED_SIGNED_RED_RGTC1    : GLenum = 0x8DBC;
    pub const GL_COMPRESSED_RG_RGTC2            : GLenum = 0x8DBD;
    pub const GL_COMPRESSED_SIGNED_RG_RGTC2     : GLenum = 0x8DBE;
}

#[cfg(arb_texture_rg)]
pub mod arb_texture_rg {
    pub const GL_RG                             : GLenum = 0x8227;
    pub const GL_RG_INTEGER                     : GLenum = 0x8228;
    pub const GL_R8                             : GLenum = 0x8229;
    pub const GL_R16                            : GLenum = 0x822A;
    pub const GL_RG8                            : GLenum = 0x822B;
    pub const GL_RG16                           : GLenum = 0x822C;
    pub const GL_R16F                           : GLenum = 0x822D;
    pub const GL_R32F                           : GLenum = 0x822E;
    pub const GL_RG16F                          : GLenum = 0x822F;
    pub const GL_RG32F                          : GLenum = 0x8230;
    pub const GL_R8I                            : GLenum = 0x8231;
    pub const GL_R8UI                           : GLenum = 0x8232;
    pub const GL_R16I                           : GLenum = 0x8233;
    pub const GL_R16UI                          : GLenum = 0x8234;
    pub const GL_R32I                           : GLenum = 0x8235;
    pub const GL_R32UI                          : GLenum = 0x8236;
    pub const GL_RG8I                           : GLenum = 0x8237;
    pub const GL_RG8UI                          : GLenum = 0x8238;
    pub const GL_RG16I                          : GLenum = 0x8239;
    pub const GL_RG16UI                         : GLenum = 0x823A;
    pub const GL_RG32I                          : GLenum = 0x823B;
    pub const GL_RG32UI                         : GLenum = 0x823C;
}

#[cfg(arb_vertex_array_object)]
pub mod arb_vertex_array_object {
    pub const GL_VERTEX_ARRAY_BINDING           : GLenum = 0x85B5;
}

#[cfg(arb_uniform_buffer_object)]
pub mod arb_uniform_buffer_object {
    pub const GL_UNIFORM_BUFFER                 : GLenum = 0x8A11;
    pub const GL_UNIFORM_BUFFER_BINDING         : GLenum = 0x8A28;
    pub const GL_UNIFORM_BUFFER_START           : GLenum = 0x8A29;
    pub const GL_UNIFORM_BUFFER_SIZE            : GLenum = 0x8A2A;
    pub const GL_MAX_VERTEX_UNIFORM_BLOCKS      : GLenum = 0x8A2B;
    pub const GL_MAX_GEOMETRY_UNIFORM_BLOCKS    : GLenum = 0x8A2C;
    pub const GL_MAX_FRAGMENT_UNIFORM_BLOCKS    : GLenum = 0x8A2D;
    pub const GL_MAX_COMBINED_UNIFORM_BLOCKS    : GLenum = 0x8A2E;
    pub const GL_MAX_UNIFORM_BUFFER_BINDINGS    : GLenum = 0x8A2F;
    pub const GL_MAX_UNIFORM_BLOCK_SIZE         : GLenum = 0x8A30;
    pub const GL_MAX_COMBINED_VERTEX_UNIFORM_COMPONENTS : GLenum = 0x8A31;
    pub const GL_MAX_COMBINED_GEOMETRY_UNIFORM_COMPONENTS : GLenum = 0x8A32;
    pub const GL_MAX_COMBINED_FRAGMENT_UNIFORM_COMPONENTS : GLenum = 0x8A33;
    pub const GL_UNIFORM_BUFFER_OFFSET_ALIGNMENT : GLenum = 0x8A34;
    pub const GL_ACTIVE_UNIFORM_BLOCK_MAX_NAME_LENGTH : GLenum = 0x8A35;
    pub const GL_ACTIVE_UNIFORM_BLOCKS          : GLenum = 0x8A36;
    pub const GL_UNIFORM_TYPE                   : GLenum = 0x8A37;
    pub const GL_UNIFORM_SIZE                   : GLenum = 0x8A38;
    pub const GL_UNIFORM_NAME_LENGTH            : GLenum = 0x8A39;
    pub const GL_UNIFORM_BLOCK_INDEX            : GLenum = 0x8A3A;
    pub const GL_UNIFORM_OFFSET                 : GLenum = 0x8A3B;
    pub const GL_UNIFORM_ARRAY_STRIDE           : GLenum = 0x8A3C;
    pub const GL_UNIFORM_MATRIX_STRIDE          : GLenum = 0x8A3D;
    pub const GL_UNIFORM_IS_ROW_MAJOR           : GLenum = 0x8A3E;
    pub const GL_UNIFORM_BLOCK_BINDING          : GLenum = 0x8A3F;
    pub const GL_UNIFORM_BLOCK_DATA_SIZE        : GLenum = 0x8A40;
    pub const GL_UNIFORM_BLOCK_NAME_LENGTH      : GLenum = 0x8A41;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORMS  : GLenum = 0x8A42;
    pub const GL_UNIFORM_BLOCK_ACTIVE_UNIFORM_INDICES : GLenum = 0x8A43;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_VERTEX_SHADER : GLenum = 0x8A44;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_GEOMETRY_SHADER : GLenum = 0x8A45;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_FRAGMENT_SHADER : GLenum = 0x8A46;
    pub const GL_INVALID_INDEX                  : u32 = 0xFFFFFFFFu32;
}

#[cfg(arb_copy_buffer)]
pub mod arb_copy_buffer {
    pub const GL_COPY_READ_BUFFER_BINDING       : GLenum = 0x8F36;
    pub const GL_COPY_READ_BUFFER               : GLenum = GL_COPY_READ_BUFFER_BINDING;
    pub const GL_COPY_WRITE_BUFFER_BINDING      : GLenum = 0x8F37;
    pub const GL_COPY_WRITE_BUFFER              : GLenum = GL_COPY_WRITE_BUFFER_BINDING;
}

#[cfg(arb_depth_clamp)]
pub mod arb_depth_clamp {
    pub const GL_DEPTH_CLAMP                    : GLenum = 0x864F;
}

#[cfg(arb_draw_elements_base_vertex)]
pub mod arb_draw_elements_base_vertex {
}

#[cfg(arb_fragment_coord_conventions)]
pub mod arb_fragment_coord_conventions {
}

#[cfg(arb_provoking_vertex)]
pub mod arb_provoking_vertex {
    pub const GL_QUADS_FOLLOW_PROVOKING_VERTEX_CONVENTION : GLenum = 0x8E4C;
    pub const GL_FIRST_VERTEX_CONVENTION        : GLenum = 0x8E4D;
    pub const GL_LAST_VERTEX_CONVENTION         : GLenum = 0x8E4E;
    pub const GL_PROVOKING_VERTEX               : GLenum = 0x8E4F;
}

#[cfg(arb_seamless_cube_map)]
pub mod arb_seamless_cube_map {
    pub const GL_TEXTURE_CUBE_MAP_SEAMLESS      : GLenum = 0x884F;
}

#[cfg(arb_sync)]
pub mod arb_sync {
    pub const GL_MAX_SERVER_WAIT_TIMEOUT        : GLenum = 0x9111;
    pub const GL_OBJECT_TYPE                    : GLenum = 0x9112;
    pub const GL_SYNC_CONDITION                 : GLenum = 0x9113;
    pub const GL_SYNC_STATUS                    : GLenum = 0x9114;
    pub const GL_SYNC_FLAGS                     : GLenum = 0x9115;
    pub const GL_SYNC_FENCE                     : GLenum = 0x9116;
    pub const GL_SYNC_GPU_COMMANDS_COMPLETE     : GLenum = 0x9117;
    pub const GL_UNSIGNALED                     : GLenum = 0x9118;
    pub const GL_SIGNALED                       : GLenum = 0x9119;
    pub const GL_ALREADY_SIGNALED               : GLenum = 0x911A;
    pub const GL_TIMEOUT_EXPIRED                : GLenum = 0x911B;
    pub const GL_CONDITION_SATISFIED            : GLenum = 0x911C;
    pub const GL_WAIT_FAILED                    : GLenum = 0x911D;
    pub const GL_SYNC_FLUSH_COMMANDS_BIT        : GLenum = 0x00000001;
    pub const GL_TIMEOUT_IGNORED                : u64 = 0xFFFFFFFFFFFFFFFFu64;
}

#[cfg(arb_texture_multisample)]
pub mod arb_texture_multisample {
    pub const GL_SAMPLE_POSITION                : GLenum = 0x8E50;
    pub const GL_SAMPLE_MASK                    : GLenum = 0x8E51;
    pub const GL_SAMPLE_MASK_VALUE              : GLenum = 0x8E52;
    pub const GL_MAX_SAMPLE_MASK_WORDS          : GLenum = 0x8E59;
    pub const GL_TEXTURE_2D_MULTISAMPLE         : GLenum = 0x9100;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE   : GLenum = 0x9101;
    pub const GL_TEXTURE_2D_MULTISAMPLE_ARRAY   : GLenum = 0x9102;
    pub const GL_PROXY_TEXTURE_2D_MULTISAMPLE_ARRAY : GLenum = 0x9103;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE : GLenum = 0x9104;
    pub const GL_TEXTURE_BINDING_2D_MULTISAMPLE_ARRAY : GLenum = 0x9105;
    pub const GL_TEXTURE_SAMPLES                : GLenum = 0x9106;
    pub const GL_TEXTURE_FIXED_SAMPLE_LOCATIONS : GLenum = 0x9107;
    pub const GL_SAMPLER_2D_MULTISAMPLE         : GLenum = 0x9108;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE     : GLenum = 0x9109;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE : GLenum = 0x910A;
    pub const GL_SAMPLER_2D_MULTISAMPLE_ARRAY   : GLenum = 0x910B;
    pub const GL_INT_SAMPLER_2D_MULTISAMPLE_ARRAY : GLenum = 0x910C;
    pub const GL_UNSIGNED_INT_SAMPLER_2D_MULTISAMPLE_ARRAY : GLenum = 0x910D;
    pub const GL_MAX_COLOR_TEXTURE_SAMPLES      : GLenum = 0x910E;
    pub const GL_MAX_DEPTH_TEXTURE_SAMPLES      : GLenum = 0x910F;
    pub const GL_MAX_INTEGER_SAMPLES            : GLenum = 0x9110;
}

#[cfg(arb_vertex_array_bgra)]
pub mod arb_vertex_array_bgra {
    /* reuse GL_BGRA */
}

#[cfg(arb_draw_buffers_blend)]
pub mod arb_draw_buffers_blend {
}

#[cfg(arb_sample_shading)]
pub mod arb_sample_shading {
    pub const GL_SAMPLE_SHADING_ARB             : GLenum = 0x8C36;
    pub const GL_MIN_SAMPLE_SHADING_VALUE_ARB   : GLenum = 0x8C37;
}

#[cfg(arb_texture_cube_map_array)]
pub mod arb_texture_cube_map_array {
    pub const GL_TEXTURE_CUBE_MAP_ARRAY_ARB     : GLenum = 0x9009;
    pub const GL_TEXTURE_BINDING_CUBE_MAP_ARRAY_ARB : GLenum = 0x900A;
    pub const GL_PROXY_TEXTURE_CUBE_MAP_ARRAY_ARB : GLenum = 0x900B;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_ARB     : GLenum = 0x900C;
    pub const GL_SAMPLER_CUBE_MAP_ARRAY_SHADOW_ARB : GLenum = 0x900D;
    pub const GL_INT_SAMPLER_CUBE_MAP_ARRAY_ARB : GLenum = 0x900E;
    pub const GL_UNSIGNED_INT_SAMPLER_CUBE_MAP_ARRAY_ARB : GLenum = 0x900F;
}

#[cfg(arb_texture_gather)]
pub mod arb_texture_gather {
    pub const GL_MIN_PROGRAM_TEXTURE_GATHER_OFFSET_ARB : GLenum = 0x8E5E;
    pub const GL_MAX_PROGRAM_TEXTURE_GATHER_OFFSET_ARB : GLenum = 0x8E5F;
    pub const GL_MAX_PROGRAM_TEXTURE_GATHER_COMPONENTS_ARB : GLenum = 0x8F9F;
}

#[cfg(arb_texture_query_lod)]
pub mod arb_texture_query_lod {
}

#[cfg(arb_shading_language_include)]
pub mod arb_shading_language_include {
    pub const GL_SHADER_INCLUDE_ARB             : GLenum = 0x8DAE;
    pub const GL_NAMED_STRING_LENGTH_ARB        : GLenum = 0x8DE9;
    pub const GL_NAMED_STRING_TYPE_ARB          : GLenum = 0x8DEA;
}

#[cfg(arb_texture_compression_bptc)]
pub mod arb_texture_compression_bptc {
    pub const GL_COMPRESSED_RGBA_BPTC_UNORM_ARB : GLenum = 0x8E8C;
    pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB : GLenum = 0x8E8D;
    pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB : GLenum = 0x8E8E;
    pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB : GLenum = 0x8E8F;
}

#[cfg(arb_blend_func_extended)]
pub mod arb_blend_func_extended {
    pub const GL_SRC1_COLOR                     : GLenum = 0x88F9;
    /* reuse GL_SRC1_ALPHA */
    pub const GL_ONE_MINUS_SRC1_COLOR           : GLenum = 0x88FA;
    pub const GL_ONE_MINUS_SRC1_ALPHA           : GLenum = 0x88FB;
    pub const GL_MAX_DUAL_SOURCE_DRAW_BUFFERS   : GLenum = 0x88FC;
}

#[cfg(arb_explicit_attrib_location)]
pub mod arb_explicit_attrib_location {
}

#[cfg(arb_occlusion_query2)]
pub mod arb_occlusion_query2 {
    pub const GL_ANY_SAMPLES_PASSED             : GLenum = 0x8C2F;
}

#[cfg(arb_sampler_objects)]
pub mod arb_sampler_objects {
    pub const GL_SAMPLER_BINDING                : GLenum = 0x8919;
}

#[cfg(arb_shader_bit_encoding)]
pub mod arb_shader_bit_encoding {
}

#[cfg(arb_texture_rgb10_a2ui)]
pub mod arb_texture_rgb10_a2ui {
    pub const GL_RGB10_A2UI                     : GLenum = 0x906F;
}

#[cfg(arb_texture_swizzle)]
pub mod arb_texture_swizzle {
    pub const GL_TEXTURE_SWIZZLE_R              : GLenum = 0x8E42;
    pub const GL_TEXTURE_SWIZZLE_G              : GLenum = 0x8E43;
    pub const GL_TEXTURE_SWIZZLE_B              : GLenum = 0x8E44;
    pub const GL_TEXTURE_SWIZZLE_A              : GLenum = 0x8E45;
    pub const GL_TEXTURE_SWIZZLE_RGBA           : GLenum = 0x8E46;
}

#[cfg(arb_timer_query)]
pub mod arb_timer_query {
    pub const GL_TIME_ELAPSED                   : GLenum = 0x88BF;
    pub const GL_TIMESTAMP                      : GLenum = 0x8E28;
}

#[cfg(arb_vertex_type_2_10_10_10_rev)]
pub mod arb_vertex_type_2_10_10_10_rev {
    /* reuse GL_UNSIGNED_INT_2_10_10_10_REV */
    pub const GL_INT_2_10_10_10_REV             : GLenum = 0x8D9F;
}

#[cfg(arb_draw_indirect)]
pub mod arb_draw_indirect {
    pub const GL_DRAW_INDIRECT_BUFFER           : GLenum = 0x8F3F;
    pub const GL_DRAW_INDIRECT_BUFFER_BINDING   : GLenum = 0x8F43;
}

#[cfg(arb_gpu_shader5)]
pub mod arb_gpu_shader5 {
    pub const GL_GEOMETRY_SHADER_INVOCATIONS    : GLenum = 0x887F;
    pub const GL_MAX_GEOMETRY_SHADER_INVOCATIONS : GLenum = 0x8E5A;
    pub const GL_MIN_FRAGMENT_INTERPOLATION_OFFSET : GLenum = 0x8E5B;
    pub const GL_MAX_FRAGMENT_INTERPOLATION_OFFSET : GLenum = 0x8E5C;
    pub const GL_FRAGMENT_INTERPOLATION_OFFSET_BITS : GLenum = 0x8E5D;
    /* reuse GL_MAX_VERTEX_STREAMS */
}

#[cfg(arb_gpu_shader_fp64)]
pub mod arb_gpu_shader_fp64 {
    /* reuse GL_DOUBLE */
    pub const GL_DOUBLE_VEC2                    : GLenum = 0x8FFC;
    pub const GL_DOUBLE_VEC3                    : GLenum = 0x8FFD;
    pub const GL_DOUBLE_VEC4                    : GLenum = 0x8FFE;
    pub const GL_DOUBLE_MAT2                    : GLenum = 0x8F46;
    pub const GL_DOUBLE_MAT3                    : GLenum = 0x8F47;
    pub const GL_DOUBLE_MAT4                    : GLenum = 0x8F48;
    pub const GL_DOUBLE_MAT2x3                  : GLenum = 0x8F49;
    pub const GL_DOUBLE_MAT2x4                  : GLenum = 0x8F4A;
    pub const GL_DOUBLE_MAT3x2                  : GLenum = 0x8F4B;
    pub const GL_DOUBLE_MAT3x4                  : GLenum = 0x8F4C;
    pub const GL_DOUBLE_MAT4x2                  : GLenum = 0x8F4D;
    pub const GL_DOUBLE_MAT4x3                  : GLenum = 0x8F4E;
}

#[cfg(arb_shader_subroutine)]
pub mod arb_shader_subroutine {
    pub const GL_ACTIVE_SUBROUTINES             : GLenum = 0x8DE5;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORMS     : GLenum = 0x8DE6;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_LOCATIONS : GLenum = 0x8E47;
    pub const GL_ACTIVE_SUBROUTINE_MAX_LENGTH   : GLenum = 0x8E48;
    pub const GL_ACTIVE_SUBROUTINE_UNIFORM_MAX_LENGTH : GLenum = 0x8E49;
    pub const GL_MAX_SUBROUTINES                : GLenum = 0x8DE7;
    pub const GL_MAX_SUBROUTINE_UNIFORM_LOCATIONS : GLenum = 0x8DE8;
    pub const GL_NUM_COMPATIBLE_SUBROUTINES     : GLenum = 0x8E4A;
    pub const GL_COMPATIBLE_SUBROUTINES         : GLenum = 0x8E4B;
    /* reuse GL_UNIFORM_SIZE */
    /* reuse GL_UNIFORM_NAME_LENGTH */
}

#[cfg(arb_tessellation_shader)]
pub mod arb_tessellation_shader {
    pub const GL_PATCHES                        : GLenum = 0x000E;
    pub const GL_PATCH_VERTICES                 : GLenum = 0x8E72;
    pub const GL_PATCH_DEFAULT_INNER_LEVEL      : GLenum = 0x8E73;
    pub const GL_PATCH_DEFAULT_OUTER_LEVEL      : GLenum = 0x8E74;
    pub const GL_TESS_CONTROL_OUTPUT_VERTICES   : GLenum = 0x8E75;
    pub const GL_TESS_GEN_MODE                  : GLenum = 0x8E76;
    pub const GL_TESS_GEN_SPACING               : GLenum = 0x8E77;
    pub const GL_TESS_GEN_VERTEX_ORDER          : GLenum = 0x8E78;
    pub const GL_TESS_GEN_POINT_MODE            : GLenum = 0x8E79;
    /* reuse GL_TRIANGLES */
    /* reuse GL_QUADS */
    pub const GL_ISOLINES                       : GLenum = 0x8E7A;
    /* reuse GL_EQUAL */
    pub const GL_FRACTIONAL_ODD                 : GLenum = 0x8E7B;
    pub const GL_FRACTIONAL_EVEN                : GLenum = 0x8E7C;
    /* reuse GL_CCW */
    /* reuse GL_CW */
    pub const GL_MAX_PATCH_VERTICES             : GLenum = 0x8E7D;
    pub const GL_MAX_TESS_GEN_LEVEL             : GLenum = 0x8E7E;
    pub const GL_MAX_TESS_CONTROL_UNIFORM_COMPONENTS : GLenum = 0x8E7F;
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_COMPONENTS : GLenum = 0x8E80;
    pub const GL_MAX_TESS_CONTROL_TEXTURE_IMAGE_UNITS : GLenum = 0x8E81;
    pub const GL_MAX_TESS_EVALUATION_TEXTURE_IMAGE_UNITS : GLenum = 0x8E82;
    pub const GL_MAX_TESS_CONTROL_OUTPUT_COMPONENTS : GLenum = 0x8E83;
    pub const GL_MAX_TESS_PATCH_COMPONENTS      : GLenum = 0x8E84;
    pub const GL_MAX_TESS_CONTROL_TOTAL_OUTPUT_COMPONENTS : GLenum = 0x8E85;
    pub const GL_MAX_TESS_EVALUATION_OUTPUT_COMPONENTS : GLenum = 0x8E86;
    pub const GL_MAX_TESS_CONTROL_UNIFORM_BLOCKS : GLenum = 0x8E89;
    pub const GL_MAX_TESS_EVALUATION_UNIFORM_BLOCKS : GLenum = 0x8E8A;
    pub const GL_MAX_TESS_CONTROL_INPUT_COMPONENTS : GLenum = 0x886C;
    pub const GL_MAX_TESS_EVALUATION_INPUT_COMPONENTS : GLenum = 0x886D;
    pub const GL_MAX_COMBINED_TESS_CONTROL_UNIFORM_COMPONENTS : GLenum = 0x8E1E;
    pub const GL_MAX_COMBINED_TESS_EVALUATION_UNIFORM_COMPONENTS : GLenum = 0x8E1F;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_CONTROL_SHADER : GLenum = 0x84F0;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_TESS_EVALUATION_SHADER : GLenum = 0x84F1;
    pub const GL_TESS_EVALUATION_SHADER         : GLenum = 0x8E87;
    pub const GL_TESS_CONTROL_SHADER            : GLenum = 0x8E88;
}

#[cfg(arb_texture_buffer_object_rgb32)]
pub mod arb_texture_buffer_object_rgb32 {
    /* reuse GL_RGB32F */
    /* reuse GL_RGB32UI */
    /* reuse GL_RGB32I */
}

#[cfg(arb_transform_feedback2)]
pub mod arb_transform_feedback2 {
    pub const GL_TRANSFORM_FEEDBACK             : GLenum = 0x8E22;
    pub const GL_TRANSFORM_FEEDBACK_PAUSED      : GLenum = 0x8E23;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_PAUSED : GLenum = GL_TRANSFORM_FEEDBACK_PAUSED;
    pub const GL_TRANSFORM_FEEDBACK_ACTIVE      : GLenum = 0x8E24;
    pub const GL_TRANSFORM_FEEDBACK_BUFFER_ACTIVE : GLenum = GL_TRANSFORM_FEEDBACK_ACTIVE;
    pub const GL_TRANSFORM_FEEDBACK_BINDING     : GLenum = 0x8E25;
}

#[cfg(arb_transform_feedback3)]
pub mod arb_transform_feedback3 {
    pub const GL_MAX_TRANSFORM_FEEDBACK_BUFFERS : GLenum = 0x8E70;
    pub const GL_MAX_VERTEX_STREAMS             : GLenum = 0x8E71;
}

#[cfg(arb_es2_compatibility)]
pub mod arb_es2_compatibility {
    pub const GL_FIXED                          : GLenum = 0x140C;
    pub const GL_IMPLEMENTATION_COLOR_READ_TYPE : GLenum = 0x8B9A;
    pub const GL_IMPLEMENTATION_COLOR_READ_FORMAT : GLenum = 0x8B9B;
    pub const GL_LOW_FLOAT                      : GLenum = 0x8DF0;
    pub const GL_MEDIUM_FLOAT                   : GLenum = 0x8DF1;
    pub const GL_HIGH_FLOAT                     : GLenum = 0x8DF2;
    pub const GL_LOW_INT                        : GLenum = 0x8DF3;
    pub const GL_MEDIUM_INT                     : GLenum = 0x8DF4;
    pub const GL_HIGH_INT                       : GLenum = 0x8DF5;
    pub const GL_SHADER_COMPILER                : GLenum = 0x8DFA;
    pub const GL_SHADER_BINARY_FORMATS          : GLenum = 0x8DF8;
    pub const GL_NUM_SHADER_BINARY_FORMATS      : GLenum = 0x8DF9;
    pub const GL_MAX_VERTEX_UNIFORM_VECTORS     : GLenum = 0x8DFB;
    pub const GL_MAX_VARYING_VECTORS            : GLenum = 0x8DFC;
    pub const GL_MAX_FRAGMENT_UNIFORM_VECTORS   : GLenum = 0x8DFD;
    pub const GL_RGB565                         : GLenum = 0x8D62;
}

#[cfg(arb_get_program_binary)]
pub mod arb_get_program_binary {
    pub const GL_PROGRAM_BINARY_RETRIEVABLE_HINT : GLenum = 0x8257;
    pub const GL_PROGRAM_BINARY_LENGTH          : GLenum = 0x8741;
    pub const GL_NUM_PROGRAM_BINARY_FORMATS     : GLenum = 0x87FE;
    pub const GL_PROGRAM_BINARY_FORMATS         : GLenum = 0x87FF;
}

#[cfg(arb_separate_shader_objects)]
pub mod arb_separate_shader_objects {
    pub const GL_VERTEX_SHADER_BIT              : GLenum = 0x00000001;
    pub const GL_FRAGMENT_SHADER_BIT            : GLenum = 0x00000002;
    pub const GL_GEOMETRY_SHADER_BIT            : GLenum = 0x00000004;
    pub const GL_TESS_CONTROL_SHADER_BIT        : GLenum = 0x00000008;
    pub const GL_TESS_EVALUATION_SHADER_BIT     : GLenum = 0x00000010;
    pub const GL_ALL_SHADER_BITS                : GLenum = 0xFFFFFFFF;
    pub const GL_PROGRAM_SEPARABLE              : GLenum = 0x8258;
    pub const GL_ACTIVE_PROGRAM                 : GLenum = 0x8259;
    pub const GL_PROGRAM_PIPELINE_BINDING       : GLenum = 0x825A;
}

#[cfg(arb_shader_precision)]
pub mod arb_shader_precision {
}

#[cfg(arb_vertex_attrib_64bit)]
pub mod arb_vertex_attrib_64bit {
    /* reuse GL_RGB32I */
    /* reuse GL_DOUBLE_VEC2 */
    /* reuse GL_DOUBLE_VEC3 */
    /* reuse GL_DOUBLE_VEC4 */
    /* reuse GL_DOUBLE_MAT2 */
    /* reuse GL_DOUBLE_MAT3 */
    /* reuse GL_DOUBLE_MAT4 */
    /* reuse GL_DOUBLE_MAT2x3 */
    /* reuse GL_DOUBLE_MAT2x4 */
    /* reuse GL_DOUBLE_MAT3x2 */
    /* reuse GL_DOUBLE_MAT3x4 */
    /* reuse GL_DOUBLE_MAT4x2 */
    /* reuse GL_DOUBLE_MAT4x3 */
}

#[cfg(arb_viewport_array)]
pub mod arb_viewport_array {
    /* reuse GL_SCISSOR_BOX */
    /* reuse GL_VIEWPORT */
    /* reuse GL_DEPTH_RANGE */
    /* reuse GL_SCISSOR_TEST */
    pub const GL_MAX_VIEWPORTS                  : GLenum = 0x825B;
    pub const GL_VIEWPORT_SUBPIXEL_BITS         : GLenum = 0x825C;
    pub const GL_VIEWPORT_BOUNDS_RANGE          : GLenum = 0x825D;
    pub const GL_LAYER_PROVOKING_VERTEX         : GLenum = 0x825E;
    pub const GL_VIEWPORT_INDEX_PROVOKING_VERTEX : GLenum = 0x825F;
    pub const GL_UNDEFINED_VERTEX               : GLenum = 0x8260;
    /* reuse GL_FIRST_VERTEX_CONVENTION */
    /* reuse GL_LAST_VERTEX_CONVENTION */
    /* reuse GL_PROVOKING_VERTEX */
}

#[cfg(arb_cl_event)]
pub mod arb_cl_event {
    pub const GL_SYNC_CL_EVENT_ARB              : GLenum = 0x8240;
    pub const GL_SYNC_CL_EVENT_COMPLETE_ARB     : GLenum = 0x8241;
}

#[cfg(arb_debug_output)]
pub mod arb_debug_output {
    pub const GL_DEBUG_OUTPUT_SYNCHRONOUS_ARB   : GLenum = 0x8242;
    pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH_ARB : GLenum = 0x8243;
    pub const GL_DEBUG_CALLBACK_FUNCTION_ARB    : GLenum = 0x8244;
    pub const GL_DEBUG_CALLBACK_USER_PARAM_ARB  : GLenum = 0x8245;
    pub const GL_DEBUG_SOURCE_API_ARB           : GLenum = 0x8246;
    pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM_ARB : GLenum = 0x8247;
    pub const GL_DEBUG_SOURCE_SHADER_COMPILER_ARB : GLenum = 0x8248;
    pub const GL_DEBUG_SOURCE_THIRD_PARTY_ARB   : GLenum = 0x8249;
    pub const GL_DEBUG_SOURCE_APPLICATION_ARB   : GLenum = 0x824A;
    pub const GL_DEBUG_SOURCE_OTHER_ARB         : GLenum = 0x824B;
    pub const GL_DEBUG_TYPE_ERROR_ARB           : GLenum = 0x824C;
    pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR_ARB : GLenum = 0x824D;
    pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR_ARB : GLenum = 0x824E;
    pub const GL_DEBUG_TYPE_PORTABILITY_ARB     : GLenum = 0x824F;
    pub const GL_DEBUG_TYPE_PERFORMANCE_ARB     : GLenum = 0x8250;
    pub const GL_DEBUG_TYPE_OTHER_ARB           : GLenum = 0x8251;
    pub const GL_MAX_DEBUG_MESSAGE_LENGTH_ARB   : GLenum = 0x9143;
    pub const GL_MAX_DEBUG_LOGGED_MESSAGES_ARB  : GLenum = 0x9144;
    pub const GL_DEBUG_LOGGED_MESSAGES_ARB      : GLenum = 0x9145;
    pub const GL_DEBUG_SEVERITY_HIGH_ARB        : GLenum = 0x9146;
    pub const GL_DEBUG_SEVERITY_MEDIUM_ARB      : GLenum = 0x9147;
    pub const GL_DEBUG_SEVERITY_LOW_ARB         : GLenum = 0x9148;
}

#[cfg(arb_robustness)]
pub mod arb_robustness {
    /* reuse GL_NO_ERROR */
    pub const GL_CONTEXT_FLAG_ROBUST_ACCESS_BIT_ARB : GLenum = 0x00000004;
    pub const GL_LOSE_CONTEXT_ON_RESET_ARB      : GLenum = 0x8252;
    pub const GL_GUILTY_CONTEXT_RESET_ARB       : GLenum = 0x8253;
    pub const GL_INNOCENT_CONTEXT_RESET_ARB     : GLenum = 0x8254;
    pub const GL_UNKNOWN_CONTEXT_RESET_ARB      : GLenum = 0x8255;
    pub const GL_RESET_NOTIFICATION_STRATEGY_ARB : GLenum = 0x8256;
    pub const GL_NO_RESET_NOTIFICATION_ARB      : GLenum = 0x8261;
}

#[cfg(arb_shader_stencil_export)]
pub mod arb_shader_stencil_export {
}

#[cfg(arb_base_instance)]
pub mod arb_base_instance {
}

#[cfg(arb_shading_language_420pack)]
pub mod arb_shading_language_420pack {
}

#[cfg(arb_transform_feedback_instanced)]
pub mod arb_transform_feedback_instanced {
}

#[cfg(arb_compressed_texture_pixel_storage)]
pub mod arb_compressed_texture_pixel_storage {
    pub const GL_UNPACK_COMPRESSED_BLOCK_WIDTH  : GLenum = 0x9127;
    pub const GL_UNPACK_COMPRESSED_BLOCK_HEIGHT : GLenum = 0x9128;
    pub const GL_UNPACK_COMPRESSED_BLOCK_DEPTH  : GLenum = 0x9129;
    pub const GL_UNPACK_COMPRESSED_BLOCK_SIZE   : GLenum = 0x912A;
    pub const GL_PACK_COMPRESSED_BLOCK_WIDTH    : GLenum = 0x912B;
    pub const GL_PACK_COMPRESSED_BLOCK_HEIGHT   : GLenum = 0x912C;
    pub const GL_PACK_COMPRESSED_BLOCK_DEPTH    : GLenum = 0x912D;
    pub const GL_PACK_COMPRESSED_BLOCK_SIZE     : GLenum = 0x912E;
}

#[cfg(arb_conservative_depth)]
pub mod arb_conservative_depth {
}

#[cfg(arb_internalformat_query)]
pub mod arb_internalformat_query {
    pub const GL_NUM_SAMPLE_COUNTS              : GLenum = 0x9380;
}

#[cfg(arb_map_buffer_alignment)]
pub mod arb_map_buffer_alignment {
    pub const GL_MIN_MAP_BUFFER_ALIGNMENT       : GLenum = 0x90BC;
}

#[cfg(arb_shader_atomic_counters)]
pub mod arb_shader_atomic_counters {
    pub const GL_ATOMIC_COUNTER_BUFFER          : GLenum = 0x92C0;
    pub const GL_ATOMIC_COUNTER_BUFFER_BINDING  : GLenum = 0x92C1;
    pub const GL_ATOMIC_COUNTER_BUFFER_START    : GLenum = 0x92C2;
    pub const GL_ATOMIC_COUNTER_BUFFER_SIZE     : GLenum = 0x92C3;
    pub const GL_ATOMIC_COUNTER_BUFFER_DATA_SIZE : GLenum = 0x92C4;
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTERS : GLenum = 0x92C5;
    pub const GL_ATOMIC_COUNTER_BUFFER_ACTIVE_ATOMIC_COUNTER_INDICES : GLenum = 0x92C6;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_VERTEX_SHADER : GLenum = 0x92C7;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_CONTROL_SHADER : GLenum = 0x92C8;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_TESS_EVALUATION_SHADER : GLenum = 0x92C9;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_GEOMETRY_SHADER : GLenum = 0x92CA;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_FRAGMENT_SHADER : GLenum = 0x92CB;
    pub const GL_MAX_VERTEX_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CC;
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CD;
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CE;
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92CF;
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92D0;
    pub const GL_MAX_COMBINED_ATOMIC_COUNTER_BUFFERS : GLenum = 0x92D1;
    pub const GL_MAX_VERTEX_ATOMIC_COUNTERS     : GLenum = 0x92D2;
    pub const GL_MAX_TESS_CONTROL_ATOMIC_COUNTERS : GLenum = 0x92D3;
    pub const GL_MAX_TESS_EVALUATION_ATOMIC_COUNTERS : GLenum = 0x92D4;
    pub const GL_MAX_GEOMETRY_ATOMIC_COUNTERS   : GLenum = 0x92D5;
    pub const GL_MAX_FRAGMENT_ATOMIC_COUNTERS   : GLenum = 0x92D6;
    pub const GL_MAX_COMBINED_ATOMIC_COUNTERS   : GLenum = 0x92D7;
    pub const GL_MAX_ATOMIC_COUNTER_BUFFER_SIZE : GLenum = 0x92D8;
    pub const GL_MAX_ATOMIC_COUNTER_BUFFER_BINDINGS : GLenum = 0x92DC;
    pub const GL_ACTIVE_ATOMIC_COUNTER_BUFFERS  : GLenum = 0x92D9;
    pub const GL_UNIFORM_ATOMIC_COUNTER_BUFFER_INDEX : GLenum = 0x92DA;
    pub const GL_UNSIGNED_INT_ATOMIC_COUNTER    : GLenum = 0x92DB;
}

#[cfg(arb_shader_image_load_store)]
pub mod arb_shader_image_load_store {
    pub const GL_VERTEX_ATTRIB_ARRAY_BARRIER_BIT : GLenum = 0x00000001;
    pub const GL_ELEMENT_ARRAY_BARRIER_BIT      : GLenum = 0x00000002;
    pub const GL_UNIFORM_BARRIER_BIT            : GLenum = 0x00000004;
    pub const GL_TEXTURE_FETCH_BARRIER_BIT      : GLenum = 0x00000008;
    pub const GL_SHADER_IMAGE_ACCESS_BARRIER_BIT : GLenum = 0x00000020;
    pub const GL_COMMAND_BARRIER_BIT            : GLenum = 0x00000040;
    pub const GL_PIXEL_BUFFER_BARRIER_BIT       : GLenum = 0x00000080;
    pub const GL_TEXTURE_UPDATE_BARRIER_BIT     : GLenum = 0x00000100;
    pub const GL_BUFFER_UPDATE_BARRIER_BIT      : GLenum = 0x00000200;
    pub const GL_FRAMEBUFFER_BARRIER_BIT        : GLenum = 0x00000400;
    pub const GL_TRANSFORM_FEEDBACK_BARRIER_BIT : GLenum = 0x00000800;
    pub const GL_ATOMIC_COUNTER_BARRIER_BIT     : GLenum = 0x00001000;
    pub const GL_ALL_BARRIER_BITS               : GLenum = 0xFFFFFFFF;
    pub const GL_MAX_IMAGE_UNITS                : GLenum = 0x8F38;
    pub const GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS : GLenum = 0x8F39;
    pub const GL_IMAGE_BINDING_NAME             : GLenum = 0x8F3A;
    pub const GL_IMAGE_BINDING_LEVEL            : GLenum = 0x8F3B;
    pub const GL_IMAGE_BINDING_LAYERED          : GLenum = 0x8F3C;
    pub const GL_IMAGE_BINDING_LAYER            : GLenum = 0x8F3D;
    pub const GL_IMAGE_BINDING_ACCESS           : GLenum = 0x8F3E;
    pub const GL_IMAGE_1D                       : GLenum = 0x904C;
    pub const GL_IMAGE_2D                       : GLenum = 0x904D;
    pub const GL_IMAGE_3D                       : GLenum = 0x904E;
    pub const GL_IMAGE_2D_RECT                  : GLenum = 0x904F;
    pub const GL_IMAGE_CUBE                     : GLenum = 0x9050;
    pub const GL_IMAGE_BUFFER                   : GLenum = 0x9051;
    pub const GL_IMAGE_1D_ARRAY                 : GLenum = 0x9052;
    pub const GL_IMAGE_2D_ARRAY                 : GLenum = 0x9053;
    pub const GL_IMAGE_CUBE_MAP_ARRAY           : GLenum = 0x9054;
    pub const GL_IMAGE_2D_MULTISAMPLE           : GLenum = 0x9055;
    pub const GL_IMAGE_2D_MULTISAMPLE_ARRAY     : GLenum = 0x9056;
    pub const GL_INT_IMAGE_1D                   : GLenum = 0x9057;
    pub const GL_INT_IMAGE_2D                   : GLenum = 0x9058;
    pub const GL_INT_IMAGE_3D                   : GLenum = 0x9059;
    pub const GL_INT_IMAGE_2D_RECT              : GLenum = 0x905A;
    pub const GL_INT_IMAGE_CUBE                 : GLenum = 0x905B;
    pub const GL_INT_IMAGE_BUFFER               : GLenum = 0x905C;
    pub const GL_INT_IMAGE_1D_ARRAY             : GLenum = 0x905D;
    pub const GL_INT_IMAGE_2D_ARRAY             : GLenum = 0x905E;
    pub const GL_INT_IMAGE_CUBE_MAP_ARRAY       : GLenum = 0x905F;
    pub const GL_INT_IMAGE_2D_MULTISAMPLE       : GLenum = 0x9060;
    pub const GL_INT_IMAGE_2D_MULTISAMPLE_ARRAY : GLenum = 0x9061;
    pub const GL_UNSIGNED_INT_IMAGE_1D          : GLenum = 0x9062;
    pub const GL_UNSIGNED_INT_IMAGE_2D          : GLenum = 0x9063;
    pub const GL_UNSIGNED_INT_IMAGE_3D          : GLenum = 0x9064;
    pub const GL_UNSIGNED_INT_IMAGE_2D_RECT     : GLenum = 0x9065;
    pub const GL_UNSIGNED_INT_IMAGE_CUBE        : GLenum = 0x9066;
    pub const GL_UNSIGNED_INT_IMAGE_BUFFER      : GLenum = 0x9067;
    pub const GL_UNSIGNED_INT_IMAGE_1D_ARRAY    : GLenum = 0x9068;
    pub const GL_UNSIGNED_INT_IMAGE_2D_ARRAY    : GLenum = 0x9069;
    pub const GL_UNSIGNED_INT_IMAGE_CUBE_MAP_ARRAY : GLenum = 0x906A;
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE : GLenum = 0x906B;
    pub const GL_UNSIGNED_INT_IMAGE_2D_MULTISAMPLE_ARRAY : GLenum = 0x906C;
    pub const GL_MAX_IMAGE_SAMPLES              : GLenum = 0x906D;
    pub const GL_IMAGE_BINDING_FORMAT           : GLenum = 0x906E;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_TYPE : GLenum = 0x90C7;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_SIZE : GLenum = 0x90C8;
    pub const GL_IMAGE_FORMAT_COMPATIBILITY_BY_CLASS : GLenum = 0x90C9;
    pub const GL_MAX_VERTEX_IMAGE_UNIFORMS      : GLenum = 0x90CA;
    pub const GL_MAX_TESS_CONTROL_IMAGE_UNIFORMS : GLenum = 0x90CB;
    pub const GL_MAX_TESS_EVALUATION_IMAGE_UNIFORMS : GLenum = 0x90CC;
    pub const GL_MAX_GEOMETRY_IMAGE_UNIFORMS    : GLenum = 0x90CD;
    pub const GL_MAX_FRAGMENT_IMAGE_UNIFORMS    : GLenum = 0x90CE;
    pub const GL_MAX_COMBINED_IMAGE_UNIFORMS    : GLenum = 0x90CF;
}

#[cfg(arb_shading_language_packing)]
pub mod arb_shading_language_packing {
}

#[cfg(arb_texture_storage)]
pub mod arb_texture_storage {
    pub const GL_TEXTURE_IMMUTABLE_FORMAT       : GLenum = 0x912F;
}

#[cfg(khr_texture_compression_astc_ldr)]
pub mod khr_texture_compression_astc_ldr {
    pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR   : GLenum = 0x93B0;
    pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR   : GLenum = 0x93B1;
    pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR   : GLenum = 0x93B2;
    pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR   : GLenum = 0x93B3;
    pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR   : GLenum = 0x93B4;
    pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR   : GLenum = 0x93B5;
    pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR   : GLenum = 0x93B6;
    pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR   : GLenum = 0x93B7;
    pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR  : GLenum = 0x93B8;
    pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR  : GLenum = 0x93B9;
    pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR  : GLenum = 0x93BA;
    pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR : GLenum = 0x93BB;
    pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR : GLenum = 0x93BC;
    pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR : GLenum = 0x93BD;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR : GLenum = 0x93D0;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR : GLenum = 0x93D1;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR : GLenum = 0x93D2;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR : GLenum = 0x93D3;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR : GLenum = 0x93D4;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR : GLenum = 0x93D5;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR : GLenum = 0x93D6;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR : GLenum = 0x93D7;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR : GLenum = 0x93D8;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR : GLenum = 0x93D9;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR : GLenum = 0x93DA;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR : GLenum = 0x93DB;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR : GLenum = 0x93DC;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR : GLenum = 0x93DD;
}

#[cfg(khr_debug)]
pub mod khr_debug {
    pub const GL_DEBUG_OUTPUT_SYNCHRONOUS       : GLenum = 0x8242;
    pub const GL_DEBUG_NEXT_LOGGED_MESSAGE_LENGTH : GLenum = 0x8243;
    pub const GL_DEBUG_CALLBACK_FUNCTION        : GLenum = 0x8244;
    pub const GL_DEBUG_CALLBACK_USER_PARAM      : GLenum = 0x8245;
    pub const GL_DEBUG_SOURCE_API               : GLenum = 0x8246;
    pub const GL_DEBUG_SOURCE_WINDOW_SYSTEM     : GLenum = 0x8247;
    pub const GL_DEBUG_SOURCE_SHADER_COMPILER   : GLenum = 0x8248;
    pub const GL_DEBUG_SOURCE_THIRD_PARTY       : GLenum = 0x8249;
    pub const GL_DEBUG_SOURCE_APPLICATION       : GLenum = 0x824A;
    pub const GL_DEBUG_SOURCE_OTHER             : GLenum = 0x824B;
    pub const GL_DEBUG_TYPE_ERROR               : GLenum = 0x824C;
    pub const GL_DEBUG_TYPE_DEPRECATED_BEHAVIOR : GLenum = 0x824D;
    pub const GL_DEBUG_TYPE_UNDEFINED_BEHAVIOR  : GLenum = 0x824E;
    pub const GL_DEBUG_TYPE_PORTABILITY         : GLenum = 0x824F;
    pub const GL_DEBUG_TYPE_PERFORMANCE         : GLenum = 0x8250;
    pub const GL_DEBUG_TYPE_OTHER               : GLenum = 0x8251;
    pub const GL_DEBUG_TYPE_MARKER              : GLenum = 0x8268;
    pub const GL_DEBUG_TYPE_PUSH_GROUP          : GLenum = 0x8269;
    pub const GL_DEBUG_TYPE_POP_GROUP           : GLenum = 0x826A;
    pub const GL_DEBUG_SEVERITY_NOTIFICATION    : GLenum = 0x826B;
    pub const GL_MAX_DEBUG_GROUP_STACK_DEPTH    : GLenum = 0x826C;
    pub const GL_DEBUG_GROUP_STACK_DEPTH        : GLenum = 0x826D;
    pub const GL_BUFFER                         : GLenum = 0x82E0;
    pub const GL_SHADER                         : GLenum = 0x82E1;
    pub const GL_PROGRAM                        : GLenum = 0x82E2;
    pub const GL_QUERY                          : GLenum = 0x82E3;
    pub const GL_PROGRAM_PIPELINE               : GLenum = 0x82E4;
    pub const GL_SAMPLER                        : GLenum = 0x82E6;
    pub const GL_DISPLAY_LIST                   : GLenum = 0x82E7;
    /* DISPLAY_LIST used in compatibility profile only */
    pub const GL_MAX_LABEL_LENGTH               : GLenum = 0x82E8;
    pub const GL_MAX_DEBUG_MESSAGE_LENGTH       : GLenum = 0x9143;
    pub const GL_MAX_DEBUG_LOGGED_MESSAGES      : GLenum = 0x9144;
    pub const GL_DEBUG_LOGGED_MESSAGES          : GLenum = 0x9145;
    pub const GL_DEBUG_SEVERITY_HIGH            : GLenum = 0x9146;
    pub const GL_DEBUG_SEVERITY_MEDIUM          : GLenum = 0x9147;
    pub const GL_DEBUG_SEVERITY_LOW             : GLenum = 0x9148;
    pub const GL_DEBUG_OUTPUT                   : GLenum = 0x92E0;
    pub const GL_CONTEXT_FLAG_DEBUG_BIT         : GLenum = 0x00000002;
    /* reuse GL_STACK_UNDERFLOW */
    /* reuse GL_STACK_OVERFLOW */
}

#[cfg(arb_arrays_of_arrays)]
pub mod arb_arrays_of_arrays {
}

#[cfg(arb_clear_buffer_object)]
pub mod arb_clear_buffer_object {
}

#[cfg(arb_compute_shader)]
pub mod arb_compute_shader {
    pub const GL_COMPUTE_SHADER                 : GLenum = 0x91B9;
    pub const GL_MAX_COMPUTE_UNIFORM_BLOCKS     : GLenum = 0x91BB;
    pub const GL_MAX_COMPUTE_TEXTURE_IMAGE_UNITS : GLenum = 0x91BC;
    pub const GL_MAX_COMPUTE_IMAGE_UNIFORMS     : GLenum = 0x91BD;
    pub const GL_MAX_COMPUTE_SHARED_MEMORY_SIZE : GLenum = 0x8262;
    pub const GL_MAX_COMPUTE_UNIFORM_COMPONENTS : GLenum = 0x8263;
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTER_BUFFERS : GLenum = 0x8264;
    pub const GL_MAX_COMPUTE_ATOMIC_COUNTERS    : GLenum = 0x8265;
    pub const GL_MAX_COMBINED_COMPUTE_UNIFORM_COMPONENTS : GLenum = 0x8266;
    pub const GL_MAX_COMPUTE_LOCAL_INVOCATIONS  : GLenum = 0x90EB;
    pub const GL_MAX_COMPUTE_WORK_GROUP_COUNT   : GLenum = 0x91BE;
    pub const GL_MAX_COMPUTE_WORK_GROUP_SIZE    : GLenum = 0x91BF;
    pub const GL_COMPUTE_LOCAL_WORK_SIZE        : GLenum = 0x8267;
    pub const GL_UNIFORM_BLOCK_REFERENCED_BY_COMPUTE_SHADER : GLenum = 0x90EC;
    pub const GL_ATOMIC_COUNTER_BUFFER_REFERENCED_BY_COMPUTE_SHADER : GLenum = 0x90ED;
    pub const GL_DISPATCH_INDIRECT_BUFFER       : GLenum = 0x90EE;
    pub const GL_DISPATCH_INDIRECT_BUFFER_BINDING : GLenum = 0x90EF;
    pub const GL_COMPUTE_SHADER_BIT             : GLenum = 0x00000020;
}

#[cfg(arb_copy_image)]
pub mod arb_copy_image {
}

#[cfg(arb_texture_view)]
pub mod arb_texture_view {
    pub const GL_TEXTURE_VIEW_MIN_LEVEL         : GLenum = 0x82DB;
    pub const GL_TEXTURE_VIEW_NUM_LEVELS        : GLenum = 0x82DC;
    pub const GL_TEXTURE_VIEW_MIN_LAYER         : GLenum = 0x82DD;
    pub const GL_TEXTURE_VIEW_NUM_LAYERS        : GLenum = 0x82DE;
    pub const GL_TEXTURE_IMMUTABLE_LEVELS       : GLenum = 0x82DF;
}

#[cfg(arb_vertex_attrib_binding)]
pub mod arb_vertex_attrib_binding {
    pub const GL_VERTEX_ATTRIB_BINDING          : GLenum = 0x82D4;
    pub const GL_VERTEX_ATTRIB_RELATIVE_OFFSET  : GLenum = 0x82D5;
    pub const GL_VERTEX_BINDING_DIVISOR         : GLenum = 0x82D6;
    pub const GL_VERTEX_BINDING_OFFSET          : GLenum = 0x82D7;
    pub const GL_VERTEX_BINDING_STRIDE          : GLenum = 0x82D8;
    pub const GL_MAX_VERTEX_ATTRIB_RELATIVE_OFFSET : GLenum = 0x82D9;
    pub const GL_MAX_VERTEX_ATTRIB_BINDINGS     : GLenum = 0x82DA;
}

#[cfg(arb_robustness_isolation)]
pub mod arb_robustness_isolation {
}

#[cfg(arb_es3_compatibility)]
pub mod arb_es3_compatibility {
    pub const GL_COMPRESSED_RGB8_ETC2           : GLenum = 0x9274;
    pub const GL_COMPRESSED_SRGB8_ETC2          : GLenum = 0x9275;
    pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 : GLenum = 0x9276;
    pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 : GLenum = 0x9277;
    pub const GL_COMPRESSED_RGBA8_ETC2_EAC      : GLenum = 0x9278;
    pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC : GLenum = 0x9279;
    pub const GL_COMPRESSED_R11_EAC             : GLenum = 0x9270;
    pub const GL_COMPRESSED_SIGNED_R11_EAC      : GLenum = 0x9271;
    pub const GL_COMPRESSED_RG11_EAC            : GLenum = 0x9272;
    pub const GL_COMPRESSED_SIGNED_RG11_EAC     : GLenum = 0x9273;
    pub const GL_PRIMITIVE_RESTART_FIXED_INDEX  : GLenum = 0x8D69;
    pub const GL_ANY_SAMPLES_PASSED_CONSERVATIVE : GLenum = 0x8D6A;
    pub const GL_MAX_ELEMENT_INDEX              : GLenum = 0x8D6B;
}

#[cfg(arb_explicit_uniform_location)]
pub mod arb_explicit_uniform_location {
    pub const GL_MAX_UNIFORM_LOCATIONS          : GLenum = 0x826E;
}

#[cfg(arb_fragment_layer_viewport)]
pub mod arb_fragment_layer_viewport {
}

#[cfg(arb_framebuffer_no_attachments)]
pub mod arb_framebuffer_no_attachments {
    pub const GL_FRAMEBUFFER_DEFAULT_WIDTH      : GLenum = 0x9310;
    pub const GL_FRAMEBUFFER_DEFAULT_HEIGHT     : GLenum = 0x9311;
    pub const GL_FRAMEBUFFER_DEFAULT_LAYERS     : GLenum = 0x9312;
    pub const GL_FRAMEBUFFER_DEFAULT_SAMPLES    : GLenum = 0x9313;
    pub const GL_FRAMEBUFFER_DEFAULT_FIXED_SAMPLE_LOCATIONS : GLenum = 0x9314;
    pub const GL_MAX_FRAMEBUFFER_WIDTH          : GLenum = 0x9315;
    pub const GL_MAX_FRAMEBUFFER_HEIGHT         : GLenum = 0x9316;
    pub const GL_MAX_FRAMEBUFFER_LAYERS         : GLenum = 0x9317;
    pub const GL_MAX_FRAMEBUFFER_SAMPLES        : GLenum = 0x9318;
}

#[cfg(arb_internalformat_query2)]
pub mod arb_internalformat_query2 {
    /* reuse GL_IMAGE_FORMAT_COMPATIBILITY_TYPE */
    /* reuse GL_NUM_SAMPLE_COUNTS */
    /* reuse GL_RENDERBUFFER */
    /* reuse GL_SAMPLES */
    /* reuse GL_TEXTURE_1D */
    /* reuse GL_TEXTURE_1D_ARRAY */
    /* reuse GL_TEXTURE_2D */
    /* reuse GL_TEXTURE_2D_ARRAY */
    /* reuse GL_TEXTURE_3D */
    /* reuse GL_TEXTURE_CUBE_MAP */
    /* reuse GL_TEXTURE_CUBE_MAP_ARRAY */
    /* reuse GL_TEXTURE_RECTANGLE */
    /* reuse GL_TEXTURE_BUFFER */
    /* reuse GL_TEXTURE_2D_MULTISAMPLE */
    /* reuse GL_TEXTURE_2D_MULTISAMPLE_ARRAY */
    /* reuse GL_TEXTURE_COMPRESSED */
    pub const GL_INTERNALFORMAT_SUPPORTED       : GLenum = 0x826F;
    pub const GL_INTERNALFORMAT_PREFERRED       : GLenum = 0x8270;
    pub const GL_INTERNALFORMAT_RED_SIZE        : GLenum = 0x8271;
    pub const GL_INTERNALFORMAT_GREEN_SIZE      : GLenum = 0x8272;
    pub const GL_INTERNALFORMAT_BLUE_SIZE       : GLenum = 0x8273;
    pub const GL_INTERNALFORMAT_ALPHA_SIZE      : GLenum = 0x8274;
    pub const GL_INTERNALFORMAT_DEPTH_SIZE      : GLenum = 0x8275;
    pub const GL_INTERNALFORMAT_STENCIL_SIZE    : GLenum = 0x8276;
    pub const GL_INTERNALFORMAT_SHARED_SIZE     : GLenum = 0x8277;
    pub const GL_INTERNALFORMAT_RED_TYPE        : GLenum = 0x8278;
    pub const GL_INTERNALFORMAT_GREEN_TYPE      : GLenum = 0x8279;
    pub const GL_INTERNALFORMAT_BLUE_TYPE       : GLenum = 0x827A;
    pub const GL_INTERNALFORMAT_ALPHA_TYPE      : GLenum = 0x827B;
    pub const GL_INTERNALFORMAT_DEPTH_TYPE      : GLenum = 0x827C;
    pub const GL_INTERNALFORMAT_STENCIL_TYPE    : GLenum = 0x827D;
    pub const GL_MAX_WIDTH                      : GLenum = 0x827E;
    pub const GL_MAX_HEIGHT                     : GLenum = 0x827F;
    pub const GL_MAX_DEPTH                      : GLenum = 0x8280;
    pub const GL_MAX_LAYERS                     : GLenum = 0x8281;
    pub const GL_MAX_COMBINED_DIMENSIONS        : GLenum = 0x8282;
    pub const GL_COLOR_COMPONENTS               : GLenum = 0x8283;
    pub const GL_DEPTH_COMPONENTS               : GLenum = 0x8284;
    pub const GL_STENCIL_COMPONENTS             : GLenum = 0x8285;
    pub const GL_COLOR_RENDERABLE               : GLenum = 0x8286;
    pub const GL_DEPTH_RENDERABLE               : GLenum = 0x8287;
    pub const GL_STENCIL_RENDERABLE             : GLenum = 0x8288;
    pub const GL_FRAMEBUFFER_RENDERABLE         : GLenum = 0x8289;
    pub const GL_FRAMEBUFFER_RENDERABLE_LAYERED : GLenum = 0x828A;
    pub const GL_FRAMEBUFFER_BLEND              : GLenum = 0x828B;
    pub const GL_READ_PIXELS                    : GLenum = 0x828C;
    pub const GL_READ_PIXELS_FORMAT             : GLenum = 0x828D;
    pub const GL_READ_PIXELS_TYPE               : GLenum = 0x828E;
    pub const GL_TEXTURE_IMAGE_FORMAT           : GLenum = 0x828F;
    pub const GL_TEXTURE_IMAGE_TYPE             : GLenum = 0x8290;
    pub const GL_GET_TEXTURE_IMAGE_FORMAT       : GLenum = 0x8291;
    pub const GL_GET_TEXTURE_IMAGE_TYPE         : GLenum = 0x8292;
    pub const GL_MIPMAP                         : GLenum = 0x8293;
    pub const GL_MANUAL_GENERATE_MIPMAP         : GLenum = 0x8294;
    pub const GL_AUTO_GENERATE_MIPMAP           : GLenum = 0x8295;
    pub const GL_COLOR_ENCODING                 : GLenum = 0x8296;
    pub const GL_SRGB_READ                      : GLenum = 0x8297;
    pub const GL_SRGB_WRITE                     : GLenum = 0x8298;
    pub const GL_SRGB_DECODE_ARB                : GLenum = 0x8299;
    pub const GL_FILTER                         : GLenum = 0x829A;
    pub const GL_VERTEX_TEXTURE                 : GLenum = 0x829B;
    pub const GL_TESS_CONTROL_TEXTURE           : GLenum = 0x829C;
    pub const GL_TESS_EVALUATION_TEXTURE        : GLenum = 0x829D;
    pub const GL_GEOMETRY_TEXTURE               : GLenum = 0x829E;
    pub const GL_FRAGMENT_TEXTURE               : GLenum = 0x829F;
    pub const GL_COMPUTE_TEXTURE                : GLenum = 0x82A0;
    pub const GL_TEXTURE_SHADOW                 : GLenum = 0x82A1;
    pub const GL_TEXTURE_GATHER                 : GLenum = 0x82A2;
    pub const GL_TEXTURE_GATHER_SHADOW          : GLenum = 0x82A3;
    pub const GL_SHADER_IMAGE_LOAD              : GLenum = 0x82A4;
    pub const GL_SHADER_IMAGE_STORE             : GLenum = 0x82A5;
    pub const GL_SHADER_IMAGE_ATOMIC            : GLenum = 0x82A6;
    pub const GL_IMAGE_TEXEL_SIZE               : GLenum = 0x82A7;
    pub const GL_IMAGE_COMPATIBILITY_CLASS      : GLenum = 0x82A8;
    pub const GL_IMAGE_PIXEL_FORMAT             : GLenum = 0x82A9;
    pub const GL_IMAGE_PIXEL_TYPE               : GLenum = 0x82AA;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_TEST : GLenum = 0x82AC;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_TEST : GLenum = 0x82AD;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_DEPTH_WRITE : GLenum = 0x82AE;
    pub const GL_SIMULTANEOUS_TEXTURE_AND_STENCIL_WRITE : GLenum = 0x82AF;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_WIDTH : GLenum = 0x82B1;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_HEIGHT : GLenum = 0x82B2;
    pub const GL_TEXTURE_COMPRESSED_BLOCK_SIZE  : GLenum = 0x82B3;
    pub const GL_CLEAR_BUFFER                   : GLenum = 0x82B4;
    pub const GL_TEXTURE_VIEW                   : GLenum = 0x82B5;
    pub const GL_VIEW_COMPATIBILITY_CLASS       : GLenum = 0x82B6;
    pub const GL_FULL_SUPPORT                   : GLenum = 0x82B7;
    pub const GL_CAVEAT_SUPPORT                 : GLenum = 0x82B8;
    pub const GL_IMAGE_CLASS_4_X_32             : GLenum = 0x82B9;
    pub const GL_IMAGE_CLASS_2_X_32             : GLenum = 0x82BA;
    pub const GL_IMAGE_CLASS_1_X_32             : GLenum = 0x82BB;
    pub const GL_IMAGE_CLASS_4_X_16             : GLenum = 0x82BC;
    pub const GL_IMAGE_CLASS_2_X_16             : GLenum = 0x82BD;
    pub const GL_IMAGE_CLASS_1_X_16             : GLenum = 0x82BE;
    pub const GL_IMAGE_CLASS_4_X_8              : GLenum = 0x82BF;
    pub const GL_IMAGE_CLASS_2_X_8              : GLenum = 0x82C0;
    pub const GL_IMAGE_CLASS_1_X_8              : GLenum = 0x82C1;
    pub const GL_IMAGE_CLASS_11_11_10           : GLenum = 0x82C2;
    pub const GL_IMAGE_CLASS_10_10_10_2         : GLenum = 0x82C3;
    pub const GL_VIEW_CLASS_128_BITS            : GLenum = 0x82C4;
    pub const GL_VIEW_CLASS_96_BITS             : GLenum = 0x82C5;
    pub const GL_VIEW_CLASS_64_BITS             : GLenum = 0x82C6;
    pub const GL_VIEW_CLASS_48_BITS             : GLenum = 0x82C7;
    pub const GL_VIEW_CLASS_32_BITS             : GLenum = 0x82C8;
    pub const GL_VIEW_CLASS_24_BITS             : GLenum = 0x82C9;
    pub const GL_VIEW_CLASS_16_BITS             : GLenum = 0x82CA;
    pub const GL_VIEW_CLASS_8_BITS              : GLenum = 0x82CB;
    pub const GL_VIEW_CLASS_S3TC_DXT1_RGB       : GLenum = 0x82CC;
    pub const GL_VIEW_CLASS_S3TC_DXT1_RGBA      : GLenum = 0x82CD;
    pub const GL_VIEW_CLASS_S3TC_DXT3_RGBA      : GLenum = 0x82CE;
    pub const GL_VIEW_CLASS_S3TC_DXT5_RGBA      : GLenum = 0x82CF;
    pub const GL_VIEW_CLASS_RGTC1_RED           : GLenum = 0x82D0;
    pub const GL_VIEW_CLASS_RGTC2_RG            : GLenum = 0x82D1;
    pub const GL_VIEW_CLASS_BPTC_UNORM          : GLenum = 0x82D2;
    pub const GL_VIEW_CLASS_BPTC_FLOAT          : GLenum = 0x82D3;
}

#[cfg(arb_invalidate_subdata)]
pub mod arb_invalidate_subdata {
}

#[cfg(arb_multi_draw_indirect)]
pub mod arb_multi_draw_indirect {
}

#[cfg(arb_program_interface_query)]
pub mod arb_program_interface_query {
    pub const GL_UNIFORM                        : GLenum = 0x92E1;
    pub const GL_UNIFORM_BLOCK                  : GLenum = 0x92E2;
    pub const GL_PROGRAM_INPUT                  : GLenum = 0x92E3;
    pub const GL_PROGRAM_OUTPUT                 : GLenum = 0x92E4;
    pub const GL_BUFFER_VARIABLE                : GLenum = 0x92E5;
    pub const GL_SHADER_STORAGE_BLOCK           : GLenum = 0x92E6;
    /* reuse GL_ATOMIC_COUNTER_BUFFER */
    pub const GL_VERTEX_SUBROUTINE              : GLenum = 0x92E8;
    pub const GL_TESS_CONTROL_SUBROUTINE        : GLenum = 0x92E9;
    pub const GL_TESS_EVALUATION_SUBROUTINE     : GLenum = 0x92EA;
    pub const GL_GEOMETRY_SUBROUTINE            : GLenum = 0x92EB;
    pub const GL_FRAGMENT_SUBROUTINE            : GLenum = 0x92EC;
    pub const GL_COMPUTE_SUBROUTINE             : GLenum = 0x92ED;
    pub const GL_VERTEX_SUBROUTINE_UNIFORM      : GLenum = 0x92EE;
    pub const GL_TESS_CONTROL_SUBROUTINE_UNIFORM : GLenum = 0x92EF;
    pub const GL_TESS_EVALUATION_SUBROUTINE_UNIFORM : GLenum = 0x92F0;
    pub const GL_GEOMETRY_SUBROUTINE_UNIFORM    : GLenum = 0x92F1;
    pub const GL_FRAGMENT_SUBROUTINE_UNIFORM    : GLenum = 0x92F2;
    pub const GL_COMPUTE_SUBROUTINE_UNIFORM     : GLenum = 0x92F3;
    pub const GL_TRANSFORM_FEEDBACK_VARYING     : GLenum = 0x92F4;
    pub const GL_ACTIVE_RESOURCES               : GLenum = 0x92F5;
    pub const GL_MAX_NAME_LENGTH                : GLenum = 0x92F6;
    pub const GL_MAX_NUM_ACTIVE_VARIABLES       : GLenum = 0x92F7;
    pub const GL_MAX_NUM_COMPATIBLE_SUBROUTINES : GLenum = 0x92F8;
    pub const GL_NAME_LENGTH                    : GLenum = 0x92F9;
    pub const GL_TYPE                           : GLenum = 0x92FA;
    pub const GL_ARRAY_SIZE                     : GLenum = 0x92FB;
    pub const GL_OFFSET                         : GLenum = 0x92FC;
    pub const GL_BLOCK_INDEX                    : GLenum = 0x92FD;
    pub const GL_ARRAY_STRIDE                   : GLenum = 0x92FE;
    pub const GL_MATRIX_STRIDE                  : GLenum = 0x92FF;
    pub const GL_IS_ROW_MAJOR                   : GLenum = 0x9300;
    pub const GL_ATOMIC_COUNTER_BUFFER_INDEX    : GLenum = 0x9301;
    pub const GL_BUFFER_BINDING                 : GLenum = 0x9302;
    pub const GL_BUFFER_DATA_SIZE               : GLenum = 0x9303;
    pub const GL_NUM_ACTIVE_VARIABLES           : GLenum = 0x9304;
    pub const GL_ACTIVE_VARIABLES               : GLenum = 0x9305;
    pub const GL_REFERENCED_BY_VERTEX_SHADER    : GLenum = 0x9306;
    pub const GL_REFERENCED_BY_TESS_CONTROL_SHADER : GLenum = 0x9307;
    pub const GL_REFERENCED_BY_TESS_EVALUATION_SHADER : GLenum = 0x9308;
    pub const GL_REFERENCED_BY_GEOMETRY_SHADER  : GLenum = 0x9309;
    pub const GL_REFERENCED_BY_FRAGMENT_SHADER  : GLenum = 0x930A;
    pub const GL_REFERENCED_BY_COMPUTE_SHADER   : GLenum = 0x930B;
    pub const GL_TOP_LEVEL_ARRAY_SIZE           : GLenum = 0x930C;
    pub const GL_TOP_LEVEL_ARRAY_STRIDE         : GLenum = 0x930D;
    pub const GL_LOCATION                       : GLenum = 0x930E;
    pub const GL_LOCATION_INDEX                 : GLenum = 0x930F;
    pub const GL_IS_PER_PATCH                   : GLenum = 0x92E7;
    /* reuse GL_NUM_COMPATIBLE_SUBROUTINES */
    /* reuse GL_COMPATIBLE_SUBROUTINES */
}

#[cfg(arb_robust_buffer_access_behavior)]
pub mod arb_robust_buffer_access_behavior {
}

#[cfg(arb_shader_image_size)]
pub mod arb_shader_image_size {
}

#[cfg(arb_shader_storage_buffer_object)]
pub mod arb_shader_storage_buffer_object {
    pub const GL_SHADER_STORAGE_BUFFER          : GLenum = 0x90D2;
    pub const GL_SHADER_STORAGE_BUFFER_BINDING  : GLenum = 0x90D3;
    pub const GL_SHADER_STORAGE_BUFFER_START    : GLenum = 0x90D4;
    pub const GL_SHADER_STORAGE_BUFFER_SIZE     : GLenum = 0x90D5;
    pub const GL_MAX_VERTEX_SHADER_STORAGE_BLOCKS : GLenum = 0x90D6;
    pub const GL_MAX_GEOMETRY_SHADER_STORAGE_BLOCKS : GLenum = 0x90D7;
    pub const GL_MAX_TESS_CONTROL_SHADER_STORAGE_BLOCKS : GLenum = 0x90D8;
    pub const GL_MAX_TESS_EVALUATION_SHADER_STORAGE_BLOCKS : GLenum = 0x90D9;
    pub const GL_MAX_FRAGMENT_SHADER_STORAGE_BLOCKS : GLenum = 0x90DA;
    pub const GL_MAX_COMPUTE_SHADER_STORAGE_BLOCKS : GLenum = 0x90DB;
    pub const GL_MAX_COMBINED_SHADER_STORAGE_BLOCKS : GLenum = 0x90DC;
    pub const GL_MAX_SHADER_STORAGE_BUFFER_BINDINGS : GLenum = 0x90DD;
    pub const GL_MAX_SHADER_STORAGE_BLOCK_SIZE  : GLenum = 0x90DE;
    pub const GL_SHADER_STORAGE_BUFFER_OFFSET_ALIGNMENT : GLenum = 0x90DF;
    pub const GL_SHADER_STORAGE_BARRIER_BIT     : GLenum = 0x2000;
    pub const GL_MAX_COMBINED_SHADER_OUTPUT_RESOURCES : GLenum = GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS;
    /* reuse GL_MAX_COMBINED_IMAGE_UNITS_AND_FRAGMENT_OUTPUTS */
}

#[cfg(arb_stencil_texturing)]
pub mod arb_stencil_texturing {
    pub const GL_DEPTH_STENCIL_TEXTURE_MODE     : GLenum = 0x90EA;
}

#[cfg(arb_texture_buffer_range)]
pub mod arb_texture_buffer_range {
    pub const GL_TEXTURE_BUFFER_OFFSET          : GLenum = 0x919D;
    pub const GL_TEXTURE_BUFFER_SIZE            : GLenum = 0x919E;
    pub const GL_TEXTURE_BUFFER_OFFSET_ALIGNMENT : GLenum = 0x919F;
}

#[cfg(arb_texture_query_levels)]
pub mod arb_texture_query_levels {
}

#[cfg(arb_texture_storage_multisample)]
pub mod arb_texture_storage_multisample {
}