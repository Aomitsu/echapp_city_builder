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

## Technique
### Map Lifecycle

```Rust
pub struct CityMap {
    pub blocks: HashMap<IVec2, FloorType>,
    pub selector: Option<IVec2>,
    /// map_size x map_size, doesn't count decorative blocks under the map
    pub map_size: i32,

}

pub struct FloorType {
    pub assignation: CityAssignation,

    // TODO: + Some things related to texturing and animations
}

pub enum CityAssignation {
    Residential,
    Commercial,
    Industrial,
    /// Can't be assigned ( Parks, Roads, Special infrastructures, )
    CantBeAssigned,
    #[default]
    Nothing,
}
```
```Rust
impl CityMap {
    /// Create a new simple map FILE, 3x3 with a Starter Road (TODO: Special infrastructures)
    fn new() -> Self {} 

    //TODO: Load

    /// Create map FILE in the WORLD ( New map, or Saved map )
    fn summon() -> Self {}

    /// Kill the map from the WORLD
    fn kill() {}

    /// Refresh the WORLD display
    /// 
    /// EX: IF Selector is OUTSIDE the map, display all
    /// 
    /// EX: IF Selector is IN THE BACK the map, display the back correctly but not the FRONT
    fn refresh() -> Self {}
}
```
Donc, une `CityMap` va vivre de cette façon :
Elle sera créée `CityMap::new()` ou load `CityMap::load()`, affichée avec `CityMap::summon()`, raffraîchie avec `CityMap::refresh()` puis kill avec `CityMap::kill()` si on quitte la map, qu'on en load une autre, etc...

TODO: Tout ce qui se fera sur la map ( a savoir les infra et habitations ) sont à penser juste après.