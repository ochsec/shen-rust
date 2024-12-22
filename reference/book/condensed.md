# Shen Kernel Specification

## Core Language Design
- Minimal functional language with 50 primitive functions
- Compilation target: Kλ (Kappa Lambda)
- Supports higher-order functions and currying
- Pattern-matching function definitions
- Non-strict evaluation strategy
- Recursive type system

## Primitive Function Categories
1. Boolean Operations
   - if, and, or, not
   - Supports non-strict evaluation

2. Computational Primitives
   - Numeric operations (+, -, *, /)
   - Comparison operations (>, <, >=, <=)
   - Type checking (number?, symbol?, string?)

3. List Manipulation
   - cons, hd, tl
   - append, map
   - Pattern-matching list destructuring

4. Control Flow
   - Recursion with tail-call optimization
   - Non-deterministic choice points
   - Backtracking support

5. Type System
   - Static, strong typing
   - Sequent calculus type inference
   - Polymorphic type operators
   - Recursive type definitions

6. Metaprogramming
   - Macro expansion
   - eval and eval-kl functions
   - Package system for namespace management

## Compilation Strategy
- Translate Shen code to Kλ
- Use triple-stack method for pattern matching
- Perform compile-time type checking
- Support platform-independent compilation

## Key Language Invariants
- All functions compilable to Kλ
- Type safety
- Referential transparency
- Minimal core with extensible type system

## Semantic Foundations
- Based on lambda calculus
- Supports higher-order functions
- Implements call-by-need evaluation
- Provides mechanism for defining new type systems
