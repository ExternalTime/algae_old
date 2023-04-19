# Algae - keyboard layout generator
Another keyboard layout generator. For now it only works as a
library. It has following features:

1. Custom keyboard shapes
2. Efficient\* layout generation for metrics defined at runtime
3. Using multiple corpora during generation (not concatenation)

I'm also pretty sure current architecture should allow optimizing
layouts with layers if the keys used to access them are pinned.

For now "cli" only serves as an example on how to use the library
and has more things hard-coded than not.

\* For now generation uses only hill climbing. This will be
imporoved soon.
