# stacktracer

This program crawls its own stack frames and dumps information about it.

As the code that performs the stack tracing is not written in a way that allows it to be used by other programs, there isn't much practical utility to it.

Here are some known limitations:

- This program probably only works on x86_64 linux machines (it may work on the \*BSDs).
- It is not able to resolve functions that live in dynamically linked libraries (for example, libc)
- The names are mangled

```console
$ cargo run
{
        sp: 0x7fff5f1a2100,
        bp: 0x7fff5f1a25e0,
        ip: 0x60e1a984fc68,
        function: _ZN12stack_tracer14get_stacktrace17h116502bf14ff44d0E,
}
{
        sp: 0x7fff5f1a25f0,
        bp: 0x7fff5f1a26d0,
        ip: 0x60e1a985181a,
        function: _ZN3std2rt10lang_start17h82f57a4a4f9bf223E,
}
{
        sp: 0x7fff5f1a26e0,
        bp: 0x7fff5f1a2730,
        ip: 0x76b11bc5decc,
        function: Unknown,
}
{
        sp: 0x7fff5f1a2740,
        bp: 0x0,
        ip: 0x60e1a984df55,
        function: _start,
}
```
