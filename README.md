# TLS | Transaction Ledger System
### The easy to learn version control system, made fully in rust.
**How to use:**
-
### Initialize your project's root directory:
> ```
>tls -i
>```
>or
>```
>tls --init
>```
### Stage your files:
>```
>tls -a ./
>```
>or 
>```
>tls --add ./
>```
### Commit your changes:
>```
>tls -c "a commit message"
>```
>or 
>```
>tls --commit "a commit message"
>```
## Our commit message system
The commit messages seen in this github repository are descriptors of how big the commit is, what type of commit was pushed, and its 4 digit id in this format: <M/m>\<id>, so Mn0520 means that the commit is major, the commit was a new feature, and it was the 520th commit. Note that this is different to our versioning system which uses a M.m.f system (Major.minor.fix).