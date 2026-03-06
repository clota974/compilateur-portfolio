# Projet en cours

J'avais avancé sur le projet, sans pouvoir aboutir à un commit qui fonctionne. 
Il se trouve sur la branche WIP.
Sur la branche WIP, le parsing d'une expresison telle que `let x = 3+5;` fonctionne.
Je suis en train d'implémenter l'évaluation d'une variable dans son environnement.

# Exemple compilateur

Sur ce dépôt, je construis pas à pas mon compilateur en reprenant les fondements de la théorie de compilation.
Je n'utilise volontairement pas le Pratt Parsing afin de comprendre en profondeur les méthodes essentielles.

Sur cette version, le compilateur décrit les étapes du parsing.

## Comment réaliser une démo du projet

Sur cette version, el compilateur n'accepte que le langage de Peano de base (+, -, *, /)
Entrez votre expression dans le fichier input.txt .
Dans le terminal, entrez 
```bash
cargo run -- input.txt
```

## Exemple
Input : `36 * (4 + 5) * 2.4 + 6`

Output : `(+ 6 (* (* 36 [ (+ 4 5) ]) 2.4))`

On remarque que 4+5 est réalisé en premier, suivis des produits et enfin de l'addition.
L'output est formatté au style Lisp, avec les crochets qui représentent les parenthèses explicites.