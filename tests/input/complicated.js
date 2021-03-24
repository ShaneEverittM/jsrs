let global = "Hey"

function is_true() {
    return global === "Hey"
}

function math() {
    if (is_true()) {
        return 1 + 5
    } else {
        return 2 + 5.5
    }
}

math()