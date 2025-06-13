# Assignments for Cryptography I by Stanford University

The programming assignments for Coursera Cryptography I by Stanford University.

### Week 1: [Many Time Pad][week-1]

Let us see what goes wrong when a stream cipher key is used more than once. Given ciphertexts encrypted with the same stream cipher key, our goal is to decrypt the target ciphertext to get the secret message.

### Week 2: [AES in CBC and CTR modes][week-2]

Implement two encryption/decryption systems, one using AES in CBC mode and another using AES in counter mode (CTR). Given pairs of AES key and ciphertext, our goal is to recover the plaintext.

### Week 3: [File Authentication System with SHA256][week-3]

Our goal in this project is to build a file authentication system that lets browsers authenticate and play video chunks as they are downloaded without having to wait for the entire file.

### Week 4: [Padding Oracle Attack][week-4]

Let's experiment with a padding oracle attack against a toy website. Knowing that the website is vulnerable to a CBC padding oracle attack, our goal is to decrypt the ciphertext in the sample intercepted URL.

### Week 5: [Meet-in-the-Middle Attack (MITM) on Discrete Log][week-5]

Our goal is to write a program to compute discrete log modulo a prime _p_. In this project, we'll implement an algorithm that runs in time roughly √2^40 = 2^20 using a meet-in-the-middle attack.

### Week 6: [RSA Problem][week-6]

Our goal in this project is to break RSA when the public modulus N is generated incorrectly. If the primes _p_ and _q_ are close to each other, _N = pq_ can be easily factored.

[week-1]: w1-many-time-pad/
[week-2]: w2-aes/
[week-3]: w3-file-auth/
[week-4]: w4-padding-oracle-attack/
[week-5]: w5-mitm-dlog/
[week-6]: w6-rsa-problem/

## Acknowledgements

The Coursera [Cryptography I](https://www.coursera.org/learn/crypto) is offered by Stanford University and taught by Professor Dan Boneh.
