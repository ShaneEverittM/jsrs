let x = {
    name: "Shane",
    nested: {height: "5:11"},
    age_in_n_years: function (num) {
        console.log("In age")
        return 24 + num;
    }
}

x.age_in_n_years(10)