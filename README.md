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
## Plan / Realisation
* Callback enfant1 => parent
    * example: https://github.com/yewstack/yew/tree/master/examples/node_refs
    * Utilisation de callback et de emit() pour passer les informations entre les composants
* Callback parent => enfant2
    * Utilisation des props pour trigger la méthode `change` du composant enfant
    * [change](https://yew.rs/docs/concepts/components#change)
* [Html list display](https://yew.rs/docs/concepts/html/lists)

* Rust
    * [LinkedList](https://doc.rust-lang.org/std/collections/struct.LinkedList.html)
    * Vectors [doc](https://doc.rust-lang.org/std/vec/struct.Vec.html) [rustbook](https://doc.rust-lang.org/book/ch08-01-vectors.html)

## Problemes
RAS

# Stream 5
## Plan
* If let cleanup
* Nettoyer Input text on send => set_value sur node_ref.
* CTRL + ENTER pour envoyer un message (ENTER ne fait pas d'erreurs console)
* Formatting des messages via HTML dans conversation.rs
    * Structure custom pour afficher le sender (moi / autre)
    * Historisation ? Page de list de message par ex.

## Realisation
* Yew
    * [KeyboardEvent](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.KeyboardEvent.html)
    * [Children & ChildrenRenderer](https://yew.rs/docs/concepts/html/components)

* Rust
    * [If let](https://doc.rust-lang.org/stable/rust-by-example/flow_control/if_let.html)
    * Pour voir la doc de notre projet et ses dépendances : `cargo doc --open`
    * [Display trait](https://doc.rust-lang.org/std/fmt/trait.Display.html)

## Problèmes
* Formating des messages via HTML

# Stream 6
## Intersteam :cow:
* Split main / lib

## Plan
* Formatting messages HTML en fonction de la provenance
* Mock envoit message reseau
* Un peu de borrowing
* Discussion autour du P2P et next steps
    * [Le Hole Punching](https://en.wikipedia.org/wiki/Hole_punching_%28networking%29)
    * [libp2p](https://docs.libp2p.io/concepts/transport/)

## Réalisation
* Code cleaning
* Utiliser une implémentation sans le trait Component pour ChatMessage
* Gérer les lifetimes https://doc.rust-lang.org/1.9.0/book/lifetimes.html

# Stream 7
## Plan
* P2P code

## Réalisation
* Récap WebRTC
    * [ICE](https://developer.mozilla.org/en-US/docs/Glossary/ICE)
    * [Signaling Server](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Signaling_and_video_calling)
    * [SDP](https://developer.mozilla.org/en-US/docs/Glossary/SDP)
    * [Introduction to WebRTC Protocol](https://developer.mozilla.org/en-US/docs/Web/API/WebRTC_API/Protocols)
    * [https://github.com/simmons/webrtc-sctp](https://github.com/simmons/webrtc-sctp)
    * [https://github.com/rasviitanen/rustysignal](https://github.com/rasviitanen/rustysignal)
* Création d'une RtcPeerConnection
    * [RtcPeerConnection](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcPeerConnection.html)
    * [RtcConfiguration](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcConfiguration.html)
    * [RtcIceServer](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.RtcIceServer.html)

# Stream 8
## Interstream :cow:
* Garder les sauts de ligne

## Plan
* Full connexion WebRTC

## Réalisation
* Create offer call
    * [Closure](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/closure/struct.Closure.html)

# Stream 9
## Plan
* Re-explication diagramme de sequence
* Finalisation create_offer et explication lifetime / Box
* Le plan de la base du feu

## Réalisation
* parsing d'une JSValue vers un objet RTCSessionInitDescription dans le but de set la local_description

# Stream 10
## Plan
* [Pokiro](https://gitlab.com/AurelienR/pokiro/)
* Problème de lifetime modification de la connection dans closure

## Réalisation
* Lifetime
    * [Rc](https://doc.rust-lang.org/std/rc/index.html)
    * [RefCell](https://doc.rust-lang.org/std/cell/struct.RefCell.html)

# Stream 11
## Plan
* Envoi offre
    * Ajouter les éléments pour le signaling manuel

# Stream 12
## Plan
* Signaling server WebRTC, Thomas nous dit tout
* Coder en Rust, après les vacances ...
* Nouvelles interfaces

## Réalisation
* [Explications WebRTC](https://github.com/nugetchar/signaling-server-poc)
* Début de la structure de [modules](https://doc.rust-lang.org/reference/items/modules.html)
* Début du composant de connexion

# Stream 13
## Plan
* Wtf : Recursive limit ?
    * https://yew.rs/docs/en/concepts/html/
* Belle structure de module
    * OK
* Début Websocket + WebRTC !
    * https://rustwasm.github.io/docs/wasm-bindgen/examples/websockets.html
    * OnOpen / OnMessage ... 
        * https://docs.rs/web-sys/0.3.45/web_sys/struct.WebSocket.html
        * https://github.com/rustwasm/wasm-bindgen

# Stream 14
## Realisation
* Presentation modifications server
* WebSocket => Ajout des premiers messages
* Serialisation:
    * Serde: https://serde.rs/
    * Deserialization en fonction du type: https://serde.rs/enum-representations.html#externally-tagged
* Refacto dans utils

# Stream 15
## Plan
* améliorer la lisibilité du code (gestion des closures)
* type alias sur les cast des closures
* continuer la gestion des websocket (améliorer la serde)

## Realisation
* Misc
    * gsv => (repo git comming soon)
    * Refacto [Clap](https://docs.rs/clap/2.33.3/clap/) / [Structopt](https://docs.rs/structopt/0.3.20/structopt/)

* PRUST
    * Parsing du message reçu : dyn_into => JsString
    * Parsing de la JsString avec `serde_json::from_str::` => nos types

# Stream 16
## Interstream
* New way to start the project
    * [cargo make](https://github.com/sagiegurari/cargo-make)
    * `cargo make --makefile Makefile.toml dev`
    * `cargo make --makefile Makefile.toml serve`

## Plan
* type alias sur les cast des closures
* améliorer serde (implement From for SocketMessage ?)
* continuer la gestion des websocket
* améliorer la lisibilité du code (gestion des closures) // Peut on les sortir

## Réalisation
* fix `cargo make`
* type alias ok
* amélioration serde
    * implémentation du trait `TryFrom`
    * management des erreurs custom
    * utilisation de l'opérateur `?`
    * problème Object VS String VS Websockets

# Stream 17
## Interstream
* cargo make concurrence entre build / wasm-bind (lol)
* cleanup des TODOs
* améliorer la lisibilité du code (gestion des closures)

## Plan
* continuer la gestion des websocket

# Stream 18

## Plan
* SDP implement
* Premier message envoyé ?

## Réalisation
* Cleanup handle_message function (trop grosse)
* Ajout de nouveau handlers
* Prise en compte de callbacks WebRTC:
* * `on_ice_candidate_callback`
* * `on_signaling_callback`

# Stream 19
## Plan
* Raffraîchissement [WebRTC](https://media.prod.mdn.mozit.cloud/attachments/2016/01/27/12363/9d667775214ae0422fae606050f60c1e/WebRTC%20-%20Signaling%20Diagram.svg)
* Prise en compte de callbacks WebRTC:
* * `on_negociation_needed`
* Faire l'implémentation de `handle_sdp_message`
* Premier message envoyé ?

## Réalisation
* on_negotiation_needed : OK
* start handle_sdp_message : ONGOING

# Stream 20
## Plan
* Terminer l'implémentation de `handle_sdp_message`
* Premier message envoyé ?

## Réalisation
* handle_sdp_message = quasi OK
* problème référence socket :(

# Stream 21
## Interstream :cow:
* Fix sockets ? OK !
* block_on: NOP NOP NOP
* Problème avec borrow_mut T_T => OK !

## Réalisation
* Correction de la perte de types du RTCSessionDescription 
    * (mapping serde)
    * Trait TryFrom
* Définition de l'expression "sur des chapeaux de roues"

# Stream 22
## Plan
* ICE candidates:
    * https://media.prod.mdn.mozit.cloud/attachments/2016/01/27/12365/b5bcd9ecac08ae0bc89b6a3e08cfe93c/WebRTC%20-%20ICE%20Candidate%20Exchange.svg
    * Faire le mapper pour les ICECandidates (on perd des informations)

## Réalisations
* Ça fucking marche !
* Merci la vie.

# Stream 24
## Plan
* Display du message dans la chatbox
* Présentation du prochain projet

## Réalisation
* Gestion des dates:
    * [Datetime](https://docs.rs/chrono/0.4.0/chrono/struct.DateTime.html) => Ne marche pas en wasm
    * [Js_sys](https://docs.rs/js-sys/0.3.47/js_sys/struct.Date.html)
* Vecteurs:
    * Merge => extends / append
    * Sort => sort_by
* [Utiliser des Closures Rust en JS](https://rustwasm.github.io/docs/wasm-bindgen/reference/passing-rust-closures-to-js.html)
* [Rust Nightly](https://doc.rust-lang.org/nightly/edition-guide/rust-2018/rustup-for-managing-rust-versions.html) => Finalement pas utile car pas de closures Rust/Js
* [Agent Yew](https://github.com/yewstack/yew/blob/master/examples/pub_sub/README.md)

# Ce qu'il reste
* Annoter les étapes ICE
* Cleanup serde-serialize quand possible
* Extraire le code de l'agent dans des fichiers séparés
* Global cleanup (exceptions, handlers via des variables ?, clippy)

:heart: