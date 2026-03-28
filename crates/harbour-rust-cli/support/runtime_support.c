#include <stdio.h>

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
