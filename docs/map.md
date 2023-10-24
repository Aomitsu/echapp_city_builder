# Maps !
L'objectif est d'avoir une map qui s'agrandis de temps en temps lorsque le joueur accomplis des étapes, comme par exemple atteindre les 10 habitants.

> Point important à penser impérativement: La caméra ne peux pas tourner, il faudrait rendre moins visible les objets de premier plan lorsqu'on va vers les objets de dernier plan.

Le selecteur de tiles est un peu celui qui décide quel tile afficher, à savoir que si il est hors map, on affiche l'intégralité de la map !

Dans le mode édition d'assignation, aucun bâtiment n'est affiché.

## Features à implémenter
- [] Rendu de la Map
- [] Agrandissement de la Map sur demande
- [] Rendu du sélécteur
- [] Différents environnement de Map
- [] Editer les cases ( leurs assignations )
- [] Faire en sorte que des bâtiments se construisent sur les cases assignés au hazard
- [] Ne pas afficher le premier plan si dernier plan selectionné

## Fonctionnement global
A la manière du random tick speed sur minecraft, si les conditions nécessaires sont réunis, aux tiles assignés d'une fonction spawnera des bâtiments avec des stats aléatoires mais aux limites fixés, les bâtiments pourront évolués si d'autres conditions sont réunies, changeant de couleurs, adoptant ceux des voisins pour former de véritables quartiers.

l'objectif pour le joueur est d'avoir une ville équilibrée, mais aussi d'agrandir son terrain avec les **Milestones**, les étapes qui lui seront toalement invisibles.

## Modes d'édition
### Mode Normal
C'est le mode général, on vois la map et on peux intéragir avec les bâtiments ( voir les stats... )

### Mode Assignage
Ce mode cache tout les bâtiments, mais permets d'assigner, sous forme de tiles de couleur en surbrillance au-dessus des tiles de base, les différents secteur d'activités

- Secteur **Habitations**
- Secteur **Commerces**
- Secteur **Industries**

### Mode Construction
Permets la construction de routes, parcs, et autres infrastructures...