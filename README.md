# OpenGL bindings for Rust

These bindings are derived from [glcorearb.h](http://www.opengl.org/registry/api/glcorearb.h):

  > glcorearb.h includes only APIs in the latest OpenGL core profile implementation together with APIs in newer ARB extensions which can be can be supported by the core profile. It does not, and never will include functionality removed from the core profile, such as fixed-function vertex and fragment processing.

Please note that this is only an interim solution and that your app will most likely fail to run on another computer. I currently have an [OpenGL function pointer loader](https://github.com/bjz/glLoadGen) (a fork of [glLoadGen](https://bitbucket.org/alfonse/glloadgen)) waiting in the wings, but this will only work once we can call extern function pointers from Rust code.

## Using the bindings

All symbols are public at the top-level module scope, so all you need to do to include in your source is `use glcore::*`.

The default make target will attempt to call [glewinfo](http://glew.sourceforge.net/basic.html), and will then only build the bindings with the extensions supported by your system. Alternatively if you're on OS X 10.6 or greater you can use `make osx`. The full list of extensions included in glcore-rs can be found in `./extensions`.

~B☼