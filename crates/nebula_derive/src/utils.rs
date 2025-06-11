pub fn field_name_to_key(field_name: &str) -> Result<String, syn::Error> {
    let converted = to_snake_case(field_name);
    Ok(converted)
}

fn to_snake_case(input: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = input.chars().collect();

    for (i, &ch) in chars.iter().enumerate() {
        if ch.is_uppercase() {
            let should_add_underscore = if i == 0 {
                false
            } else {
                let prev_char = chars[i - 1];
                let next_char = chars.get(i + 1);

                prev_char.is_lowercase()
                    || (prev_char.is_uppercase() && next_char.map_or(false, |c| c.is_lowercase()))
            };

            if should_add_underscore {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }

    result
}
