# ARTIE Common Library for Artie Distance Calculation

## Description

This Rust library provides functionality to calculate the "Artie distance," a measure of dissimilarity between two hierarchical structures, such as those represented by workspaces in programming environments. The Artie distance encompasses several metrics: family, block, position, and input distances, each measuring different aspects of difference.

## Features

- **Family Distance**: Counts the unique families present in one workspace but absent in the other.
- **Block Distance**: Measures the number of uniquely named blocks that do not match between workspaces.
- **Position Distance**: Aggregates positional differences for blocks with the same name but located in different places within the two workspaces.
- **Input Distance**: For common blocks, this metric calculates the absolute difference in numerical fields and counts mismatches in non-numerical fields.

## Installation

Include the following in your `Cargo.toml` file to use this library:

```toml
[dependencies]
artie_common = "0.2.0"
