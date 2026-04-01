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
        HARBOUR_VALUE_ARRAY = 5,
        HARBOUR_VALUE_ERROR = 6
    } kind;
    union {
        _Bool logical;
        long long integer;
        double floating;
        const char *string;
        const char *error;
        struct {
            struct harbour_runtime_Value *items;
            size_t length;
            unsigned long long identity;
        } array;
    } as;
};

_Bool harbour_value_is_true(struct harbour_runtime_Value value);
struct harbour_runtime_Value harbour_value_equals(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_exact_equals(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_not_equals(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_add(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_subtract(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_multiply(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_divide(
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
struct harbour_runtime_Value harbour_value_greater_than(
    struct harbour_runtime_Value left,
    struct harbour_runtime_Value right
);
struct harbour_runtime_Value harbour_value_greater_than_or_equal(
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
struct harbour_runtime_Value harbour_builtin_len(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_str(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_substr(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_left(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_right(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_upper(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_lower(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_trim(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_ltrim(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_rtrim(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_at(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_replicate(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_space(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_aclone(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_aadd(
    struct harbour_runtime_Value *array,
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_builtin_asize(
    struct harbour_runtime_Value *array,
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
);
struct harbour_runtime_Value harbour_value_nil(void);
struct harbour_runtime_Value harbour_value_from_logical(_Bool logical);
struct harbour_runtime_Value harbour_value_from_integer(long long integer);
struct harbour_runtime_Value harbour_value_from_float(double floating);
struct harbour_runtime_Value harbour_value_from_string_literal(const char *string);
struct harbour_runtime_Value harbour_value_error_literal(const char *error);

#endif
