A (for now) very minimal Rust runtime for embedded ARM targets.  Provides a minimal set of lang_items and Cortex-M exception vectors, all of which are weak symbols that can be overridden by other code as needed.

Eventually I'd like to flesh this out a bit more - adding unwinding, split-stacks, lightweight threads, etc.  For now, it's just the bare minimum needed to get off the ground.

TODO: explain what's provided, what the default implementations are, and how to set up a linker script to work with this.  For now, see https://github.com/mokus0/stm32.rs for an example.
