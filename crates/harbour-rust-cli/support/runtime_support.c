#include <stdio.h>
#include <string.h>

#include "runtime_support.h"

typedef struct harbour_runtime_Value harbour_runtime_Value;

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
    default:
        return 0;
    }
}

harbour_runtime_Value harbour_value_less_than(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_logical(left.as.integer < right.as.integer);
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
        return harbour_value_from_logical(left_number < right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) < 0);
    }

    return harbour_value_from_logical(0);
}

harbour_runtime_Value harbour_value_less_than_or_equal(
    harbour_runtime_Value left,
    harbour_runtime_Value right
) {
    if (left.kind == HARBOUR_VALUE_INTEGER && right.kind == HARBOUR_VALUE_INTEGER) {
        return harbour_value_from_logical(left.as.integer <= right.as.integer);
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
        return harbour_value_from_logical(left_number <= right_number);
    }

    if (left.kind == HARBOUR_VALUE_STRING && right.kind == HARBOUR_VALUE_STRING) {
        return harbour_value_from_logical(strcmp(left.as.string, right.as.string) <= 0);
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
