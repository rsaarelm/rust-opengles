/* automatically generated by rust-bindgen */

import libc::*;
import libc::types::common::c99::*;
import ptr::addr_of;
import str::{as_c_str, from_bytes};
import sys::size_of;
import unsafe::reinterpret_cast;
import vec::from_elem;
import vec::unsafe::to_ptr;

// Constants

/* BeginMode */
const POINTS:         c_uint = 0x0000 as c_uint;
const LINES:          c_uint = 0x0001 as c_uint;
const LINE_LOOP:      c_uint = 0x0002 as c_uint;
const LINE_STRIP:     c_uint = 0x0003 as c_uint;
const TRIANGLES:      c_uint = 0x0004 as c_uint;
const TRIANGLE_STRIP: c_uint = 0x0005 as c_uint;
const TRIANGLE_FAN:   c_uint = 0x0006 as c_uint;

const DEPTH_BUFFER_BIT:   c_uint = 0x00000100 as c_uint;
const STENCIL_BUFFER_BIT: c_uint = 0x00000400 as c_uint;
const COLOR_BUFFER_BIT:   c_uint = 0x00004000 as c_uint;

const NO_ERROR: c_uint = 0 as c_uint;

/* DataType */
const BYTE:           c_uint = 0x1400 as c_uint;
const UNSIGNED_BYTE:  c_uint = 0x1401 as c_uint;
const SHORT:          c_uint = 0x1402 as c_uint;
const UNSIGNED_SHORT: c_uint = 0x1403 as c_uint;
const INT:            c_uint = 0x1404 as c_uint;
const UNSIGNED_INT:   c_uint = 0x1405 as c_uint;
const FLOAT:          c_uint = 0x1406 as c_uint;
const FIXED:          c_uint = 0x140C as c_uint;

/* Shaders */
const FRAGMENT_SHADER:                  c_uint = 0x8B30 as c_uint;
const VERTEX_SHADER:                    c_uint = 0x8B31 as c_uint;
const MAX_VERTEX_ATTRIBS:               c_uint = 0x8869 as c_uint;
const MAX_VERTEX_UNIFORM_VECTORS:       c_uint = 0x8DFB as c_uint;
const MAX_VARYING_VECTORS:              c_uint = 0x8DFC as c_uint;
const MAX_COMBINED_TEXTURE_IMAGE_UNITS: c_uint = 0x8B4D as c_uint;
const MAX_VERTEX_TEXTURE_IMAGE_UNITS:   c_uint = 0x8B4C as c_uint;
const MAX_TEXTURE_IMAGE_UNITS:          c_uint = 0x8872 as c_uint;
const MAX_FRAGMENT_UNIFORM_VECTORS:     c_uint = 0x8DFD as c_uint;
const SHADER_TYPE:                      c_uint = 0x8B4F as c_uint;
const DELETE_STATUS:                    c_uint = 0x8B80 as c_uint;
const LINK_STATUS:                      c_uint = 0x8B82 as c_uint;
const VALIDATE_STATUS:                  c_uint = 0x8B83 as c_uint;
const ATTACHED_SHADERS:                 c_uint = 0x8B85 as c_uint;
const ACTIVE_UNIFORMS:                  c_uint = 0x8B86 as c_uint;
const ACTIVE_UNIFORM_MAX_LENGTH:        c_uint = 0x8B87 as c_uint;
const ACTIVE_ATTRIBUTES:                c_uint = 0x8B89 as c_uint;
const ACTIVE_ATTRIBUTE_MAX_LENGTH:      c_uint = 0x8B8A as c_uint;
const SHADING_LANGUAGE_VERSION:         c_uint = 0x8B8C as c_uint;
const CURRENT_PROGRAM:                  c_uint = 0x8B8D as c_uint;

/* Shader Source */
const COMPILE_STATUS:       c_uint = 0x8B81 as c_uint;
const INFO_LOG_LENGTH:      c_uint = 0x8B84 as c_uint;
const SHADER_SOURCE_LENGTH: c_uint = 0x8B88 as c_uint;
const SHADER_COMPILER:      c_uint = 0x8DFA as c_uint;

/* Buffer Objects */
const ARRAY_BUFFER:                 c_uint = 0x8892 as c_uint;
const ELEMENT_ARRAY_BUFFER:         c_uint = 0x8893 as c_uint;
const ARRAY_BUFFER_BINDING:         c_uint = 0x8894 as c_uint;
const ELEMENT_ARRAY_BUFFER_BINDING: c_uint = 0x8895 as c_uint;

const STREAM_DRAW:  c_uint = 0x88E0 as c_uint;
const STATIC_DRAW:  c_uint = 0x88E4 as c_uint;
const DYNAMIC_DRAW: c_uint = 0x88E8 as c_uint;

// Types

type GLvoid = c_void /* unknown kind Void */;

type GLchar = c_char;

type GLenum = c_uint;

type GLboolean = c_uchar;

type GLbitfield = c_uint;

type GLbyte = int8_t;

type GLshort = c_short;

type GLint = c_int;

type GLsizei = c_int;

type GLubyte = uint8_t;

type GLushort = c_ushort;

type GLuint = c_uint;

type GLfloat = f32;

type GLclampf = f32;

type GLfixed = int32_t;

type GLintptr = intptr_t;

type GLsizeiptr = ssize_t;


// Helper functions

fn destroy<T>(-_x: T) {
    // Just let the object drop.
}


// Exposed Rust API using Rust naming conventions

fn attach_shader(program: GLuint, shader: GLuint) {
    ll::glAttachShader(program, shader);
}

fn bind_buffer(target: GLenum, buffer: GLuint) {
    ll::glBindBuffer(target, buffer);
}

// FIXME: There should be some type-safe wrapper for this...
fn buffer_data<T>(target: GLenum, data: ~[T], usage: GLenum) unsafe {
    ll::glBufferData(target, (data.len() * size_of::<T>()) as GLsizeiptr,
                     to_ptr(data) as *GLvoid, usage);
}

fn clear(mask: GLbitfield) {
    ll::glClear(mask);
}

fn clear_color(red: GLclampf, green: GLclampf, blue: GLclampf, alpha: GLclampf) {
    ll::glClearColor(red, green, blue, alpha);
}

fn compile_shader(shader: GLuint) {
    ll::glCompileShader(shader);
}

fn create_program() -> GLuint {
    ret ll::glCreateProgram();
}

fn create_shader(shader_type: GLenum) -> GLuint {
    ret ll::glCreateShader(shader_type);
}

fn draw_arrays(mode: GLenum, first: GLint, count: GLsizei) {
    ret ll::glDrawArrays(mode, first, count);
}

fn draw_elements(mode: GLenum, element_type: GLenum, indices: ~[u8]) unsafe {
    ret ll::glDrawElements(mode, indices.len() as GLsizei, element_type,
                           to_ptr(indices) as *c_void);
}

fn enable(cap: GLenum) {
    ll::glEnable(cap);
}

fn enable_vertex_attrib_array(index: GLuint) {
    ll::glEnableVertexAttribArray(index);
}

fn finish() {
    ret ll::glFinish();
}

fn flush() {
    ret ll::glFlush();
}

fn gen_buffers(n: GLsizei) -> ~[GLuint] unsafe {
    let result = from_elem(n as uint, 0 as GLuint);
    ll::glGenBuffers(n, to_ptr(result));
    ret result;
}

fn get_attrib_location(program: GLuint, name: str) -> c_int unsafe {
    ret as_c_str(name, |name_bytes|
        ll::glGetAttribLocation(program, name_bytes as *GLchar));
}

fn get_error() -> GLenum {
    ret ll::glGetError();
}

fn get_program_iv(program: GLuint, pname: GLenum) -> GLint unsafe {
    let result: GLint = 0 as GLint;
    ll::glGetProgramiv(program, pname, addr_of(result));
    ret result;
}

fn get_shader_info_log(shader: GLuint) -> str unsafe {
    let result = from_elem(1024u, 0u8);
    let result_len: GLsizei = 0 as GLsizei;
    ll::glGetShaderInfoLog(shader, 1024 as GLsizei, addr_of(result_len),
                           to_ptr(result) as *GLchar);
    ret from_bytes(result);
}

fn get_shader_iv(shader: GLuint, pname: GLenum) -> GLint unsafe {
    let result: GLint = 0 as GLint;
    ll::glGetShaderiv(shader, pname, addr_of(result));
    ret result;
}

fn get_uniform_location(program: GLuint, name: str) -> c_int unsafe {
    ret as_c_str(name, |name_bytes|
        ll::glGetUniformLocation(program, name_bytes as *GLchar));
}

fn link_program(program: GLuint) {
    ret ll::glLinkProgram(program);
}

fn shader_source(shader: GLuint, strings: ~[~[u8]]) unsafe {
    let pointers = strings.map(|string| to_ptr(string));
    let lengths = strings.map(|string| string.len() as GLint);
    ll::glShaderSource(shader, pointers.len() as GLsizei,
                       to_ptr(pointers) as **GLchar, to_ptr(lengths));
    destroy(lengths);
    destroy(pointers);
}

fn use_program(program: GLuint) {
    ll::glUseProgram(program);
}

fn vertex_attrib_pointer_f32(index: GLuint, size: GLint, normalized: bool,
                             stride: GLsizei, offset: GLuint) unsafe {
    ll::glVertexAttribPointer(index, size, FLOAT, normalized as GLboolean,
                              stride, reinterpret_cast(offset as uint));
}


#[nolink]
#[link_args="-framework OpenGL"]
extern mod ll {

// Lower-level API

fn glActiveTexture(++texture: GLenum);

fn glAttachShader(++program: GLuint, ++shader: GLuint);

fn glBindAttribLocation(++program: GLuint, ++index: GLuint, ++name: *GLchar);

fn glBindBuffer(++target: GLenum, ++buffer: GLuint);

fn glBindFramebuffer(++target: GLenum, ++framebuffer: GLuint);

fn glBindRenderbuffer(++target: GLenum, ++renderbuffer: GLuint);

fn glBindTexture(++target: GLenum, ++texture: GLuint);

fn glBlendColor(++red: GLclampf, ++green: GLclampf, ++blue: GLclampf, ++alpha: GLclampf);

fn glBlendEquation(++mode: GLenum);

fn glBlendEquationSeparate(++modeRGB: GLenum, ++modeAlpha: GLenum);

fn glBlendFunc(++sfactor: GLenum, ++dfactor: GLenum);

fn glBlendFuncSeparate(++srcRGB: GLenum, ++dstRGB: GLenum, ++srcAlpha: GLenum, ++dstAlpha: GLenum);

fn glBufferData(++target: GLenum, ++size: GLsizeiptr, ++data: *GLvoid, ++usage: GLenum);

fn glBufferSubData(++target: GLenum, ++offset: GLintptr, ++size: GLsizeiptr, ++data: *GLvoid);

fn glCheckFramebufferStatus(++target: GLenum) -> GLenum;

fn glClear(++mask: GLbitfield);

fn glClearColor(++red: GLclampf, ++green: GLclampf, ++blue: GLclampf, ++alpha: GLclampf);

// Unsupported on Mac:
//fn glClearDepthf(++depth: GLclampf);

fn glClearStencil(++s: GLint);

fn glColorMask(++red: GLboolean, ++green: GLboolean, ++blue: GLboolean, ++alpha: GLboolean);

fn glCompileShader(++shader: GLuint);

fn glCompressedTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei, ++border: GLint, ++imageSize: GLsizei, ++data: *GLvoid);

fn glCompressedTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++imageSize: GLsizei, ++data: *GLvoid);

fn glCopyTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLenum, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++border: GLint);

fn glCopyTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);

fn glCreateProgram() -> GLuint;

fn glCreateShader(++_type: GLenum) -> GLuint;

fn glCullFace(++mode: GLenum);

fn glDeleteBuffers(++n: GLsizei, ++buffers: *GLuint);

fn glDeleteFramebuffers(++n: GLsizei, ++framebuffers: *GLuint);

fn glDeleteProgram(++program: GLuint);

fn glDeleteRenderbuffers(++n: GLsizei, ++renderbuffers: *GLuint);

fn glDeleteShader(++shader: GLuint);

fn glDeleteTextures(++n: GLsizei, ++textures: *GLuint);

fn glDepthFunc(++func: GLenum);

fn glDepthMask(++flag: GLboolean);

// Unsupported on Mac:
//fn glDepthRangef(++zNear: GLclampf, ++zFar: GLclampf);

fn glDetachShader(++program: GLuint, ++shader: GLuint);

fn glDisable(++cap: GLenum);

fn glDisableVertexAttribArray(++index: GLuint);

fn glDrawArrays(++mode: GLenum, ++first: GLint, ++count: GLsizei);

fn glDrawElements(++mode: GLenum, ++count: GLsizei, ++_type: GLenum, ++indices: *GLvoid);

fn glEnable(++cap: GLenum);

fn glEnableVertexAttribArray(++index: GLuint);

fn glFinish();

fn glFlush();

fn glFramebufferRenderbuffer(++target: GLenum, ++attachment: GLenum, ++renderbuffertarget: GLenum, ++renderbuffer: GLuint);

fn glFramebufferTexture2D(++target: GLenum, ++attachment: GLenum, ++textarget: GLenum, ++texture: GLuint, ++level: GLint);

fn glFrontFace(++mode: GLenum);

fn glGenBuffers(++n: GLsizei, ++buffers: *GLuint);

fn glGenerateMipmap(++target: GLenum);

fn glGenFramebuffers(++n: GLsizei, ++framebuffers: *GLuint);

fn glGenRenderbuffers(++n: GLsizei, ++renderbuffers: *GLuint);

fn glGenTextures(++n: GLsizei, ++textures: *GLuint);

fn glGetActiveAttrib(++program: GLuint, ++index: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++size: *GLint, ++_type: *GLenum, ++name: *GLchar);

fn glGetActiveUniform(++program: GLuint, ++index: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++size: *GLint, ++_type: *GLenum, ++name: *GLchar);

fn glGetAttachedShaders(++program: GLuint, ++maxcount: GLsizei, ++count: *GLsizei, ++shaders: *GLuint);

fn glGetAttribLocation(++program: GLuint, ++name: *GLchar) -> c_int;

fn glGetBooleanv(++pname: GLenum, ++params: *GLboolean);

fn glGetBufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

fn glGetError() -> GLenum;

fn glGetFloatv(++pname: GLenum, ++params: *GLfloat);

fn glGetFramebufferAttachmentParameteriv(++target: GLenum, ++attachment: GLenum, ++pname: GLenum, ++params: *GLint);

fn glGetIntegerv(++pname: GLenum, ++params: *GLint);

fn glGetProgramiv(++program: GLuint, ++pname: GLenum, ++params: *GLint);

fn glGetProgramInfoLog(++program: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++infolog: *GLchar);

fn glGetRenderbufferParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

fn glGetShaderiv(++shader: GLuint, ++pname: GLenum, ++params: *GLint);

fn glGetShaderInfoLog(++shader: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++infolog: *GLchar);

// Unsupported on Mac:
//fn glGetShaderPrecisionFormat(++shadertype: GLenum, ++precisiontype: GLenum, ++range: *GLint, ++precision: *GLint);

fn glGetShaderSource(++shader: GLuint, ++bufsize: GLsizei, ++length: *GLsizei, ++source: *GLchar);

fn glGetString(++name: GLenum) -> *GLubyte;

fn glGetTexParameterfv(++target: GLenum, ++pname: GLenum, ++params: *GLfloat);

fn glGetTexParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

fn glGetUniformfv(++program: GLuint, ++location: GLint, ++params: *GLfloat);

fn glGetUniformiv(++program: GLuint, ++location: GLint, ++params: *GLint);

fn glGetUniformLocation(++program: GLuint, ++name: *GLchar) -> c_int;

fn glGetVertexAttribfv(++index: GLuint, ++pname: GLenum, ++params: *GLfloat);

fn glGetVertexAttribiv(++index: GLuint, ++pname: GLenum, ++params: *GLint);

fn glGetVertexAttribPointerv(++index: GLuint, ++pname: GLenum, ++pointer: **GLvoid);

fn glHint(++target: GLenum, ++mode: GLenum);

fn glIsBuffer(++buffer: GLuint) -> GLboolean;

fn glIsEnabled(++cap: GLenum) -> GLboolean;

fn glIsFramebuffer(++framebuffer: GLuint) -> GLboolean;

fn glIsProgram(++program: GLuint) -> GLboolean;

fn glIsRenderbuffer(++renderbuffer: GLuint) -> GLboolean;

fn glIsShader(++shader: GLuint) -> GLboolean;

fn glIsTexture(++texture: GLuint) -> GLboolean;

fn glLineWidth(++width: GLfloat);

fn glLinkProgram(++program: GLuint);

fn glPixelStorei(++pname: GLenum, ++param: GLint);

fn glPolygonOffset(++factor: GLfloat, ++units: GLfloat);

fn glReadPixels(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++_type: GLenum, ++pixels: *GLvoid);

// Unsupported on Mac:
// fn glReleaseShaderCompiler();

fn glRenderbufferStorage(++target: GLenum, ++internalformat: GLenum, ++width: GLsizei, ++height: GLsizei);

fn glSampleCoverage(++value: GLclampf, ++invert: GLboolean);

fn glScissor(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);

// Unsupported on Mac:
//fn glShaderBinary(++n: GLsizei, ++shaders: *GLuint, ++binaryformat: GLenum, ++binary: *GLvoid, ++length: GLsizei);

fn glShaderSource(++shader: GLuint, ++count: GLsizei, ++string: **GLchar, ++length: *GLint);

fn glStencilFunc(++func: GLenum, ++ref: GLint, ++mask: GLuint);

fn glStencilFuncSeparate(++face: GLenum, ++func: GLenum, ++ref: GLint, ++mask: GLuint);

fn glStencilMask(++mask: GLuint);

fn glStencilMaskSeparate(++face: GLenum, ++mask: GLuint);

fn glStencilOp(++_fail: GLenum, ++zfail: GLenum, ++zpass: GLenum);

fn glStencilOpSeparate(++face: GLenum, ++_fail: GLenum, ++zfail: GLenum, ++zpass: GLenum);

fn glTexImage2D(++target: GLenum, ++level: GLint, ++internalformat: GLint, ++width: GLsizei, ++height: GLsizei, ++border: GLint, ++format: GLenum, ++_type: GLenum, ++pixels: *GLvoid);

fn glTexParameterf(++target: GLenum, ++pname: GLenum, ++param: GLfloat);

fn glTexParameterfv(++target: GLenum, ++pname: GLenum, ++params: *GLfloat);

fn glTexParameteri(++target: GLenum, ++pname: GLenum, ++param: GLint);

fn glTexParameteriv(++target: GLenum, ++pname: GLenum, ++params: *GLint);

fn glTexSubImage2D(++target: GLenum, ++level: GLint, ++xoffset: GLint, ++yoffset: GLint, ++width: GLsizei, ++height: GLsizei, ++format: GLenum, ++_type: GLenum, ++pixels: *GLvoid);

fn glUniform1f(++location: GLint, ++x: GLfloat);

fn glUniform1fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

fn glUniform1i(++location: GLint, ++x: GLint);

fn glUniform1iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

fn glUniform2f(++location: GLint, ++x: GLfloat, ++y: GLfloat);

fn glUniform2fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

fn glUniform2i(++location: GLint, ++x: GLint, ++y: GLint);

fn glUniform2iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

fn glUniform3f(++location: GLint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat);

fn glUniform3fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

fn glUniform3i(++location: GLint, ++x: GLint, ++y: GLint, ++z: GLint);

fn glUniform3iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

fn glUniform4f(++location: GLint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat, ++w: GLfloat);

fn glUniform4fv(++location: GLint, ++count: GLsizei, ++v: *GLfloat);

fn glUniform4i(++location: GLint, ++x: GLint, ++y: GLint, ++z: GLint, ++w: GLint);

fn glUniform4iv(++location: GLint, ++count: GLsizei, ++v: *GLint);

fn glUniformMatrix2fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);

fn glUniformMatrix3fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);

fn glUniformMatrix4fv(++location: GLint, ++count: GLsizei, ++transpose: GLboolean, ++value: *GLfloat);

fn glUseProgram(++program: GLuint);

fn glValidateProgram(++program: GLuint);

fn glVertexAttrib1f(++indx: GLuint, ++x: GLfloat);

fn glVertexAttrib1fv(++indx: GLuint, ++values: *GLfloat);

fn glVertexAttrib2f(++indx: GLuint, ++x: GLfloat, ++y: GLfloat);

fn glVertexAttrib2fv(++indx: GLuint, ++values: *GLfloat);

fn glVertexAttrib3f(++indx: GLuint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat);

fn glVertexAttrib3fv(++indx: GLuint, ++values: *GLfloat);

fn glVertexAttrib4f(++indx: GLuint, ++x: GLfloat, ++y: GLfloat, ++z: GLfloat, ++w: GLfloat);

fn glVertexAttrib4fv(++indx: GLuint, ++values: *GLfloat);

fn glVertexAttribPointer(++indx: GLuint, ++size: GLint, ++_type: GLenum, ++normalized: GLboolean, ++stride: GLsizei, ++ptr: *GLvoid);

fn glViewport(++x: GLint, ++y: GLint, ++width: GLsizei, ++height: GLsizei);

}