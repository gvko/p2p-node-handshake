# p2p-node-handshake

## Quickstart
As a target node I have used a [Multichain](https://www.multichain.com/) node, running locally inside a Docker container. I reused a
solution that I have developed several years ago, where I bootstrapped
a Docker image of the Multichain instance myself. You can see the
full github repo [here](https://github.com/gvko/multichain-db).

1. Create an `.env` file in the root dir and copy the contents of `.env.example` into `.env`. You can set different password, but then you have to change that in the `docker-compose.yml` file in the next step. (That's to be improved, but is out of the current scope).
2. `cd` to the `multichain-node` dir and run `docker-compose up`. This will start the Multichain node container, to which the app will connect later.
3. back in the root dir just execute `cargo run`, which will start the app and run the handshake logic.

## Assumptions and Considerations

Ideally, based on my understanding, a full-protocol handshake should 
follow something like the following steps, where the "client" is
my app and the "server" is the target node:

1. Handshake Initiation: send a hello/info message to the server to
initiate the handshake. The server will probably reply with a message
containing its version and protocol information
2. Public Key Exchange: send a message to the server to obtain its 
server public address. Then send a message to the server to get the
public key from the address.
3. Send the client's public key to the server, thus performing a 
full public key exchange.
4. Send a message to the server, encrypted with the client's private
key. The server should return some kind of success message, 
acknowledging that it can decrypt the client's messages.
5. Send another message, e.g. already getting some actual data, 
encrypted with the client's key, and the server returning data
encrypted with its key. The client already having the server's
public key can decrypt the message.

At this point, step 5. would indicate that the full-protocol handshake has been
successful.

In order to test the solutin, I've tried different targe nodes:
* I've tried setting up my Electrumx node, but that has proven too time-consuming and way out of the scope of the task
* Then I tried using an Alchemy rpc api as a target node, but they do not support endpoints that would allow for a full-protocol, post-TCP handshake implementation. All the endpoints are user-facing, focused on sending/reading data to/from the blockchain.
* Finally, I set up my own Multichain node inside a docker container,
since they have a decent [RPC API documentation](https://www.multichain.com/developers/json-rpc-api/), which I could use for the
purposes of this task. Unfortunately, they do not expose endpoints
that allow for public key exchange and data encryption, so I could not
implement steps 3-5 from the full-protocol handshake.

In order to address the *Security* part of the *Evaluation*, it would
be ideal to send the calls over HTTPS instead of plain HTTP, but that
would require additional setup in the Multichain node, which would
be very out of the scope of the task.