# rksuid-cli
Rust cli for creating new serialized ksuids and deserialized and unpacking ksuids from their base62 encoded string form.

# Usage
```
rksuid-cli create
```
```rust
Got ksuid: 1Qu4dNIRriAzAzXt4Kb4TyJvg98
 Contents: Ksuid {
    timestamp: 168607233,
    payload: 120430477075427929952839731923742191110,
}
```
```
rksuid-cli inspect 1QtFxXJfPVU6NOwPOsHsaihkm8U
```
```rust
Got ksuid: 1QtFxXJfPVU6NOwPOsHsaihkm8U
 Contents: Ksuid {
    timestamp: 168582232,
    payload: 38425313529232428683585399619468893070,
}
```
