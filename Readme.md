#   Rust Logic of Action (RLA)
Leslie Lamport developed a logic he called Temporal Logic of Actions to describe behaviours of concurrent systems.
TLA+ is a formal specification language to design, model, document, and verify programs, especially concurrent systems and distributed systems.

##  The Idea
I suggest an rusty implementation and syntax to formally describe the programme and automatically check

0.  if this specification is flawed and
1.  if the code of the programme implements this specification.

### For-All and Exist

There are predefined generic functions `forAll` and `exists`.
