## Morse global interpreter

Listens specific (main) key and interprets it as Morse code (long/short key hold). On pauses between `letters` forms event message and sends it to the user-defined `callback`.

May be configured via `toml` config file.

Implemented only for `Windows` (perform `GetAsyncKeyState` calls for `main_key`)
