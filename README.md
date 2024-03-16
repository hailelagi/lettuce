# lettuce

A toy ~no-std~, mmapped persisted key value store backed by a weak version of a self balancing binary search tree based on the Adelson-Velsky and Landis (AVL Tree) and a linear hash map.
note: persisted *does not* mean primarily on-disk but implies immutability.

## Further
- https://dspace.cuni.cz/bitstream/handle/20.500.11956/127952/130308737.pdf?sequence=1&isAllowed=y
- https://ics.uci.edu/~goodrich/teach/cs165/notes/WeakAVLTrees.pdf
- https://github.com/pvachon/wavl_tree
- https://sites.google.com/view/bst-comparison/home
- https://oar.princeton.edu/handle/88435/pr1nz5z
- https://sidsen.azurewebsites.net//papers/rb-trees-talg.pdf

## Installation
Requires Linux or a virtualization like [orbstack on macOS](https://docs.orbstack.dev/machines/).
