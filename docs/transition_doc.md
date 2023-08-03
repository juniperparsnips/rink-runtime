# Transition from fork to first MWE

## Goal

The goal is to use the [original repo](https://github.com/fveilly/rink-runtime)
and make a usable rust runtime for ink.

This document describes steps taken in order to transition to a minimum working
example (MWE).

## Dead Code

Most of the code in the repo is unused and not covered by tests. Rather than
deleting them, they have been moved to the folder `dead_code`.

They are useful for future reference and excerpts might be useful in the future
when reimplementing the features.
