function params(x, y) {
    return x + y;
}

let x = {
    name: "Shane",
    age_in_n_years: function (num) {
        return 24 + num;
    }
}

params(1 + 2, x.age_in_n_years(10)) // 37
