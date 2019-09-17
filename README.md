# rksuid-cli
Rust cli for creating new serialized ksuids and deserialized and unpacking ksuids from their base62 encoded string form.

# Usage
```
rksuid-cli create
```
```rust
REPRESENTATION:
        String: 1QyYNBGsajtQywsofgDVq8fEyGa
        Raw: A0ED5444CDAC9DED3C25FDD67FB2247
COMPONENTS:
        Time: Tue, 17 Sep 2019 18:17:40 +0000
        Timestamp: 168744260
        Payload: 4CDAC9DED3C25FDD67FB224798418694
```
```
rksuid-cli inspect 1QtFxXJfPVU6NOwPOsHsaihkm8U
```
```rust
REPRESENTATION:
        String: 1QtFxXJfPVU6NOwPOsHsaihkm8U
        Raw: A0C5C581CE87239D33C9F3E68491291
COMPONENTS:
        Time: Sun, 15 Sep 2019 21:17:12 +0000
        Timestamp: 168582232
        Payload: 1CE87239D33C9F3E684912918D639B8E
```
