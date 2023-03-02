# RTIC

**Examples used in slides**

See [RTIC.rs](https://rtic.rs) for more info and examples
## APP attribute

```rust
{{#rustdoc_include ./rtic_example.rs:app_attr}}
```

## Resources
```rust
{{#rustdoc_include ./rtic_example.rs:resources}}
```

## Init task
```rust
{{#rustdoc_include ./rtic_example.rs:init}}
```

## Idle task
```rust
{{#rustdoc_include ./rtic_example.rs:idle}}
```

## Hardware task
```rust
{{#rustdoc_include ./rtic_example.rs:hw_task}}
```

## Software task
```rust
{{#rustdoc_include ./rtic_example.rs:sw_task}}
```

## Resource locking
Error
```rust
{{#rustdoc_include ./rtic_example.rs:lock_bad}}
```

OK
```rust
{{#rustdoc_include ./rtic_example.rs:lock_ok}}
```