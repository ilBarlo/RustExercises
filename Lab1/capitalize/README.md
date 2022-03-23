# Instructions
Creare con cargo un nuovo package chiamato capitalize e, al suo interno, definire la funzione

      fn capitalize(s: &str) -> String {}
      
La funzione deve convertire in maiuscolo il primo carattere di ogni parola che compone il testo s, ignorando eventuali altri caratteri maiuscoli al suo interno.
Le parole sono separate da uno o più spazi.
Nel main leggere una sequenza di parole come argomento da command line, invocare la funzione e stampare il risultato.
Esempio:

      cargo run -- “questa è una   frase”
      
E’ possibile ipotizzare di trasformare in maiuscolo i caratteri in place, senza copiare la stringa? Motivare la risposta.
______
# Suggerimento: 
in alcune lingue non c’è una corrispondenza 1:1 come numero di codepoint tra maiuscole e minuscole. Ad esempio in tedesco la maiuscola di ß può essere sia SS che ẞ.

Scrivere anche i test case necessari per verificare il corretto comportamento. Coprire almeno i seguenti casi:

- stringa con più di una parola
- stringa con una sola parola senza spazi
- stringa con caratteri accentati all’inizio di parola stringa vuota
- stringa con più spazi

Per il libreria clap parsing degli argomenti di command line si suggerisce di inserire nel progetto la
https://docs.rs/clap/latest/clap/
