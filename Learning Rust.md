# Introduzione
- Il Rust è un linguaggio di programmazioneoo di basso livello con paradigma molto simile a C/C++
-
- L'esempio di `hello world` in Rust è
- ```rust
  fn main() {
      println!("Hello 🌍!");
  }
  ```
- Possiamo fare le prime osservazioni.
	- La funzione `main` viene dichiarata tramite `fn` il resto del paradigma è molto simile a quello di C/C++
	- Poi troviamo il comando `println!` (print-line) con un punto esclamativo `!`
		- Il punto esclamativo va a definire che quella è una funzione **macro** #igienica
-
- Il vantaggio più importante di Rust è che è un linguaggio di velocità pari a C/C++ ma con una sicurezza all'interno della memoria Unica.
-
- # Variabili e tipizzazione
- Rust è un linguaggio tipizzato, il che significa che dovremmo dichiarare il tipo di ogni variabile.
- > Le variabili sono **IMMUTABILI** (almeno che non gli venga dato il parametro `mut`)
- ```rust
  fn main() {
      let x: i32 = 10;
      println!("x: {x}");
      // x = 20;
      // println!("x: {x}");
  }
  ```
- > OUTPUT `x: 10`
- La dichirazione delle variabili è molto simile a quella di TypeScript.
- Nel caso provassimo a togliere le linee commentate troveremmo un errore di questo genere:
- ```error[E0384]: cannot assign twice to immutable variable `x`
   --> src/main.rs:4:5
    |
  2 |     let x: i32 = 10;
    |         - first assignment to `x`
  3 |     println!("x: {x}");
  4 |     x = 20;
    |     ^^^^^^ cannot assign twice to immutable variable
    |
  help: consider making this binding mutable
    |
  2 |     let mut x: i32 = 10;
    |         +++
  ```
- Rust ci aiuta molto nella comprensione degli errori, difatti ci dice `cannot assign twice to immutable variable` -> `Non puoi asseggnare due volte il valore ad una variabile immutabile`
- Ci aiuta anche presso dei suggerimenti, dicendoci di aggiungere `mut` per far diventare mutabile la variabile e fare in modo che ne possiamo modificare il valore durante l'esecuzione del programma.
- ##  Valori di assegnazione
- Rust in quanto estremamente **memory safe** ci guida in maniera precisa anche nell'assegnazione delle variabili. Infatti troviamo varie Tipologie e non le classiche.
- |  **Descrizione** | **Tipi** | **Esempio** |
  | --- | --- | --- |
  | Interi con segno | `i8`,`i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123i64`|
  | Interi senza segno| Dato B2 | Dato B3 |
  | Numeri in virgola mobile | | |
  | Valori scalari Unicode | | |
  | Booleani | | |
- > https://google.github.io/comprehensive-rust/it/types-and-values/values.html
-
-
-