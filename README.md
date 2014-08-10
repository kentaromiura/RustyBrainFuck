RustyBrainFuck
==============

A BrainFuck interpreter written in rust

This project is my first experiment with the rust programming language

Why?
====
I wanted to do a simple task while trying to experiment with Rust,
so I decided to write a simple BrainFuck interpreter in this language.

Why BrainFuck?
==============
I already wrote BF interpreters in the past, and it's a relative simple task,
I've done in [C](https://planet-source-code.com/vb/scripts/ShowCode.asp?txtCodeId=6451&lngWId=3)
and in [JavaScript](http://freeforumzone.leonardo.it/d/2926042/kentaromiura-BF-interpreter/discussione.aspx)
both example above where not following the spec 100%, I remember I fixed the point made by Daniel Cristofani in the comment section, [linking to this](http://www.hevanet.com/cristofd/brainfuck/epistle.html)
in my C implementation but I cannot find that version online, also there should be a Perl implementation and maybe a D one but probably I never published them.

Anyway this implementation must be 100% compatible with [ENSI specification](http://esoteric.sange.fi/ENSI/brainfuck-1.3.txt) and possibly with the [portable brainfuck spec](http://www.muppetlabs.com/~breadbox/bf/standards.html)

Ideally there should be some tests written to ensure this happens.

Build and test
==============
You can build the main by executing
`make all`
You can just run the tests by executing
`make test`
You can build and execute the hello world source with the
`make hello`
command.
