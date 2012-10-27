# OpenGL bindings for Rust

These bindings are derived from the latest version of [glcorearb.h](http://www.opengl.org/registry/api/glcorearb.h):

  > glcorearb.h includes only APIs in the latest OpenGL core profile implementation together with APIs in newer ARB extensions which can be can be supported by the core profile. It does not, and never will include functionality removed from the core profile, such as fixed-function vertex and fragment processing.

All symbols are public at the top-level module scope, so all you need to do to include in your source is `use glcore::*`.

Unfortunately Rust doesn't currently support dynamic loading of libraries. As an interim solution, I've included a cfg attribute for each api extension. You can find a list of these in `./extensions`.

The default make target includes all the extensions, but unless you're *very* fortunate it will almost certainly give you undefined symbol errors. If you're on OS X Lion or greater you can use `make osx-lion`. If not you'll have to compose your own target. My suggestion is to progressively add flags over time as you require sections of the api for your projects. Good luck!

~B☼