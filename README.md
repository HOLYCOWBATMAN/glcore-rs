# OpenGL bindings for Rust

These bindings are derived from the latest version of [glcorearb.h](http://www.opengl.org/registry/api/glcorearb.h):

  > glcorearb.h includes only APIs in the latest OpenGL core profile implementation together with APIs in newer ARB extensions which can be can be supported by the core profile. It does not, and never will include functionality removed from the core profile, such as fixed-function vertex and fragment processing.

Unfortunately Rust doesn't currently support dynamic loading of libraries, so in the mean time you'll have to comment out the functions that your platform doesn't support.

Be warned: although I tried to be careful there was a great deal of regex involved in the conversion process. I might have made some mistakes!

~B☼