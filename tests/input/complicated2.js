let global = "Hey"

function is_true() {
    return global === "Hey"
}

function math() {
    if (is_true()) {
        let y = 5;
        for (let x = 1; x < 4; x++) {
            y = y * 2;
        }
        return y + 5 //45
    } else {
        return 50
    }
}

function runner() {
    for (let i = 0; i < 10; ++i) {
        for (let j = 0; j < 10; ++j) {
            if (i === 5) {
                if (j === 5) {
                    return math() + 5
                }
            }
        }
    }
    return 123
}

runner() // 50
