# SeqSet

SeqSet is a deterministic and ordered set implementation in Rust that preserves the insertion order of elements.
It combines the efficiency of a HashSet for quick element lookups with the ordered iteration provided by a Vec.
This makes SeqSet ideal for scenarios where the order of elements is important and predictable, such as maintaining 
consistent iteration sequences in compilers, serializers, or any application where order matters.

## Features

- Deterministic Ordering: Maintains the order of elements based on their insertion sequence, ensuring consistent 
iteration order across runs.

- Efficient Lookups: Utilizes a HashSet internally for O(1) average-case element lookups, ensuring fast performance
even with large datasets.

- Comprehensive API: Provides methods for insertion, removal, retrieval, mutation, iteration, and more, mirroring the
familiar HashSet interface.

## Installation

Add `seq-set` to your Cargo.toml dependencies:

```toml
[dependencies]
seq-set = "0.0.1"
```
