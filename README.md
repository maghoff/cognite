# Cognite Chat

Programming test for my application to Cognite.

For the given timeframe of 2 hours, I picked a rather ambitious project,
attempting to implement a working client and server side for a multiperson
chat application.

Overview:
- There is one global chat room. Messages have a numeric unique ID and a text
- Server side in Rust, using Warp and Tokio
   - Message history is an in-memory vector (state lost on restart)
   - There are two endpoints:
      - GET /messages: Yields a JSON array of messages
      - POST /add: Accepts a JSON formated message `{ id: number, text: string
        }` which gets added to the list of messages
- Client side in TypeScript, using React
   - Reads the list of messages from `/messages` on load
   - Updates the client side state when attempting to send messages, but I did
     not have the time to propagate this to the server side
   - Client side must be refreshed to see new messages that have been added

Further improvements possible:
- Propagate messages from the client to the server
   - As a nice hack, reload messages from the server when having sent a message
- Sort out the message IDs. The IDs are necessary for React to have a `key`,
  but I'm not sure I need them for anything else (currently). Could be the right
  thing in the short term to remove them entirely and generate them from the
  array index in the React code.
- Add senders to the messages
- Web Sockets for server-initiated signalling that there is a new message
- Storage backend on the server side
- Support for multiple rooms (and/or direct messaging)

## Server

```bash
cd server
cargo run
```

Server is now listening on <http://0.0.0.0:5555>.

Test with `curl`:

    curl http://localhost:5555/messages

Send a message with `curl`. You must create a unique ID:

    curl http://localhost:5555/add \
        -H Content-Type:application/json \
        -d '{"id":3,"text":"System working?"}'

## Client

```bash
cd client
npm start
```

This will start a development server which proxies some requests to the
server side. It is listening on <http://0.0.0.0:3000>.

Test proxying with `curl`:

    curl http://localhost:3000/messages
