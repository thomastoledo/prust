# Prust
Projet d'apprentissage de Rust et de webassembly via une application P2P.
Chaque developpement est live sur [twitch](https://www.twitch.tv/imflog).

Les features envisagees:
* Chat textuel
* Chat video
* Partage de fichier
* Wizz :heart:

# Stream 1
## Realisation
* Notre projet
    * P2P chat / video / file share en Rust compilation en webassembly (wasm)
    * Apprentissage de Rust
    * Apprentissage de webassembly

* Installation RUST
    * [Rust](https://www.rust-lang.org/tools/install)
    * [VSCode RLS](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)
    * [Standard types](https://doc.rust-lang.org/1.2.0/std/index.html)

* Rust
    * Traits: 
    * Closures:

* [YEW](https://yew.rs/)
    * [Setup](https://yew.rs/docs/getting-started/project-setup)
    * [Cargo web](https://github.com/koute/cargo-web)
    * [Composants](https://yew.rs/docs/concepts/components)
    * [Callbacks / Messages](https://yew.rs/docs/concepts/components/callbacks)

## Problemes
RAS

# Stream 2
## Plan
* Design HTML / CSS
* Hierarchie de composants
* Passage d'informations entre composants

## Realisations
* Design complet page (chat mode mais pas beau)
    * [Decoupage et references entre composants](https://yew.rs/docs/concepts/html/components)

* Exploration CSS / YEW
    * [static files embarqués par cargo web](https://github.com/koute/cargo-web#static-files)
    * [CSS in rust a explorer](https://github.com/lukidoescode/css-in-rust)

* Structure pour l'envoi de message
    * Recuperation du contenu du textarea via [NodeRef](https://yew.rs/docs/concepts/components/refs) (implique cast en HtmlInputElement)
    * Recuperation possible du content via une reference directe [ex](https://github.com/yewstack/yew/blob/master/examples/textarea/)

* Rust
    * [Derive](https://doc.rust-lang.org/reference/attributes/derive.html)
    * [Pattern Matching](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html)
    * [Option](https://doc.rust-lang.org/std/option/)
    * [String vs str](https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html)

## Problemes
* Console logging => Ne semble pas fonctionner avec std-web

* [web-sys VS std-web](https://yew.rs/docs/getting-started/choose-web-library)

* Cargo-web
    * Ne fonctionne pas avec web-sys
    * Derniere MAJ il y a 9 mois

* Wasm-bindgen / Wasm-pack
    * Support par [rustwasm group](https://rustwasm.github.io/)
    * Fonctionne avec web-sys
    * Derniere maj il y a 30min

* Plus possible de lancer le projet dans l'etat actuel

# Stream 3
## Intersteam :cow:
* Explication rework vers web-sys
    * Installation de wasm bindgen : `cargo install wasm-bindgen-cli`
    * Remove de cargo-web `cargo uninstall cargo-web` :pray:
    * Change the Cargo.toml to use websys + wasm-bindgen
    * `cargo build --features "console_error_panic_hook" --target wasm32-unknown-unknown`
    * `wasm-bindgen --target web --no-typescript --out-dir static/ --out-name app target/wasm32-unknown-unknown/debug/prust.wasm`
    * cd static && python3 -m http.server
* Console Logging / Debugging

## Plan
* Rework textarea via reference
* Communication entre composants => envoyer le contenu du textarea dans le composant de conversation
    * Utilisation d'un systeme d'acteurs ?

## Realisation
* Tooling
    * (Cargo watch)[https://github.com/passcod/cargo-watch] => Now !
    * (Cargo makefile)[https://github.com/sagiegurari/cargo-make] => Soon ?
    * Notre commande watch `cargo watch -x 'build --features "console_error_panic_hook" --target wasm32-unknown-unknown' -s 'wasm-bindgen --target web --no-typescript --out-dir static/ --out-name app target/wasm32-unknown-unknown/debug/prust.wasm'`

* Communication entre composants => envoyer le contenu du textarea dans le composant de conversation
    * https://yew.rs/docs/concepts/components/callbacks#callbacks => Now !
    * https://yew.rs/docs/concepts/agents => Soon ?

## Problemes
* Gestion des callbacks parent / enfant
    * Regarder plus en détail ce qu'est un Callback en RUST
    * Piste : (Elm}[https://elm-lang.org/] (Yew est apparemment inspiré de Elm)
    * (Test in doc)[https://doc.rust-lang.org/rustdoc/documentation-tests.html]

# Stream 4
## Plan
* Callback enfant1 => parent => enfant2