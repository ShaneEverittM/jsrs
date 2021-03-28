function recurse(x) {
    if (x === 1) {
        return recurse(2) + 2
    } else {
        return 5
    }
}

recurse(1) //7