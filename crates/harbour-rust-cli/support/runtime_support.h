#ifndef HARBOUR_RUST_RUNTIME_SUPPORT_H
#define HARBOUR_RUST_RUNTIME_SUPPORT_H

#include <stddef.h>

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

_Bool harbour_value_is_true(struct harbour_runtime_Value value);
struct harbour_runtime_Value harbour_value_less_than(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_postfix_increment(
    struct harbour_runtime_Value *value
);
struct harbour_runtime_Value harbour_builtin_qout(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_value_nil(void);
struct harbour_runtime_Value harbour_value_from_logical(_Bool logical);
struct harbour_runtime_Value harbour_value_from_integer(long long integer);
struct harbour_runtime_Value harbour_value_from_float(double floating);
struct harbour_runtime_Value harbour_value_from_string_literal(const char *string);

#endif
