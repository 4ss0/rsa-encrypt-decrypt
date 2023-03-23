# rsa-encrypt-decrypt

This tool allows encrypting messages with a specific public key and, if the user is in possession of the private key, decrypting the message.

# Use case

Alice wants to share a secret with Bob. Both of them have this application. Alice and Bob start the application, and Bob gives Alice the "Public key" he finds in the started program.
Alice puts her secret in the right window and Bob's public key in the corresponding window (overwriting her public key), then presses "Encrypt" and passes the generated encrypted string to Bob.
Bob puts the encrypted string in the corresponding box and clicks "Decrypt" to read Alice's decrypted secret.