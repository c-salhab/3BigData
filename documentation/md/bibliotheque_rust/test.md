# Test du bon fonctionnement des différents modèles

Dans cette section, on retrouve un ensemble de tests pour les différents modèles afin de s'assurer de leur bonne implémentation.

**Voici la liste des tests disponibles:**

1. Tests pour MLP (Multi-Layer Perceptron):
   - mlp_classification_linear_simple
   - mlp_classification_linear_multiple
   - mlp_classification_multi_linear_3_classes
   - mlp_classification_cross
   - mlp_classification_multi_cross
   - mlp_classification_xor
   - mlp_regression_linear_simple_2D
   - mlp_regression_linear_simple_3D
   - mlp_regression_linear_tricky_3D
   - mlp_regression_non_linear_simple_2D
   - mlp_regression_non_linear_simple_3D


2. Tests pour RBF (Radial Basis Function):
   - rbf_classification_linear_simple
   - rbf_classification_linear_multiple
   - rbf_classification_multi_linear_3_classes
   - rbf_classification_cross
   - rbf_classification_multi_cross
   - rbf_classification_xor
   - rbf_regression_linear_simple_2D
   - rbf_regression_linear_simple_3D
   - rbf_regression_linear_tricky_3D
   - rbf_regression_non_linear_simple_2D
   - rbf_regression_non_linear_simple_3D

La nomenclature des tests suit le format suivant : `modele_classification_ou_regression_nom_du_test`

---
Il existe un notebook pour chaque modèle (Linéaire, MLP, RBF, SVM) contenant tous les cas de test correspondants. 
Ces notebooks peuvent être trouvés dans le dossier interpretability/ du projet.
---
Voici un exemple de comment exécuter les tests :
```bash
# commande a executer dans le dossier mylib
cargo run --example ml_classification_linear_simple
```