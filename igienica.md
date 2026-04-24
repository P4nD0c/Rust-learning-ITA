- E' un paradigma di #[[Learning Rust]] che deriva dalle **macro_rules**. Serve per evitare la sovrapposizione di variabili omonime.
-
- Questa metodologia è stata influenzata affinché non vengano inquinate delle variabili nel codice ed evitare sovrapposizioni di nomi.
- ```rust
  macro_rules! crea_variabile {
      () => {
          let x = 42;
          println!("Valore interno alla macro: {}", x);
      };
  }
  
  fn main() {
      let x = 10;
      crea_variabile!();
      
      // Questo 'x' si riferisce al 10 definito sopra, 
      // NON al 42 creato dentro la macro.
      println!("Valore nel main: {}", x); 
  }
  ```
- Dall'esempio è visibile che avremo come "`return`" della funzione `crea_variabile` il valore di x che è stata dichiarata 42 `let x = 42`
- In quanto stiamo usando una "macro" tutto ciò che avviene in una determinata funzione, rimarrà per tutta l'esecuzione del programma (almeno che non venga richiesto) all'interno della funzione senza compromettere eventuali dichiarazioni di variabile omonime.
-