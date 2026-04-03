#include <ctype.h>
#include <ctype.h>
#include <math.h>
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
static harbour_runtime_Value harbour_string_value_from_owned_buffer(const char *text);
static _Bool harbour_try_str_integer_arg(
    harbour_runtime_Value value,
    long long *result
);
static harbour_runtime_Value harbour_str_apply_width(
    const char *formatted,
    long long width,
    _Bool explicit_width
);
static harbour_runtime_Value harbour_str_non_finite_placeholder(
    long long width,
    _Bool explicit_width
);
static harbour_runtime_Value harbour_str_default_numeric(harbour_runtime_Value value);
static harbour_runtime_Value harbour_val_parse_string(const char *text);
static _Bool harbour_try_truncated_repeat_count(
    harbour_runtime_Value value,
    long long *count
);
static _Bool harbour_string_is_empty(const char *text);
static const char *harbour_type_trim_ascii(const char *text, size_t *length);
static _Bool harbour_type_matches_ascii_case_insensitive(
    const char *text,
    size_t length,
    const char *expected
);
static _Bool harbour_type_string_is_numeric(const char *text, size_t length);
static _Bool harbour_type_string_is_quoted(const char *text, size_t length);
static _Bool harbour_type_string_is_array_literal(const char *text, size_t length);
static _Bool harbour_try_numeric_pair(
    harbour_runtime_Value left,
    harbour_runtime_Value right,
    double *left_number,
    double *right_number
);
static _Bool harbour_try_max_min_compare(
    harbour_runtime_Value left,
    harbour_runtime_Value right,
    int *comparison
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

harbour_runtime_Value harbour_builtin_abs(
    const harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (arguments == NULL || argument_count == 0) {
        return harbour_value_error_literal("BASE 1089 Argument error (ABS)");
    }

    if (arguments[0].kind == HARBOUR_VALUE_INTEGER) {
        if (arguments[0].as.integer >= 0) {
            return harbour_value_from_integer(arguments[0].as.integer);
        }
        if (arguments[0].as.integer == (-9223372036854775807LL - 1LL)) {
            return harbour_value_from_float(fabs((double) arguments[0].as.integer));
        }
        return harbour_value_from_integer(-arguments[0].as.integer);
    }

    if (arguments[0].kind == HARBOUR_VALUE_FLOAT) {
        return harbour_value_from_float(fabs(arguments[0].as.floating));
    }

    return harbour_value_error_literal("BASE 1089 Argument error (ABS)");
}

struct harbour_runtime_Value harbour_builtin_sqrt(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double value;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1097 Argument error (SQRT)");
    }

    value = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;

    if (value <= 0.0) {
        return harbour_value_from_float(0.0);
    }

    return harbour_value_from_float(sqrt(value));
}

struct harbour_runtime_Value harbour_builtin_sin(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double value;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1091 Argument error (SIN)");
    }

    value = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;

    return harbour_value_from_float(sin(value));
}

struct harbour_runtime_Value harbour_builtin_cos(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double value;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1091 Argument error (COS)");
    }

    value = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;

    return harbour_value_from_float(cos(value));
}

struct harbour_runtime_Value harbour_builtin_tan(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double value;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1091 Argument error (TAN)");
    }

    value = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;

    return harbour_value_from_float(tan(value));
}

struct harbour_runtime_Value harbour_builtin_exp(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double value;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1096 Argument error (EXP)");
    }

    value = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;

    return harbour_value_from_float(exp(value));
}

struct harbour_runtime_Value harbour_builtin_log(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double value;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1095 Argument error (LOG)");
    }

    value = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;

    if (value <= 0.0) {
        return harbour_value_from_float(-HUGE_VAL);
    }

    return harbour_value_from_float(log(value));
}

struct harbour_runtime_Value harbour_builtin_int(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double truncated;

    if (arguments == NULL || argument_count == 0) {
        return harbour_value_error_literal("BASE 1090 Argument error (INT)");
    }

    if (arguments[0].kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_integer(arguments[0].as.integer);
    }

    if (arguments[0].kind == HARBOUR_VALUE_FLOAT) {
        truncated = trunc(arguments[0].as.floating);
        if (
            truncated >= (double) (-9223372036854775807LL - 1LL) &&
            truncated <= (double) 9223372036854775807LL
        ) {
            return harbour_value_from_integer((long long) truncated);
        }
        return harbour_value_from_float(truncated);
    }

    return harbour_value_error_literal("BASE 1090 Argument error (INT)");
}

struct harbour_runtime_Value harbour_builtin_round(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    harbour_runtime_Value number;
    long long decimals;
    double value;
    double factor;
    double rounded;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT ) ||
        ( arguments[1].kind != HARBOUR_VALUE_INTEGER &&
          arguments[1].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1094 Argument error (ROUND)");
    }

    number = arguments[0];
    decimals = arguments[1].kind == HARBOUR_VALUE_INTEGER
        ? arguments[1].as.integer
        : (long long) trunc(arguments[1].as.floating);

    if (decimals == 0 && number.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_integer(number.as.integer);
    }

    value = number.kind == HARBOUR_VALUE_INTEGER
        ? (double) number.as.integer
        : number.as.floating;

    if (decimals >= 0) {
        factor = pow(10.0, (double) decimals);
        rounded = round(value * factor) / factor;
    } else {
        factor = pow(10.0, (double) (-decimals));
        rounded = round(value / factor) * factor;
    }

    if (
        decimals <= 0 &&
        rounded >= (double) (-9223372036854775807LL - 1LL) &&
        rounded <= (double) 9223372036854775807LL
    ) {
        return harbour_value_from_integer((long long) rounded);
    }

    return harbour_value_from_float(rounded);
}

struct harbour_runtime_Value harbour_builtin_mod(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    double number;
    double divisor;
    double result;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT ) ||
        ( arguments[1].kind != HARBOUR_VALUE_INTEGER &&
          arguments[1].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1085 Argument error (%)");
    }

    number = arguments[0].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[0].as.integer
        : arguments[0].as.floating;
    divisor = arguments[1].kind == HARBOUR_VALUE_INTEGER
        ? (double) arguments[1].as.integer
        : arguments[1].as.floating;

    if (divisor == 0.0) {
        return harbour_value_error_literal("BASE 1341 Zero divisor (%)");
    }

    result = fmod(number, divisor);
    if (
        result != 0.0 &&
        ((number > 0.0 && divisor < 0.0) || (number < 0.0 && divisor > 0.0))
    ) {
        result += divisor;
    }

    if (result == 0.0) {
        result = 0.0;
    }

    return harbour_value_from_float(result);
}

struct harbour_runtime_Value harbour_builtin_max(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    int comparison;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        !harbour_try_max_min_compare(arguments[0], arguments[1], &comparison)
    ) {
        return harbour_value_error_literal("BASE 1093 Argument error (MAX)");
    }

    return comparison < 0 ? arguments[1] : arguments[0];
}

struct harbour_runtime_Value harbour_builtin_min(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    int comparison;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        !harbour_try_max_min_compare(arguments[0], arguments[1], &comparison)
    ) {
        return harbour_value_error_literal("BASE 1092 Argument error (MIN)");
    }

    return comparison > 0 ? arguments[1] : arguments[0];
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

struct harbour_runtime_Value harbour_builtin_str(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    harbour_runtime_Value number;
    long long width = 10;
    long long decimals = 0;
    _Bool explicit_width = 0;
    _Bool explicit_decimals = 0;
    char buffer[128];

    if (
        arguments == NULL ||
        argument_count == 0 ||
        ( arguments[0].kind != HARBOUR_VALUE_INTEGER &&
          arguments[0].kind != HARBOUR_VALUE_FLOAT )
    ) {
        return harbour_value_error_literal("BASE 1099 Argument error (STR)");
    }

    number = arguments[0];

    if (argument_count >= 2) {
        if (!harbour_try_str_integer_arg(arguments[1], &width)) {
            return harbour_value_error_literal("BASE 1099 Argument error (STR)");
        }
        explicit_width = 1;
    }

    if (argument_count >= 3) {
        if (!harbour_try_str_integer_arg(arguments[2], &decimals)) {
            return harbour_value_error_literal("BASE 1099 Argument error (STR)");
        }
        if (decimals < 0) {
            decimals = 0;
        }
        explicit_decimals = 1;
    }

    if (number.kind == HARBOUR_VALUE_FLOAT && !isfinite(number.as.floating)) {
        return harbour_str_non_finite_placeholder(width, explicit_width);
    }

    if (explicit_decimals) {
        double numeric = number.kind == HARBOUR_VALUE_INTEGER
            ? (double) number.as.integer
            : number.as.floating;
        snprintf(buffer, sizeof(buffer), "%.*f", (int) decimals, numeric);
        return harbour_str_apply_width(buffer, width, 1);
    }

    if (explicit_width) {
        if (number.kind == HARBOUR_VALUE_INTEGER) {
            snprintf(buffer, sizeof(buffer), "%lld", number.as.integer);
        } else {
            snprintf(buffer, sizeof(buffer), "%.0f", number.as.floating);
        }
        return harbour_str_apply_width(buffer, width, 1);
    }

    return harbour_str_default_numeric(number);
}

struct harbour_runtime_Value harbour_builtin_val(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1098 Argument error (VAL)");
    }

    return harbour_val_parse_string(arguments[0].as.string);
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

struct harbour_runtime_Value harbour_builtin_at(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *needle;
    const char *haystack;
    const char *found;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        arguments[0].kind != HARBOUR_VALUE_STRING ||
        arguments[1].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1108 Argument error (AT)");
    }

    needle = arguments[0].as.string;
    haystack = arguments[1].as.string;
    if (needle[0] == '\0' || haystack[0] == '\0') {
        return harbour_value_from_integer(0);
    }

    found = strstr(haystack, needle);
    if (found == NULL) {
        return harbour_value_from_integer(0);
    }

    return harbour_value_from_integer((long long) (found - haystack) + 1);
}

struct harbour_runtime_Value harbour_builtin_replicate(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *text;
    size_t unit_length;
    long long repeat_count;
    size_t total_length;
    char *buffer;
    size_t offset = 0;
    size_t index;

    if (
        arguments == NULL ||
        argument_count < 2 ||
        arguments[0].kind != HARBOUR_VALUE_STRING ||
        !harbour_try_truncated_repeat_count(arguments[1], &repeat_count)
    ) {
        return harbour_value_error_literal("BASE 1106 Argument error (REPLICATE)");
    }

    if (repeat_count <= 0) {
        return harbour_value_from_string_literal("");
    }

    text = arguments[0].as.string;
    unit_length = strlen(text);
    if (unit_length == 0) {
        return harbour_value_from_string_literal("");
    }

    if ((size_t) repeat_count > ((size_t) -1) / unit_length) {
        return harbour_value_error_literal("BASE 1234 String overflow (REPLICATE)");
    }

    total_length = (size_t) repeat_count * unit_length;
    buffer = (char *) malloc(total_length + 1);
    if (buffer == NULL) {
        return harbour_value_error_literal("BASE 1234 String overflow (REPLICATE)");
    }

    for (index = 0; index < (size_t) repeat_count; ++index) {
        memcpy(buffer + offset, text, unit_length);
        offset += unit_length;
    }
    buffer[total_length] = '\0';

    return harbour_value_from_string_literal(buffer);
}

struct harbour_runtime_Value harbour_builtin_space(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    long long repeat_count;
    char *buffer;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        !harbour_try_truncated_repeat_count(arguments[0], &repeat_count)
    ) {
        return harbour_value_error_literal("BASE 1105 Argument error (SPACE)");
    }

    if (repeat_count <= 0) {
        return harbour_value_from_string_literal("");
    }

    if ((size_t) repeat_count == (size_t) -1) {
        return harbour_value_error_literal("BASE 1234 String overflow (SPACE)");
    }

    buffer = (char *) malloc((size_t) repeat_count + 1);
    if (buffer == NULL) {
        return harbour_value_error_literal("BASE 1234 String overflow (SPACE)");
    }

    memset(buffer, ' ', (size_t) repeat_count);
    buffer[repeat_count] = '\0';
    return harbour_value_from_string_literal(buffer);
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

static harbour_runtime_Value harbour_string_value_from_owned_buffer(const char *text) {
    size_t length;
    char *owned;

    if (text == NULL) {
        return harbour_value_from_string_literal("");
    }

    length = strlen(text);
    owned = (char *) malloc(length + 1);
    if (owned == NULL) {
        return harbour_value_from_string_literal("");
    }

    memcpy(owned, text, length + 1);
    return harbour_value_from_string_literal(owned);
}

static _Bool harbour_try_str_integer_arg(
    harbour_runtime_Value value,
    long long *result
) {
    if (result == NULL) {
        return 0;
    }

    if (value.kind == HARBOUR_VALUE_INTEGER) {
        *result = value.as.integer;
        return 1;
    }

    if (value.kind == HARBOUR_VALUE_FLOAT) {
        *result = (long long) value.as.floating;
        return 1;
    }

    return 0;
}

static harbour_runtime_Value harbour_str_apply_width(
    const char *formatted,
    long long width,
    _Bool explicit_width
) {
    size_t length;
    size_t target_width;
    char *buffer;

    if (formatted == NULL) {
        return harbour_value_from_string_literal("");
    }

    length = strlen(formatted);
    if (!explicit_width) {
        target_width = 10;
        if (length >= target_width) {
            return harbour_string_value_from_owned_buffer(formatted);
        }
    } else if (width <= 0) {
        return harbour_string_value_from_owned_buffer(formatted);
    } else {
        target_width = (size_t) width;
        if (length > target_width) {
            buffer = (char *) malloc(target_width + 1);
            if (buffer == NULL) {
                return harbour_value_from_string_literal("");
            }

            memset(buffer, '*', target_width);
            buffer[target_width] = '\0';
            return harbour_value_from_string_literal(buffer);
        }
    }

    buffer = (char *) malloc(target_width + 1);
    if (buffer == NULL) {
        return harbour_value_from_string_literal("");
    }

    memset(buffer, ' ', target_width);
    memcpy(buffer + (target_width - length), formatted, length);
    buffer[target_width] = '\0';
    return harbour_value_from_string_literal(buffer);
}

static harbour_runtime_Value harbour_str_non_finite_placeholder(
    long long width,
    _Bool explicit_width
) {
    size_t target_width;
    char *buffer;

    if (!explicit_width) {
        target_width = 23;
    } else if (width <= 0) {
        target_width = 10;
    } else {
        target_width = (size_t) width;
    }

    buffer = (char *) malloc(target_width + 1);
    if (buffer == NULL) {
        return harbour_value_from_string_literal("");
    }

    memset(buffer, '*', target_width);
    buffer[target_width] = '\0';
    return harbour_value_from_string_literal(buffer);
}

static harbour_runtime_Value harbour_str_default_numeric(harbour_runtime_Value value) {
    char buffer[128];

    if (value.kind == HARBOUR_VALUE_INTEGER) {
        snprintf(buffer, sizeof(buffer), "%lld", value.as.integer);
        return harbour_str_apply_width(buffer, 10, 0);
    }

    if (value.kind == HARBOUR_VALUE_FLOAT) {
        size_t length;

        snprintf(buffer, sizeof(buffer), "%.15f", value.as.floating);
        length = strlen(buffer);
        while (length > 0 && buffer[length - 1] == '0') {
            buffer[--length] = '\0';
        }
        if (length > 0 && buffer[length - 1] == '.') {
            buffer[length++] = '0';
            buffer[length] = '\0';
        }
        return harbour_str_apply_width(buffer, 10, 0);
    }

    return harbour_value_error_literal("BASE 1099 Argument error (STR)");
}

static harbour_runtime_Value harbour_val_parse_string(const char *text) {
    const unsigned char *cursor = (const unsigned char *) text;
    double sign = 1.0;
    char integer_buffer[128];
    char fraction_buffer[128];
    size_t integer_length = 0;
    size_t fraction_length = 0;
    _Bool saw_fraction = 0;
    char numeric_buffer[260];
    double parsed;

    while (*cursor != '\0' && isspace(*cursor)) {
        cursor++;
    }

    if (*cursor == '-') {
        sign = -1.0;
        cursor++;
    } else if (*cursor == '+') {
        cursor++;
    }

    while (*cursor != '\0' && isdigit(*cursor)) {
        if (integer_length + 1 < sizeof(integer_buffer)) {
            integer_buffer[integer_length++] = (char) *cursor;
        }
        cursor++;
    }
    integer_buffer[integer_length] = '\0';

    if (*cursor == '.') {
        cursor++;
        while (*cursor != '\0' && isdigit(*cursor)) {
            saw_fraction = 1;
            if (fraction_length + 1 < sizeof(fraction_buffer)) {
                fraction_buffer[fraction_length++] = (char) *cursor;
            }
            cursor++;
        }
    }
    fraction_buffer[fraction_length] = '\0';

    if (integer_length == 0 && !saw_fraction) {
        return harbour_value_from_integer(0);
    }

    if (saw_fraction) {
        if (integer_length == 0) {
            snprintf(
                numeric_buffer,
                sizeof(numeric_buffer),
                "0.%s",
                fraction_buffer
            );
        } else {
            snprintf(
                numeric_buffer,
                sizeof(numeric_buffer),
                "%s.%s",
                integer_buffer,
                fraction_buffer
            );
        }

        parsed = strtod(numeric_buffer, NULL) * sign;
        if (parsed == 0.0) {
            return harbour_value_from_float(0.0);
        }
        return harbour_value_from_float(parsed);
    }

    parsed = strtod(integer_buffer, NULL);
    return harbour_value_from_integer((long long) (parsed * sign));
}

struct harbour_runtime_Value harbour_builtin_valtype(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    if (arguments == NULL || argument_count == 0) {
        return harbour_value_from_string_literal("U");
    }

    switch (arguments[0].kind) {
        case HARBOUR_VALUE_NIL:
            return harbour_value_from_string_literal("U");
        case HARBOUR_VALUE_LOGICAL:
            return harbour_value_from_string_literal("L");
        case HARBOUR_VALUE_INTEGER:
        case HARBOUR_VALUE_FLOAT:
            return harbour_value_from_string_literal("N");
        case HARBOUR_VALUE_STRING:
            return harbour_value_from_string_literal("C");
        case HARBOUR_VALUE_ARRAY:
            return harbour_value_from_string_literal("A");
        case HARBOUR_VALUE_ERROR:
            return harbour_value_from_string_literal("U");
    }

    return harbour_value_from_string_literal("U");
}

struct harbour_runtime_Value harbour_builtin_type(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    const char *source;
    size_t source_length;

    if (
        arguments == NULL ||
        argument_count == 0 ||
        arguments[0].kind != HARBOUR_VALUE_STRING
    ) {
        return harbour_value_error_literal("BASE 1121 Argument error (TYPE)");
    }

    source = harbour_type_trim_ascii(arguments[0].as.string, &source_length);
    if (source_length == 0) {
        return harbour_value_from_string_literal("U");
    }

    if (
        harbour_type_matches_ascii_case_insensitive(source, source_length, "NIL") ||
        harbour_type_matches_ascii_case_insensitive(source, source_length, ".T.") ||
        harbour_type_matches_ascii_case_insensitive(source, source_length, ".F.")
    ) {
        return harbour_value_from_string_literal(
            harbour_type_matches_ascii_case_insensitive(source, source_length, "NIL")
                ? "U"
                : "L"
        );
    }

    if (harbour_type_string_is_numeric(source, source_length)) {
        return harbour_value_from_string_literal("N");
    }

    if (harbour_type_string_is_quoted(source, source_length)) {
        return harbour_value_from_string_literal("C");
    }

    if (harbour_type_string_is_array_literal(source, source_length)) {
        return harbour_value_from_string_literal("A");
    }

    return harbour_value_from_string_literal("U");
}

struct harbour_runtime_Value harbour_builtin_empty(
    const struct harbour_runtime_Value *arguments,
    size_t argument_count
) {
    harbour_runtime_Value value;

    if (arguments == NULL || argument_count == 0) {
        return harbour_value_from_logical(1);
    }

    value = arguments[0];
    switch (value.kind) {
        case HARBOUR_VALUE_NIL:
            return harbour_value_from_logical(1);
        case HARBOUR_VALUE_LOGICAL:
            return harbour_value_from_logical(!value.as.logical);
        case HARBOUR_VALUE_INTEGER:
            return harbour_value_from_logical(value.as.integer == 0);
        case HARBOUR_VALUE_FLOAT:
            return harbour_value_from_logical(value.as.floating == 0.0);
        case HARBOUR_VALUE_STRING:
            return harbour_value_from_logical(harbour_string_is_empty(value.as.string));
        case HARBOUR_VALUE_ARRAY:
            return harbour_value_from_logical(value.as.array.length == 0);
        case HARBOUR_VALUE_ERROR:
            return harbour_value_from_logical(1);
    }

    return harbour_value_from_logical(1);
}

static _Bool harbour_string_is_empty(const char *text) {
    const unsigned char *current;

    if (text == NULL) {
        return 1;
    }

    current = (const unsigned char *) text;
    while (*current != '\0') {
        if (!isspace(*current)) {
            return 0;
        }
        ++current;
    }

    return 1;
}

static const char *harbour_type_trim_ascii(const char *text, size_t *length) {
    const char *start = text;
    const char *end;

    if (text == NULL) {
        if (length != NULL) {
            *length = 0;
        }
        return "";
    }

    while (*start != '\0' && isspace((unsigned char) *start)) {
        ++start;
    }

    end = start + strlen(start);
    while (end > start && isspace((unsigned char) end[-1])) {
        --end;
    }

    if (length != NULL) {
        *length = (size_t) (end - start);
    }

    return start;
}

static _Bool harbour_type_matches_ascii_case_insensitive(
    const char *text,
    size_t length,
    const char *expected
) {
    size_t index;

    if (strlen(expected) != length) {
        return 0;
    }

    for (index = 0; index < length; ++index) {
        if (
            toupper((unsigned char) text[index]) !=
            toupper((unsigned char) expected[index])
        ) {
            return 0;
        }
    }

    return 1;
}

static _Bool harbour_type_string_is_numeric(const char *text, size_t length) {
    size_t index = 0;
    _Bool saw_dot = 0;
    _Bool saw_digit = 0;

    if (length == 0) {
        return 0;
    }

    if (text[index] == '+' || text[index] == '-') {
        ++index;
    }

    if (index >= length) {
        return 0;
    }

    if (text[index] == '.') {
        saw_dot = 1;
        ++index;
        if (index >= length) {
            return 0;
        }
    }

    for (; index < length; ++index) {
        if (isdigit((unsigned char) text[index])) {
            saw_digit = 1;
            continue;
        }

        if (!saw_dot && text[index] == '.') {
            saw_dot = 1;
            if (index + 1 >= length) {
                return 0;
            }
            continue;
        }

        return 0;
    }

    return saw_digit;
}

static _Bool harbour_type_string_is_quoted(const char *text, size_t length) {
    if (length < 2) {
        return 0;
    }

    return (
        (text[0] == '"' && text[length - 1] == '"') ||
        (text[0] == '\'' && text[length - 1] == '\'')
    );
}

static _Bool harbour_type_string_is_array_literal(const char *text, size_t length) {
    return length >= 2 && text[0] == '{' && text[length - 1] == '}';
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

static _Bool harbour_try_max_min_compare(
    harbour_runtime_Value left,
    harbour_runtime_Value right,
    int *comparison
) {
    double left_number;
    double right_number;

    if (comparison == NULL) {
        return 0;
    }

    if (harbour_try_numeric_pair(left, right, &left_number, &right_number)) {
        if (left_number < right_number) {
            *comparison = -1;
        } else if (left_number > right_number) {
            *comparison = 1;
        } else {
            *comparison = 0;
        }
        return 1;
    }

    if (left.kind == HARBOUR_VALUE_LOGICAL && right.kind == HARBOUR_VALUE_LOGICAL) {
        if (left.as.logical == right.as.logical) {
            *comparison = 0;
        } else if (!left.as.logical && right.as.logical) {
            *comparison = -1;
        } else {
            *comparison = 1;
        }
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

static _Bool harbour_try_truncated_repeat_count(
    harbour_runtime_Value value,
    long long *count
) {
    if (count == NULL) {
        return 0;
    }

    if (value.kind == HARBOUR_VALUE_INTEGER) {
        *count = value.as.integer;
        return 1;
    }

    if (value.kind == HARBOUR_VALUE_FLOAT) {
        *count = (long long) value.as.floating;
        return 1;
    }

    return 0;
}
