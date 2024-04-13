training-cli
============

Strumento da linea di comando per verificare i progressi di un utente di [territoriali.olinfo.it](territoriali.olinfo.it) ([cmsocial](https://github.com/algorithm-ninja/terry)).

Installazione
-------------

Copia la repository in una directory locale ed esegui
`cargo install --path .` dalla directory.

Utilizzo
--------

* `terry-spy [user1] [user2] ...` mostra i progressi degli utenti indicati.

Dipendenze
----------
Questo programma utilizza:
* `colored` ([crates.io](https://crates.io/crates/colored), [github](https://github.com/mackwic/colored))
* `reqwest` ([crates.io](https://crates.io/crates/reqwest), [github](https://github.com/seanmonstar/reqwest))
* `serde_json` ([crates.io](https://crates.io/crates/serde_json), [github](https://github.com/serde-rs/json))