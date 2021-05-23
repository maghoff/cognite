# Cognite Chat

Programming test for my application to Cognite.

## Server

```bash
cd server
cargo run
```

Server is now listening on <http://0.0.0.0:5555>.

Test with `curl`:

    curl http://localhost:5555/messages

Send a message with `curl`:

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
