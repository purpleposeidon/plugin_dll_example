#!/usr/bin/python3

f = open("src/shit_ton.rs", 'w')

for i in range(0, 2**16 + 100):
    f.write("#[no_mangle] pub fn f{}() {{}}\n".format(i))
