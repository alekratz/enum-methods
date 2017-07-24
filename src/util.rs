pub(crate) fn to_snake_case<S: AsRef<str>>(ident: &S) -> String {
    let mut snake_case = String::new();

    for (i, c) in ident.as_ref().chars().enumerate() {
        if i > 0 && c.is_uppercase() {
            snake_case.push('_');
        }

        snake_case.push(c.to_lowercase().next().unwrap());
    }

    snake_case
}
