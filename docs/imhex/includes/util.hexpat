#include <std/io.pat>

struct Vector<T> {
    u32 size;
    T elements[size];
};

using String = Vector<char> [[format("format_string")]];

fn format_string(ref String input) { 
    return input.elements;
};
