# Cryptography I — Programming Assignments in Rust  
> Based on the **Coursera** course [*Cryptography I*](https://www.coursera.org/learn/crypto) by **Stanford University** (Instructor: Prof. Dan Boneh)

This repository contains Rust implementations of the programming assignments from the *Cryptography I* course offered on Coursera.

---

## Assignments Overview

### Week 1: [Many Time Pad][week-1]  
Explore what goes wrong when a stream cipher key is reused. Given multiple ciphertexts encrypted with the same key, your task is to decrypt the target ciphertext and recover the original message.

### Week 2: [AES in CBC and CTR Modes][week-2]  
Implement AES encryption and decryption in both CBC and CTR modes. Given an AES key and ciphertext pairs, recover the corresponding plaintexts.

### Week 3: [File Authentication with SHA256][week-3]  
Build a file authentication system to allow browsers to verify and stream video files in chunks using SHA256, without needing to download the entire file first.

### Week 4: [Padding Oracle Attack][week-4]  
Carry out a CBC padding oracle attack on a simulated vulnerable website. Given an intercepted URL with ciphertext, decrypt the hidden message.

### Week 5: [Meet-in-the-Middle Attack on Discrete Log][week-5]  
Implement a meet-in-the-middle attack to solve a discrete logarithm problem modulo a prime number. This algorithm runs in approximately √2^40 = 2^20 time complexity.

### Week 6: [RSA Factoring Attack][week-6]  
Break RSA encryption by exploiting weak key generation. Specifically, factor the public modulus _N = pq_ when _p_ and _q_ are too close together.

---

## Directory Structure

Each week’s assignment is located in its own subdirectory:

- `w1-many-time-pad/`
- `w2-aes/`
- `w3-mac/`
- `w4-padding-oracle/`
- `w5-mitm-dlog/`
- `w6-rsa-problem/`

[week-1]: w1-many-time-pad/
[week-2]: w2-aes/
[week-3]: w3-mac/
[week-4]: w4-padding-oracle/
[week-5]: w5-mitm-dlog/
[week-6]: w6-rsa-problem/

---

## Acknowledgements

This project is inspired by the *Cryptography I* course on Coursera, developed by **Stanford University** and taught by **Professor Dan Boneh**.
