#include <ctype.h>
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "runtime_support.h"

typedef struct harbour_runtime_Value harbour_runtime_Value;

static harbour_runtime_Value harbour_value_clone(harbour_runtime_Value value);
static _Bool harbour_value_resize_array(harbour_runtime_Value *value, size_t length);
static unsigned long long harbour_allocate_array_identity(void);
static harbour_runtime_Value harbour_substr_from_bounds(
    const char *text,
    size_t length,
    size_t start,
    size_t count
);
static harbour_runtime_Value harbour_ascii_case_transform(
    const char *text,
    int (*transform)(int)
);
static _Bool harbour_try_numeric_pair(
    harbour_runtime_Value left,
    harbour_runtime_Value right,
    double *left_number,
    double *right_number
);
static harbour_runtime_Value harbour_unsupported_comparison(void);
static harbour_runtime_Value harbour_array_comparison_error(const char *message);

static unsigned long long harbour_array_identity_seed = 1;

harbour_runtime_Value harbour_value_nil(void) {
    harbour_runtime_Value value;
    value.kind = HARBOUR_VALUE_NIL;
    return value;
}

harbour_runtime_Value harbour_value_from_logical(_Bool logical) {
    harbour_runtime_Value value;
    value.kind = HARBOUR_VALUE_LOGICAL;
    value.as.logical = logical;
    return value;
}

harbour_runtime_Value harbour_value_from_integer(long long integer) {
    harbour_runtime_Value value;
    value.kind = HARBOUR_VALUE_INTEGER;
    value.as.integer = integer;
    return value;
}

harbour_runtime_Value harbour_value_from_float(double floating) {
    harbour_runtime_Value value;
    value.kind = HARBOUR_VALUE_FLOAT;
    value.as.floating = floating;
    return value;
}

harbour_runtime_Value harbour_value_from_string_literal(const char *string) {
    harbour_runtime_Value value;
    value.kind = HARBOUR_VALUE_STRING;
    value.as.string = string;
    return value;
}

harbour_runtime_Value harbour_value_error_literal(const char *error) {
    harbour_runtime_Value value;
    value.kind = HARBOUR_VALUE_ERROR;
    value.as.error = error;
    return value;
}

harbour_runtime_Value harbour_value_from_array_items(
    const harbour_runtime_Value *items,
    size_t length
) {
    harbour_runtime_Value value;
    size_t index;

    value.kind = HARBOUR_VALUE_ARRAY;
    value.as.array.length = length;
    value.as.array.items = NULL;
    value.as.array.identity = harbour_allocate_array_identity();

    if (length == 0) {
        return value;
    }

    value.as.array.items = (harbour_runtime_Value *) malloc(
        sizeof(harbour_runtime_Value) * length
    );
    if (value.as.array.items == NULL) {
        value.kind = HARBOUR_VALUE_NIL;
        value.as.array.length = 0;
        value.as.array.identity = 0;
        return value;
    }

    for (index = 0; index < length; ++index) {
        value.as.array.items[index] = items[index];
    }

    return value;
}

size_t harbour_value_array_len(harbour_runtime_Value value) {
    if (value.kind == HARBOUR_VALUE_ARRAY) {
        return value.as.array.length;
    }

    return 0;
}

harbour_runtime_Value harbour_value_array_get(
    harbour_runtime_Value value,
    harbour_runtime_Value index
) {
    long long position;

    if (
        index.kind != HARBOUR_VALUE_INTEGER ||
        value.kind != HARBOUR_VALUE_ARRAY ||
        index.as.integer <= 0 ||
        (size_t) index.as.integer > value.as.array.length
    ) {
        return harbour_value_nil();
    }

    position = index.as.integer;
    return value.as.array.items[position - 1];
}

harbour_runtime_Value harbour_value_array_set_path(
    harbour_runtime_Value *value,
    const harbour_runtime_Value *indices,
    size_t index_count,
    harbour_runtime_Value assigned
) {
    harbour_runtime_Value *current;
    size_t position;

    if (value == NULL || indices == NULL || index_count == 0) {
        return harbour_value_nil();
    }

    current = value;

    for (position = 0; position < index_count; ++position) {
        long long index;

        if (
            indices[position].kind != HARBOUR_VALUE_INTEGER ||
            current->kind != HARBOUR_VALUE_ARRAY
        ) {
            return harbour_value_nil();
        }

        index = indices[position].as.integer;
        if (index <= 0 || (size_t) index > current->as.array.length) {
            return harbour_value_nil();
        }

        if (position + 1 == index_count) {
            current->as.array.items[index - 1] = assigned;
            return assigned;
        }

        current = &current->as.array.items[index - 1];
    }

    return harbour_value_nil();
}

harbour_runtime_Value harbour_value_add(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_integer(left.as.integer + right.as.integer);
    }

    if (
        (left.kind == HARBOUR_VALUE_INTEGER || left.kind == HARBOUR_VALUE_FLOAT) &&
        (right.kind == HARBOUR_VALUE_INTEGER || right.kind == HARBOUR_VALUE_FLOAT)
    ) {
        double left_number = left.kind == HARBOUR_VALUE_INTEGER
            ? (double) left.as.integer
            : left.as.floating;
        double right_number = right.kind == HARBOUR_VALUE_INTEGER
            ? (double) right.as.integer
            : right.as.floating;
        return harbour_value_from_float(left_number + right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_string_literal("");
    }

    return harbour_value_nil();
}

harbour_runtime_Value harbour_value_subtract(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_integer(left.as.integer - right.as.integer);
    }

    if (
        (left.kind == HARBOUR_VALUE_INTEGER || left.kind == HARBOUR_VALUE_FLOAT) &&
        (right.kind == HARBOUR_VALUE_INTEGER || right.kind == HARBOUR_VALUE_FLOAT)
    ) {
        double left_number = left.kind == HARBOUR_VALUE_INTEGER
            ? (double) left.as.integer
            : left.as.floating;
        double right_number = right.kind == HARBOUR_VALUE_INTEGER
            ? (double) right.as.integer
            : right.as.floating;
        return harbour_value_from_float(left_number - right_number);
    }

    return harbour_value_nil();
}

harbour_runtime_Value harbour_value_multiply(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_integer(left.as.integer * right.as.integer);
    }

    if (
        (left.kind == HARBOUR_VALUE_INTEGER || left.kind == HARBOUR_VALUE_FLOAT) &&
        (right.kind == HARBOUR_VALUE_INTEGER || right.kind == HARBOUR_VALUE_FLOAT)
    ) {
        double left_number = left.kind == HARBOUR_VALUE_INTEGER
            ? (double) left.as.integer
            : left.as.floating;
        double right_number = right.kind == HARBOUR_VALUE_INTEGER
            ? (double) right.as.integer
            : right.as.floating;
        return harbour_value_from_float(left_number * right_number);
    }

    return harbour_value_nil();
}

harbour_runtime_Value harbour_value_divide(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (!harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_nil();
    }

    if (right_number == 0.0) {
        return harbour_value_error_literal("divide by zero");
    }

    return harbour_value_from_float(left_number / right_number);
}

harbour_runtime_Value harbour_value_equals(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (left.kind == HARBOUR_VALUE_ARRAY || right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_array_comparison_error("BASE 1071 Argument error (=)");
    }

    if (left.kind == HARBOUR_VALUE_NIL && right.kind == HARBOUR_VALUE_NIL) {
        return harbour_value_from_logical(1);
    }

    if (left.kind == HARBOUR_VALUE_NIL || right.kind == HARBOUR_VALUE_NIL) {
        return harbour_value_from_logical(0);
    }

    if (left.kind == HARBOUR_VALUE_LOGICAL && right.kind == HARBOUR_VALUE_LOGICAL) {
        return harbour_value_from_logical(left.as.logical == right.as.logical);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) == 0);
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_from_logical(left_number == right_number);
    }

    return harbour_unsupported_comparison();
}

harbour_runtime_Value harbour_value_exact_equals(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (left.kind == HARBOUR_VALUE_NIL && right.kind == HARBOUR_VALUE_NIL) {
        return harbour_value_from_logical(1);
    }

    if (left.kind == HARBOUR_VALUE_NIL || right.kind == HARBOUR_VALUE_NIL) {
        return harbour_value_from_logical(0);
    }

    if (left.kind == HARBOUR_VALUE_LOGICAL && right.kind == HARBOUR_VALUE_LOGICAL) {
        return harbour_value_from_logical(left.as.logical == right.as.logical);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) == 0);
    }

    if (left.kind == HARBOUR_VALUE_ARRAY && right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_value_from_logical(left.as.array.identity == right.as.array.identity);
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_from_logical(left_number == right_number);
    }

    return harbour_unsupported_comparison();
}

harbour_runtime_Value harbour_value_not_equals(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    if (left.kind == HARBOUR_VALUE_ARRAY || right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_array_comparison_error("BASE 1072 Argument error (<>)");
    }

    harbour_runtime_Value equals = harbour_value_equals(left, right);

    if (equals.kind == HARBOUR_VALUE_LOGICAL) {
        return harbour_value_from_logical(!equals.as.logical);
    }

    return equals;
}

_Bool harbour_value_is_true(harbour_runtime_Value value) {
    switch (value.kind) {
    case HARBOUR_VALUE_NIL:
        return 0;
    case HARBOUR_VALUE_LOGICAL:
        return value.as.logical;
    case HARBOUR_VALUE_INTEGER:
        return value.as.integer != 0;
    case HARBOUR_VALUE_FLOAT:
        return value.as.floating != 0.0;
    case HARBOUR_VALUE_STRING:
        return value.as.string != NULL && value.as.string[0] != '\0';
    case HARBOUR_VALUE_ARRAY:
        return value.as.array.length != 0;
    default:
        return 0;
    }
}

harbour_runtime_Value harbour_value_less_than(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_logical(left.as.integer < right.as.integer);
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_from_logical(left_number < right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) < 0);
    }

    if (left.kind == HARBOUR_VALUE_ARRAY || right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_array_comparison_error("BASE 1073 Argument error (<)");
    }

    return harbour_value_from_logical(0);
}

harbour_runtime_Value harbour_value_less_than_or_equal(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_logical(left.as.integer <= right.as.integer);
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_from_logical(left_number <= right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) <= 0);
    }

    if (left.kind == HARBOUR_VALUE_ARRAY || right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_array_comparison_error("BASE 1074 Argument error (<=)");
    }

    return harbour_value_from_logical(0);
}

harbour_runtime_Value harbour_value_greater_than(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_logical(left.as.integer > right.as.integer);
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_from_logical(left_number > right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) > 0);
    }

    if (left.kind == HARBOUR_VALUE_ARRAY || right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_array_comparison_error("BASE 1075 Argument error (>)");
    }

    return harbour_value_from_logical(0);
}

harbour_runtime_Value harbour_value_greater_than_or_equal(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    double left_number;
    double right_number;

    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_logical(left.as.integer >= right.as.integer);
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        return harbour_value_from_logical(left_number >= right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) >= 0);
    }

    if (left.kind == HARBOUR_VALUE_ARRAY || right.kind == HARBOUR_VALUE_ARRAY) {
        return harbour_array_comparison_error("BASE 1076 Argument error (>=)");
    }

    return harbour_value_from_logical(0);
}

harbour_runtime_Value harbour_value_postfix_increment(harbour_runtime_Value *value) {
    harbour_runtime_Value previous = *value;

    if (value->kind == HARBOUR_VALUE_INTEGER) {
        value->as.integer += 1;
    } else if (value->kind == HARBOUR_VALUE_FLOAT) {
        value->as.floating += 1.0;
    }

    return previous;
}

static void harbour_print_value(const harbour_runtime_Value *value) {
    switch (value->kind) {
    case HARBOUR_VALUE_NIL:
        fputs("NIL", stdout);
        break;
    case HARBOUR_VALUE_LOGICAL:
        fputs(value->as.logical ? ".T." : ".F.", stdout);
        break;
    case HARBOUR_VALUE_INTEGER:
        fprintf(stdout, "%lld", value->as.integer);
        break;
    case HARBOUR_VALUE_FLOAT:
        fprintf(stdout, "%g", value->as.floating);
        break;
    case HARBOUR_VALUE_STRING:
        fputs(value->as.string, stdout);
        break;
    case HARBOUR_VALUE_ERROR:
        fputs(value->as.error, stdout);
        break;
    case HARBOUR_VALUE_ARRAY:
        fprintf(stdout, "{ Array(%zu) }", value->as.array.length);
        break;
    default:
        fputs("<invalid>", stdout);
        break;
    }
}

harbour_runtime_Value harbour_builtin_qout(
    const harbour_runtime_Value *arguments,
    size_t argument_count
) {
    size_t index;

    if (arguments != NULL) {
        for (index = 0; index < argument_count; ++index) {
            if (index > 0) {
                fputc(' ', stdout);
            }
            harbour_print_value(&arguments[index]);
        }
    }

    fputc('\n', stdout);
    fflush(stdout);

    return harbour_value_nil();
}

struct harbour_runtime_Value harbour_builtin_len(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (arguments == NULL || argument_count == 0) {
        return harbour_value_error_literal("BASE 1111 Argument error (LEN)");
    }

    if (arguments[0].kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_integer((long long) strlen(arguments[0].as.string));
    }

    if (arguments[0].kind == HARBOUR_VALUE_ARRAY) {
        return harbour_value_from_integer((long long) arguments[0].as.array.length);
    }

    return harbour_value_error_literal("BASE 1111 Argument error (LEN)");
}

struct harbour_runtime_Value harbour_builtin_substr(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *text;
    size_t text_length;
    long long start;
    long long count;
    size_t start_index = 0;
    size_t available;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        arguments[0].kind != HARBOUR_VALUE_STRING ||
        arguments[1].kind != HARBOUR_VALUE_INTEGER ||
        ( argument_count >= 3 && arguments[2].kind != HARBOUR_VALUE_INTEGER )
    ) {
        return harbour_value_error_literal("BASE 1110 Argument error (SUBSTR)");
    }

    text = arguments[0].as.string;
    text_length = strlen(text);
    start = arguments[1].as.integer;
    count = argument_count < 3 ? (long long) text_length : arguments[2].as.integer;

    if (start > 0) {
        start -= 1;
        if (start > (long long) text_length) {
            count = 0;
        }
    }

    if (count <= 0) {
        return harbour_value_from_string_literal("");
    }

    if (start < 0) {
        start += (long long) text_length;
    }

    available = text_length;
    if (start > 0) {
        start_index = (size_t) start;
        available = text_length - start_index;
    }

    if (count > (long long) available) {
        count = (long long) available;
    }

    if (count <= 0) {
        return harbour_value_from_string_literal("");
    }

    return harbour_substr_from_bounds(text, text_length, start_index, (size_t) count);
}

struct harbour_runtime_Value harbour_builtin_left(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *text;
    size_t text_length;
    long long count;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        arguments[0].kind != HARBOUR_VALUE_STRING ||
        arguments[1].kind != HARBOUR_VALUE_INTEGER
    ) {
        return harbour_value_error_literal("BASE 1124 Argument error (LEFT)");
    }

    text = arguments[0].as.string;
    text_length = strlen(text);
    count = arguments[1].as.integer;

    if (count <= 0) {
        return harbour_value_from_string_literal("");
    }

    if ((size_t) count >= text_length) {
        return harbour_value_from_string_literal(text);
    }

    return harbour_substr_from_bounds(text, text_length, 0, (size_t) count);
}

struct harbour_runtime_Value harbour_builtin_right(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *text;
    size_t text_length;
    long long count;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        arguments[0].kind != HARBOUR_VALUE_STRING ||
        arguments[1].kind != HARBOUR_VALUE_INTEGER
    ) {
        return harbour_value_from_string_literal("");
    }

    text = arguments[0].as.string;
    text_length = strlen(text);
    count = arguments[1].as.integer;

    if (count <= 0) {
        return harbour_value_from_string_literal("");
    }

    if ((size_t) count >= text_length) {
        return harbour_value_from_string_literal(text);
    }

    return harbour_substr_from_bounds(
        text,
        text_length,
        text_length - (size_t) count,
        (size_t) count
    );
}

struct harbour_runtime_Value harbour_builtin_upper(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1102 Argument error (UPPER)");
    }

    return harbour_ascii_case_transform(arguments[0].as.string, toupper);
}

struct harbour_runtime_Value harbour_builtin_lower(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1103 Argument error (LOWER)");
    }

    return harbour_ascii_case_transform(arguments[0].as.string, tolower);
}

struct harbour_runtime_Value harbour_builtin_trim(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    return harbour_builtin_rtrim(arguments, argument_count);
}

struct harbour_runtime_Value harbour_builtin_ltrim(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *text;
    size_t length;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1101 Argument error (LTRIM)");
    }

    text = arguments[0].as.string;
    while (*text != '\0' && isspace((unsigned char) *text)) {
        text++;
    }

    length = strlen(text);
    return harbour_substr_from_bounds(text, length, 0, length);
}

struct harbour_runtime_Value harbour_builtin_rtrim(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *text;
    size_t length;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1100 Argument error (TRIM)");
    }

    text = arguments[0].as.string;
    length = strlen(text);

    while (length > 0 && text[length - 1] == ' ') {
        length--;
    }

    return harbour_substr_from_bounds(text, strlen(text), 0, length);
}

struct harbour_runtime_Value harbour_builtin_aclone(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (arguments == NULL || argument_count == 0) {
        return harbour_value_nil();
    }

    if (arguments[0].kind != HARBOUR_VALUE_ARRAY) {
        return harbour_value_nil();
    }

    return harbour_value_clone(arguments[0]);
}

struct harbour_runtime_Value harbour_builtin_aadd(
    struct harbour_runtime_Value *array,
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    size_t previous_length;

    if (array == NULL || array->kind != HARBOUR_VALUE_ARRAY) {
        return harbour_value_nil();
    }

    if (arguments == NULL || argument_count == 0) {
        return harbour_value_nil();
    }

    previous_length = array->as.array.length;
    if (!harbour_value_resize_array(array, previous_length + 1)) {
        return harbour_value_nil();
    }

    array->as.array.items[previous_length] = arguments[0];
    return arguments[0];
}

struct harbour_runtime_Value harbour_builtin_asize(
    struct harbour_runtime_Value *array,
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    long long requested_length;

    if (array == NULL || array->kind != HARBOUR_VALUE_ARRAY) {
        return harbour_value_nil();
    }

    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_INTEGER
    ) {
        return harbour_value_nil();
    }

    requested_length = arguments[0].as.integer;
    if (requested_length <= 0) {
        if (!harbour_value_resize_array(array, 0)) {
            return harbour_value_nil();
        }
    } else if (!harbour_value_resize_array(array, (size_t) requested_length)) {
        return harbour_value_nil();
    }

    return harbour_value_clone(*array);
}

static harbour_runtime_Value harbour_value_clone(harbour_runtime_Value value) {
    size_t index;
    harbour_runtime_Value cloned;
    harbour_runtime_Value *items;

    if (value.kind != HARBOUR_VALUE_ARRAY) {
        return value;
    }

    if (value.as.array.length == 0) {
        return harbour_value_from_array_items(NULL, 0);
    }

    items = (harbour_runtime_Value *) malloc(sizeof(harbour_runtime_Value) * value.as.array.length);
    if (items == NULL) {
        return harbour_value_nil();
    }

    for (index = 0; index < value.as.array.length; ++index) {
        items[index] = harbour_value_clone(value.as.array.items[index]);
    }

    cloned = harbour_value_from_array_items(items, value.as.array.length);
    free(items);
    return cloned;
}

static _Bool harbour_value_resize_array(harbour_runtime_Value *value, size_t length) {
    harbour_runtime_Value *resized_items;
    size_t index;
    size_t copied_length;

    if (value == NULL || value->kind != HARBOUR_VALUE_ARRAY) {
        return 0;
    }

    if (length == 0) {
        value->as.array.items = NULL;
        value->as.array.length = 0;
        return 1;
    }

    resized_items = (harbour_runtime_Value *) malloc(sizeof(harbour_runtime_Value) * length);
    if (resized_items == NULL) {
        return 0;
    }

    copied_length = value->as.array.length < length ? value->as.array.length : length;
    for (index = 0; index < copied_length; ++index) {
        resized_items[index] = value->as.array.items[index];
    }

    for (index = copied_length; index < length; ++index) {
        resized_items[index] = harbour_value_nil();
    }

    value->as.array.items = resized_items;
    value->as.array.length = length;
    return 1;
}

static harbour_runtime_Value harbour_substr_from_bounds(
    const char *text,
    size_t length,
    size_t start,
    size_t count
) {
    char *slice;

    if (count == 0) {
        return harbour_value_from_string_literal("");
    }

    if (start == 0 && count == length) {
        return harbour_value_from_string_literal(text);
    }

    slice = (char *) malloc(count + 1);
    if (slice == NULL) {
        return harbour_value_nil();
    }

    memcpy(slice, text + start, count);
    slice[count] = '\0';
    return harbour_value_from_string_literal(slice);
}

static unsigned long long harbour_allocate_array_identity(void) {
    return harbour_array_identity_seed++;
}

static _Bool harbour_try_numeric_pair(
    harbour_runtime_Value left,
    harbour_runtime_Value right,
    double *left_number,
    double *right_number
) {
    if (
        (left.kind == HARBOUR_VALUE_INTEGER || left.kind == HARBOUR_VALUE_FLOAT) &&
        (right.kind == HARBOUR_VALUE_INTEGER || right.kind == HARBOUR_VALUE_FLOAT)
    ) {
        *left_number = left.kind == HARBOUR_VALUE_INTEGER
            ? (double) left.as.integer
            : left.as.floating;
        *right_number = right.kind == HARBOUR_VALUE_INTEGER
            ? (double) right.as.integer
            : right.as.floating;
        return 1;
    }

    return 0;
}

static harbour_runtime_Value harbour_unsupported_comparison(void) {
    return harbour_value_nil();
}

static harbour_runtime_Value harbour_array_comparison_error(const char *message) {
    return harbour_value_error_literal(message);
}
static harbour_runtime_Value harbour_ascii_case_transform(
    const char *text,
    int (*transform)(int)
) {
    size_t index;
    size_t length;
    char *buffer;

    length = strlen(text);
    if (length == 0) {
        return harbour_value_from_string_literal("");
    }

    buffer = (char *) malloc(length + 1);
    if (buffer == NULL) {
        return harbour_value_from_string_literal("");
    }

    for (index = 0; index < length; ++index) {
        buffer[index] = (char) transform((unsigned char) text[index]);
    }
    buffer[length] = '\0';

    return harbour_value_from_string_literal(buffer);
}
