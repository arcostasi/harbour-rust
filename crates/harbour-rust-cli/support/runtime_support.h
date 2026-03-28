#ifndef HARBOUR_RUST_RUNTIME_SUPPORT_H
#define HARBOUR_RUST_RUNTIME_SUPPORT_H

struct harbour_runtime_Value {
    enum {
        HARBOUR_VALUE_NIL = 0,
        HARBOUR_VALUE_LOGICAL = 1,
        HARBOUR_VALUE_INTEGER = 2,
        HARBOUR_VALUE_FLOAT = 3,
        HARBOUR_VALUE_STRING = 4
    } kind;
    union {
        _Bool logical;
        long long integer;
        double floating;
        const char *string;
    } as;
};

#endif
