- add intermedite between config and task generator for node conversion
- move task generation and validity check out of config and into this new struct
- update launcher and sub commands to use this new struct

## Idea
We can rename Launcher to something like TaskManager, then create a new Launcher struct,
which will act as the intermediate struct mentioned above.  The new launcher will contain
vectors of nodes, in addition to relevant metadata.  It can be used in place of Config in
sub commands and implement things like validity checks and task generation/execution (
where we would move current Config impls into this new struct).  Not only will this simplify
extensions like sub commands, but it simplify the entry point of our api, so as to make custom
clients easier to design.
