# Small Queue

A FIFO queue which stores its data in-place when containing 0 or 1 elements, but
expands to a dynamically sized heap allocation when more elements are inserted, and
can free its heap allocation if the size returns later to 1 or 0.