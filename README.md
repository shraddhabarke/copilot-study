# copilot-study
Thanks for taking part in this study!
We want to understand how people interact with Copilot.
- When do you use Copilot?
- Do you trust its output?
  - If not, what would make you more confident?
- Do you plan your input or let it just work in the background?
- How confident are you about what you get back?

To get at these kinds of questions *and more* we've created a small application,
YOU'RE going to build (with Copilot's help).

You'll be writing a secure instant message application.

## Organization
This project will be a set of tasks to accomplish with copilot.
```
/.
  chat/
    py/   <- python implementation
    rust/ <- rust implementation
```

# Before you begin
You'll need:
- VSCode
- the Copilot plugin for VSCode
- If doing python:
  - pip3
  - run `chat/py$ pip3 install -r requirements.txt`

# Chat Server
MyRC is a secure chat client and server. Messages are encrypted and sent to all
connected clients. Clients pick a username by which they are identified. There
is only one big room for all clients. All clients should have unique usernames.

## Protocol
The server's protocol is simple:
0. Client connects to server. A TCP connection on port 4040 is default.
1. Client and server perform a handshake to establish a shared secret key.
2. Server asks for a username
3. Client gives a username
4. Server sends a welcome message and all connected clients.

Messaging now commences at this point.

A user can quit either with the `/quit` command or by quitting the client (e.g.,
ctrl-c or ctrl-d).

## Handshake
The handshake establishes a shared secret key between the client and server.
This shared key is used as input to an AES-256 cipher in what's called `ECB`
mode. To find the shared secret, the handshake protocol is as follows:
Server -> Client:
{"type":"PrimeDiffieHellman", "key":<public-key-as-int>}
Client -> Server:
{"type":"PrimeDiffieHellman", "key":<public-key-as-int>}

The client and server each can take the other's public key, along with their own
private key, and generate a shared secret key.

## AES Cipher
This shared key is used as the key for an AES-256-ECB cipher.
The cipher takes in a byte string of plaintext and outputs a byte string of
ciphertext. This input to AES in ECB mode needs to be padded so it is always a
multiple of AES.block_size (16 bytes). Remember, this padding will need to be
removed after decryption, too.

All messages after the diffie hellman handshake are encrypted with this AES
cipher. Each client will have a different secret key with the server.

## Commands
A user can type any of the following commands for the desired effect:
/quit : disconnects. Same as ctrl-c or ctrl-d
/list : lists the connected other client usernames
/help : show this list


# Tasks
While implementing these tasks, you're welcome to (but not required to):
- Import extra libraries
- Look on the internet for API information / stackoverflow
- Write tests
- Write extra functions or classes
- Rename the solution files so you can test your work (e.g., renaming
  `crypto.py` and `task_crypto.py` so you can test your crypto work with the
  solution client/server).
- Run the solutions to test against (e.g., testing your client against the
  solution server)

Please DON'T:
- read the solution code. It's in the same directory as your tasks so you can
  test against them, and use the solutions in your imports.

## Task 1 Implement DiffieHellman Key Exchange, 2 ways
The crypto library needs to be implemented.
The superclass `Crypto` uses stubs and relies on its subclasses to actually
implement the unimplemented methods. You do not need to change anything in the
`Crypto` class itself.
### PrimeDiffieHellman
Create an implementation of this crypto class
using the traditional diffie-hellman key exchange protocol,
using prime numbers.
It works by implementing the following math:
Alice and Bob have already agreed on a prime, p, and a group generator number g.
    In this case, use g=2; and p can be any prime of your choosing
    [51,997]. We need small-ish numbers to use Python's built-in math
    operations.
Alice:
    1. pick some number priv_a [2,p-1]
    2. generate pub_a as g^priv_a mod p
Bob:
    1. pick some number priv_b [2,p-1]
    2. generate pub_b as g^priv_b mod p

3. Alice: generate shared secret as (pub_b^priv_a) mod p
3. Bob: generate shared secret as (pub_a^priv_b) mod p

## Task 2: Implement the Client
You have only the main function from the client and need to implement the rest.
You know the client needs to take an IP and a port number, and that it uses TCP.

After establishing a connection to the server, the client needs to facilitate
the diffie-hellman handshake to establish a secure connection. The client then
needs to listen for input from both the secure connection and also from the
console. Content from the server is put on screen and messages from the console
are sent to the server.

Import any packages you need. Remember, the cryptographic module provides the
interfaces and implementations you'll need for the crypto part.

## Task 3 Implement the Server
You need to implement the server. It should be listening for connections on port
4040 over TCP. Any connection it makes, it should perform a handshake; ask for a
unique username; and then send a welcome message to all connected clients. Any
message sent to the server should be sent to all the other clients.

The server should support the following commands:
- /quit: disconnects the client (same as ctrl-d or EOF)
- /list: list all connected clients (this message is only sent to the asking client)
- /help: show this list

As an example exchange:
Server: listening...
Client 1: <connects>
Server: <handshake: sends its public-key>
Client 1: <handshake: sends its public-key>
Server(encrypted to Client 1): Please pick a username:
Client 1: MySuperCoolUsername
Server(to all): Welcome MySuperCoolUsername!
Client 2: <connects; handshakes; picks username>
Client 1: Hello!
Server(to all but Client 1): MySuperCoolUsername: Hello!