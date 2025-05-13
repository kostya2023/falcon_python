# Falcon Python Bindings


Python bindings for the Falcon post-quantum signature scheme, implemented in Rust using PyO3.

## Features

- **Two security levels**:
  - Falcon-512 (NIST security level 1)
  - Falcon-1024 (NIST security level 5)
- **Complete functionality**:
  - Key pair generation
  - Message signing (both attached and detached signatures)
  - Signature verification
- **High performance** - Rust implementation provides faster operations than pure Python
- **Type annotated** - full `.pyi` stub file included

## Installation

```bash
pip install falcon-python
```

For building from source:

```bash
pip install maturin
maturin build --release
pip install ./target/wheels/falcon_python-*.whl
```

## Quick Start

```python
from falcon_python import Falcon512, Falcon1024

# Falcon-512 example
pub, priv = Falcon512.generate_keypair()
message = b"Important message"

# Attached signature
signed = Falcon512.sign_message(message, priv)
verified_msg = Falcon512.verify_sign(signed, pub)

# Detached signature
signature = Falcon512.detached_sign(priv, message)
is_valid = Falcon512.verify_detached_sign(signature, message, pub)
```

## Key Sizes

| Algorithm | Public Key | Secret Key | Signature |
|-----------|-----------|------------|-----------|
| Falcon-512 | 897 bytes | 1281 bytes | 666 bytes |
| Falcon-1024 | 1793 bytes | 2305 bytes | 1280 bytes |

## Security Notice

This implementation uses the reference implementation from PQClean. While Falcon has been selected for standardization by NIST, this is still a relatively new cryptographic algorithm. Use in production systems should be done with proper security considerations.

## License

MIT License. Contains code derived from PQClean project (also MIT licensed).