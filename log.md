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

10:45: Deadline
