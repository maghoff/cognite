Work log
========

Sunday, May 23.

08:45: Sit down to work.

According to my CV, my distinguishing strength is that I'm able to meaningfully
work full-stack. I will therefore focus on the end-to-end workflow, with the
following milestones:

 1. Basic client-side
 2. Server side, serving the client-side
 3. Some state changes, some way to transfer messages between different client
    sessions. Will probably require manual refresh.
 4. Web Sockets for signalling

Then, I might consider some of these stretch goals:
 * User IDs/names
 * Support for different rooms/direct messages between users
 * Storage backend (SQLite)

08:55: Wifi problems while running `npm install` and similar.
09:00: Wifi problems resolved.

09:00: Grr, unexpected error from `npx create-react-app my-app --template typescript`.

Must try not to get sidetracked by debugging this.

Apparently fixed with `npm cache clean --force`. Cache maybe poisoned from
above mentioned Wifi issues.

09:11: Committed generated client side.

09:42: Committed basic event propagation and state handling in client.
Realising that my React has gotten rusty. Considering whether to ditch the
server side entirely. Hm.

10:00: Starting server side work. LEEEEEEEEERROOY JENKINS!

Plan:

 1. Serve out static list of messages
 2. Update client to read messages from server on load
 3. Add endpoint for posting a new message

10:30: 1 and 2 done

10:40: Dynamic state on the server

10:45: Deadline. `git tag deadline`.

Working on point 3, "Add endpoint for posting a new message"

10:50: Implemented `/add` endpoint. Rounding off.

