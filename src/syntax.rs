fn default_expressions_syntax() -> Vec<(String, String)> {
    let mut syn_map = Vec::new();
    let mut register = |tag: &str, evolution: &str| {
        syn_map.push((tag.to_string(), evolution.to_string()))
    };
    register(".", "get($1,$2)");
    register("?:", "get_or_default($1,$2)");
    register("?.", "get_or_none($1,$2)");

    register("*", "multi($1,$2)");
    register("/", "div($1,$2)");
    register("%", "mod($1,$2)");

    register("+", "add($1,$2)");
    register("-", "sub($1,$2)");

    register(" is ", "eq(type($1),$2)");
    register("==", "eq($1,$2)");
    register("!=", "not_eq($1,$2)");
    register(">=", "ge($1,$2)");
    register("<=", "le($1,$2)");
    register(">", "r_angle($1,$2)");
    register("<", "l_angle($1,$2)");

    register("&&", "and($1,$2)");
    register("||", "or($1,$2)");
    syn_map
}
