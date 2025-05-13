from typing import Tuple

class Falcon512:
    """Falcon-512 post-quantum digital signature scheme implementation.
    
    Provides cryptographic operations using the Falcon-512 algorithm, which is 
    part of the NIST Post-Quantum Cryptography standardization process.
    """
    
    @staticmethod
    def generate_keypair() -> Tuple[bytes, bytes]:
        """Generate a new Falcon-512 key pair.
        
        Returns:
            Tuple[bytes, bytes]: A tuple containing (public_key, secret_key) where:
                - public_key: 897-byte public key
                - secret_key: 1281-byte secret key
        
        Example:
            >>> pub, priv = Falcon512.generate_keypair()
            >>> len(pub)
            897
            >>> len(priv)
            1281
        """
        ...
    
    @staticmethod
    def detached_sign(secret_key: bytes, message: bytes) -> bytes:
        """Create a detached signature for a message.
        
        Args:
            secret_key: The 1281-byte secret key
            message: The message to sign (arbitrary length)
        
        Returns:
            bytes: The 666-byte signature
        
        Raises:
            ValueError: If secret_key is invalid
            
        Example:
            >>> sig = Falcon512.detached_sign(priv_key, b"Hello world")
            >>> len(sig)
            666
        """
        ...
    
    @staticmethod
    def verify_sign(signed_message: bytes, public_key: bytes) -> bytes:
        """Verify a signed message and return the original message.
        
        Args:
            signed_message: The signed message (message + signature)
            public_key: The 897-byte public key
        
        Returns:
            bytes: The original message if verification succeeds
        
        Raises:
            ValueError: If public_key or signed_message is invalid
            RuntimeError: If verification fails
            
        Example:
            >>> msg = Falcon512.verify_sign(signed_msg, pub_key)
        """
        ...
    
    @staticmethod
    def sign_message(message: bytes, secret_key: bytes) -> bytes:
        """Sign a message and return the combined signed message.
        
        Args:
            message: The message to sign
            secret_key: The 1281-byte secret key
        
        Returns:
            bytes: The signed message (message + signature)
            
        Example:
            >>> signed = Falcon512.sign_message(b"Hello", priv_key)
        """
        ...
    
    @staticmethod
    def verify_detached_sign(signature: bytes, message: bytes, public_key: bytes) -> bool:
        """Verify a detached signature.
        
        Args:
            signature: The 666-byte signature to verify
            message: The original message
            public_key: The 897-byte public key
        
        Returns:
            bool: True if signature is valid, False otherwise
        
        Raises:
            ValueError: If signature or public_key is invalid
            
        Example:
            >>> valid = Falcon512.verify_detached_sign(sig, b"Hello", pub_key)
        """
        ...

class Falcon1024:
    """Falcon-1024 post-quantum digital signature scheme implementation.
    
    Provides cryptographic operations using the Falcon-1024 algorithm (stronger variant
    with larger keys and signatures than Falcon-512).
    """
    
    @staticmethod
    def generate_keypair() -> Tuple[bytes, bytes]:
        """Generate a new Falcon-1024 key pair.
        
        Returns:
            Tuple[bytes, bytes]: A tuple containing (public_key, secret_key) where:
                - public_key: 1793-byte public key
                - secret_key: 2305-byte secret key
        """
        ...
    
    @staticmethod
    def detached_sign(secret_key: bytes, message: bytes) -> bytes:
        """Create a detached signature for a message using Falcon-1024.
        
        Args:
            secret_key: The 2305-byte secret key
            message: The message to sign
        
        Returns:
            bytes: The 1280-byte signature
        """
        ...
    
    @staticmethod
    def verify_sign(signed_message: bytes, public_key: bytes) -> bytes:
        """Verify a signed message and return the original message.
        
        Args:
            signed_message: The signed message (message + signature)
            public_key: The 1793-byte public key
        
        Returns:
            bytes: The original message if verification succeeds
        """
        ...
    
    @staticmethod
    def sign_message(message: bytes, secret_key: bytes) -> bytes:
        """Sign a message and return the combined signed message.
        
        Args:
            message: The message to sign
            secret_key: The 2305-byte secret key
        
        Returns:
            bytes: The signed message (message + signature)
        """
        ...
    
    @staticmethod
    def verify_detached_sign(signature: bytes, message: bytes, public_key: bytes) -> bool:
        """Verify a detached Falcon-1024 signature.
        
        Args:
            signature: The 1280-byte signature to verify
            message: The original message
            public_key: The 1793-byte public key
        
        Returns:
            bool: True if signature is valid
        """
        ...