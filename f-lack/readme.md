# F-lack

F-lack is a lightweight chat application built with Rust that puts YOU back in control. No subscriptions, no vendor lock-in, just a clean and simple self-hosted chat platform that you own forever. Keep your data on your own server/system.

## Features

- Real-time messaging (near, to allow a lot of optimizations to reduce CPU use).
- File attachments
- Clean, minimalist UI
- Built with performance in mind
- SQLite database for persistence

## Development prerequisites

- Rust (latest nightly version)
- Git

## Allowed technologies

- Rust (latest nightly version)
- SQLite
- Rstml's html! macro for HTML generation
- Raw CSS
- Raw JS
- Pretty much nothing else. 
- The code should as simple as possible, and easily auditable by anyone to verify there are no backdoors or even the potential for malicious code or behavior.

Contributions adding frameworks or "the latest thing" to F-lack will be rejected with extreme prejudice.
If you attempt to add an ORM you will be ridiculed and then banned from the project.

## Development setup

1. Clone the repository & cd into the  f-lack directory.

2. Install dependencies:
cargo install

3. Create/migrate db (you will have to change the permissions of the script to execute):
./f_lack_development_db_init.sh

4. Run the application and develop:
cargo watch -x run

5. Build Linux and Windows binaries (fuck Apple):
./f_lack_release_build.sh

6. Drink a victory beer

7. Repeat