#ifndef HARBOUR_RUST_RUNTIME_SUPPORT_H
#define HARBOUR_RUST_RUNTIME_SUPPORT_H

#include <stddef.h>

struct harbour_runtime_Value {
    enum {
        HARBOUR_VALUE_NIL = 0,
        HARBOUR_VALUE_LOGICAL = 1,
        HARBOUR_VALUE_INTEGER = 2,
        HARBOUR_VALUE_FLOAT = 3,
        HARBOUR_VALUE_STRING = 4,
        HARBOUR_VALUE_ARRAY = 5
    } kind;
    union {
        _Bool logical;
        long long integer;
        double floating;
        const char *string;
        struct {
            struct harbour_runtime_Value *items;
            size_t length;
        } array;
    } as;
};

_Bool harbour_value_is_true(struct harbour_runtime_Value value);
struct harbour_runtime_Value harbour_value_add(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_less_than(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_less_than_or_equal(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_postfix_increment(
    struct harbour_runtime_Value *value
);
struct harbour_runtime_Value harbour_value_from_array_items(
    const struct harbour_runtime_Value *items,
    size_t length
);
size_t harbour_value_array_len(struct harbour_runtime_Value value);
struct harbour_runtime_Value harbour_value_array_get(
    struct harbour_runtime_Value value,
    struct harbour_runtime_Value index
);
struct harbour_runtime_Value harbour_value_array_set_path(
    struct harbour_runtime_Value *value,
    const struct harbour_runtime_Value *indices,
    size_t index_count,
    struct harbour_runtime_Value assigned
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
