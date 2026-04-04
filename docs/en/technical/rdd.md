# RDD and DBF

- [English](./rdd.md)
- [Português do Brasil](../../pt-BR/technical/rdd.md)

## Role

The RDD layer provides the project's initial DBF data-access model, inspired by the historical Replaceable Database Driver architecture.

## Current Baseline

The current alpha subset includes:

- a core `Rdd` trait;
- DBF schema parsing;
- navigation primitives;
- field reads for the current supported field types;
- append, update, delete, and recall persistence in the supported DBF subset.

## Design Rules

- keep storage-driver concerns separate from frontend/compiler layers;
- start with a minimal, testable DBF baseline;
- preserve room for future driver replacement and index support.

## Current Status

RDD support is present as a foundation, not as a complete xBase database layer. It is intentionally small, practical, and documented as such.
