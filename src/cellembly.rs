enum DNABase {
    A,
    G,
    C,
    T,
}

struct Signal {
    ident: [DNABase; 16],
}

//all operations add 1 to the signal of the output

//all proteins have input/output identifiers for each input/output value

enum Protein {
    //Synthesies a protein from a DNA strand with identifier i.
    //input: identifier of sequence to synthesise
    Synthesiser {
        i: Signal,
    },
    //Concatenate strands into single strand
    //input: strand with vector of strand identifiers (strands to concatenate)
    //output strand with concatenated strands
    Concat {
        i: Signal,
        o: Signal,
    },
    //Split a DNA strand into multiple strands, at all places in strand that matches
    //another strand sequence. Signals are extracted from the beginning of each returned
    //sequence.
    Split {
        haystack: Signal,
        needle: Signal,
    },
    //Performs unsigned little-endian addition of all integers in two DNA strands
    Add {
        a: Signal,
        b: Signal,
        out: Signal,
    },
    Compare {
        a: Signal,
        b: Signal,
        less: Signal,
        equal: Signal,
        more: Signal,
    },
    //creates strand o with contents of i and identifier = body of redirect
    Redirect {
        i: Signal,
        redirect: Signal,
    },
    //Creates a potentially infinite copies of a strand
    Inf {
        i: Signal,
    },
}


START

:read_line
line = []
READ_CHAR -> c
c, '\n' -> COMPARE -> keep_going, reset, keep_going


/*

---BASIC THINGS---
A = 98453490
B = 9345342
C = A + B
C = A - B -> representable with complements

---IF---
if C == 5 {
    T = 49598
}
could probably be represented by a
*/
