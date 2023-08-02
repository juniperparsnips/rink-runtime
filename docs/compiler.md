# How to deal with the compiler?

## Architecture

For more information about the global architecture, refer to the [official
documentation of
ink](https://github.com/inkle/ink/blob/master/Documentation/ArchitectureAndDevOverview.md).

The current repo is only the runtime, which is needed for future integration
with the Bevy engine. For the compiler, you can use
[inklecate](https://github.com/inkle/ink/releases/tag/v1.1.1). There is only
support for Windows & Linux, for other platforms we will need to do a rust port
of the compiler or create a wrapper around an existing (not C#) compiler.

## How to use inklecate?

1. Download the release package corresponding to your distro
2. Write the _ink_  in a .ink file
3. Call `./inklecate [my_file].ink`
4. It will create a file [my_file].ink.json
5. You can now use the content of the JSON file to test the runtime.
