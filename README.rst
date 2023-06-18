
Magic Switcheroo
================

|Build Status| |Latest Version| |crates.io| |License| |Commit Activity|
|Rust 1.70|

.. |Build Status| image:: https://github.com/gabrielfalcao/magic-switcheroo/actions/workflows/rust.yml/badge.svg
.. |Latest Version| image:: https://img.shields.io/crates/v/magic-switcheroo.svg
.. |crates.io| image:: https://img.shields.io/crates/v/magic-switcheroo.svg
.. |License| image:: https://img.shields.io/crates/l/magic-switcheroo
.. |Commit Activity| image:: https://img.shields.io/github/last-commit/gabrielfalcao/magic-switcheroo
.. |Rust 1.70| image:: https://img.shields.io/badge/Rust%20Version-1.70-red

**Rust Crate and command-line companion to switch the magic numbers of a
file with your own magic numbers based on a string of a string containing a "magic" word**

Don't use it in your CTF's, it would be a very bad idea to turn regular files into ".dat" file... 😇 and vice-versa, OF COURSE.
--------------

Installation
------------

.. code:: bash

   cargo install magic-switcheroo

Usage
-----


As a Library
............

Most of the magic is done within the struct ``MetaMagic`` which works
with a ``Vec<u8>`` most of the time.


Enchanting a Vec<u8>
~~~~~~~~~~~~~~~~~~~~


As command-line utility
.......................

The crate ships with the binary ``ms`` which provides subcommands to enchant, restore and dump ``MetaMagic`` structs in JSON, among other things.


Enchanting a file
~~~~~~~~~~~~~~~~~


   number: a valid integer suffix: smh

.. code:: bash

   ms switch <THEMAGICWORD> <FILE>


Examples
^^^^^^^^

   Wait for 5 seconds

   .. code:: bash

      magic-switcheroo time 5s

..

   Wait for 4 minutes

   .. code:: bash

      magic-switcheroo time 4m
