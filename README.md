# lettuce

Proof of concept deep dive into erlang term storage internals.
Compares an `ART` index, a `CA` Tree and `WAVL` Tree under workloads similar to those benchmarked on `:ets`
for it's ordered apis.

## References/Notes
- https://dspace.cuni.cz/bitstream/handle/20.500.11956/127952/130308737.pdf?sequence=1&isAllowed=y
- https://ics.uci.edu/~goodrich/teach/cs165/notes/WeakAVLTrees.pdf
- https://github.com/pvachon/wavl_tree
- https://sites.google.com/view/bst-comparison/home
- https://oar.princeton.edu/handle/88435/pr1nz5z
- https://sidsen.azurewebsites.net//papers/rb-trees-talg.pdf

## Installation
Requires Linux or a virtualization like [orbstack on macOS](https://docs.orbstack.dev/machines/).
