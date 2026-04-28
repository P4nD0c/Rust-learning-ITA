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
  | Interi senza segno| `u8`,`u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10u16` |
  | Numeri in virgola mobile | `f32`, `f64` | `3.14`,  `-10.0e20`, `2f32` |
  | Valori scalari Unicode | `char` | `'a'`, `'α'`, `'∞'` |
  | Booleani | `bool` | `true`, `false`|
- > In rust si possono scrivere i numeri più grandi mettendo lo `_` ogni 3 cifre per facilitare la lettura. ex: `1_000`, `123_i64`
-
- ## Aritmetica
- La sintassi matematica è la classica di ogni linguaggio di programmazione. infatti troviamo i soliti operatori aritmetici `+`, `-`, `*`, `/`
- ```rust
  fn interproduct(a: i32, b: i32, c: i32) -> i32 {
      return a * b + b * c + c * a;
  }
  
  fn main() {
      println!("result: {}", interproduct(120, 100, 248));
  }
  ```
-
- ## Le Stringhe
- Possiamo trovare due tipologie di stringhe
- > `String` -> Una stringa modificabile e propria
  > `&str` -> Una stringa di **sola lettura**
- ```rust
  fn main() {
      let greeting: &str = "Greetings";
      let planet: &str = "🪐";
      let mut sentence: Strings = String::new();
      sentence.push_str(greeting);
      sentence.push_str(", ");
      sentence.push_str(planet);
      println!("final sentence: {}", sentence);
      println!("{:?}", &sentence[0..5]);
      //println!("{:?}", &sentence[12..13]); -> Non include tutti i byte di 🪐
      //println!("{:?}", &sentence[11..15]); -> Include tutti i byte di 🪐
  
  }
  ```
- Dal seguente esempio possiamo capire tutto il necessario della manipolazione.
	- le variabili di tipo `&str`; `greeting` e `planet` **non** possono essere modificate.
		- la `&` di fa capire che indica una reference -> Qualcosa di esclusivamente modificabile
	- la variabile `sentence` è di tipo `Strings` ed è stata dichiarata come *mutabile*
	- Per aggiugnere delle stringhe ad una variabile stringa vuota è possibile usare il metodo `.push_str(<variabile/"contenuto">)`
	- E' anche possibile utilizzare le **raw-strings**, ovvero delle stringhe dove non vengono contati i caratteri di esacpe. Il concetto è equivalente a quello di python anche nella forma.
	- > E' oltretutto possibile fare una specie di ~slicing~  ma non è da visionare come quello di python, che si basa su caratteri, ma è da pensare "byte per byte" di conseguenza nell'esempio: `println!("{:?}", &sentence[12..13]);` riceveremmo un errore, perché l'emoji "🪐" non vale 1 singolo byte come ogni carattere, ma ne vale di più, quindi dovremmo includere la sua intera combinazione di byte e includerlo completamente, non parzialmente(come fatto nell'esempio)
-
- ## IF EXPRESSION
- > Le `IF expression` in Rust funzionano in maniera simile a quella di C/C++ e Java.
- ```rust
  fn main() {
      let x = 10;
      if x < 20 {
          println!("small");
      } else if x < 100 {
          println!("biggish");
      } else {
          println!("huge");
      }
  }
  ```
- > OUTPUT: `small`
- La particolarità è che similmente a python si può utilizzare l'IF anche nelle variabili per definire il tutto in una sola linea.
- ```rust
  fn main() {
      let x = 10;
      let size = if x < 20 { "small" } else { "large" };
      println!("number size: {}", size);
  }
  ```
- > OUTPUT: `small`
-
- ## CICLI INTERATTIVI
- ###  `while`
	- Continua a ripetere una porzione di codice finché una condizione non risulta falsa.
	- ```rust
	  fn main() {
	      let mut x = 200;
	      while x >= 10 {
	          x = x / 2;
	      }
	      println!("Final x: {x}");
	  }
	  ```
- ### `for`
	- Ripete una porzione di codice per un determinato range di valute.
	- ```rust
	  fn main() {
	      for x in 1..5 {
	          println!("x: {x}");
	      }
	  }
	  ```
	- In questo caso ripeterà l'istruzione da `1` fino a `4` in quanto `5` non è incluso.
	- per includere anche il `5` dobbiamo utilizzare questa tipologia di espressione:
	- ```rust
	  fn main() {
	      for x in 1..=5 {
	          println!("x: {x}");
	      }
	  }```
- ### `loop`
	- Ripete all'infinito un espressione finché non troverà l'istruzione `break`
	- ```rust
	  fn main() {
	      let mut i = 0;
	      loop {
	          i += 1;
	          println!("{i}");
	          if i > 100 {
	              break;
	          }
	      }
	  }
	  ```
-
- ## Blocchi di istruzioni
- In rust è possibile immettere dei blocchi d'istruzione semplicemente immettendo il codice all'interno delle parentesi graffe e immettere il risultato all'interno di una variabile.
- ```rust
  fn main() {
      let z = 13;
      let x = {
          let y = 10;
          println!("y: {y}");
          z - y
      };
      println!("x: {x}");
  }
  ```
- In più ci sono la **variabili di scopo**, ovvero variabili che all'interno di questi blocchi di codice rimangono indipendenti e che possono variabile ed essere ridichiarate e sovrapposte.
- ```rust
  fn main() {
      let a = 10;
      println!("before: {a}");
      {
          let a = "hello";
          println!("inner scope: {a}");
  
          let a = true;
          println!("shadowed in inner scope: {a}");
      }
  
      println!("after: {a}");
  }
  ```
-
- ## Funzioni
- ```rust
  fn gcd(a: u32, b: u32) -> u32 {
      if b > 0 {
          gcd(b, a % b)
      } else {
          a
      }
  }
  
  fn main() {
      println!("gcd: {}", gcd(143, 52));
  }
  ```
- Nelle funzioni non abbiamo tassativamente bisogno del `return`
- In Rust è buona norma definire sempre i tipi delle variabile di cui abbiamo bisogno all'interno della funzione e di dichiarare anche il tipo restituito.
- Nel caso la funziona non devva restituire niente possiamo usare la dicitura `-> ()`, risulterebbe come semplice funzione di esecuzione.
-
