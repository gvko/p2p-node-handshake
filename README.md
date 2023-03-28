# p2p-node-handshake

## Quickstart
1. Create an `.env` file in the root dir and copy the contents of `.env.example` into `.env`. You can set different password, but then you have to change that in the `docker-compose.yml` file in the next step. (That's to be improved, but is out of the current scope).
2. `cd` to the `multichain-node` dir and run `docker-compose up`. This will start the Multichain node container, to which the app will connect later.
3. back in the root dir just execute `cargo run`, which will start the app and run the handshake logic.