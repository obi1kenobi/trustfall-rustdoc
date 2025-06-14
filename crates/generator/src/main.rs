use std::{collections::BTreeMap, env, path::Path};

use handlebars::{handlebars_helper, Handlebars};
use serde_json::Value;

// a helper to return only `<= cutoff` elements from the list of numbers
handlebars_helper!(map_lte: |args: Value, cutoff: i64| {
    if let Value::Array(arr) = args {
        Value::Array(arr
            .iter()
            .map(|x| {
                x.as_i64().expect("non-number included")
            })
            .filter(|num| *num <= cutoff)
            .map(Into::into)
            .collect::<Vec<_>>()
        )
    } else {
        unreachable!("non-array value provided: {args:?}")
    }
});

// a helper to return only `> cutoff` elements from the list of numbers
handlebars_helper!(map_ge: |args: Value, cutoff: i64| {
    if let Value::Array(arr) = args {
        Value::Array(arr
            .iter()
            .map(|x| {
                x.as_i64().expect("non-number included")
            })
            .filter(|num| *num > cutoff)
            .map(Into::into)
            .collect::<Vec<_>>()
        )
    } else {
        unreachable!("non-array value provided: {args:?}")
    }
});

fn main() {
    let versions: Vec<i64> = env::args()
        .skip(1) // skip the binary's own name, only keep explicit args
        .map(|value| {
            value
                .parse()
                .unwrap_or_else(|e| panic!("invalid version \"{value}\": {e}"))
        })
        .collect();
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_helper("map_lte", Box::new(map_lte));
    handlebars.register_helper("map_ge", Box::new(map_ge));

    let mut args = BTreeMap::new();
    args.insert("version_numbers", versions.as_slice());

    let template_and_output = [
        ("../../template/parser.rs.template", "../../src/parser.rs"),
        ("../../template/query.rs.template", "../../src/query.rs"),
        (
            "../../template/versioned.rs.template",
            "../../src/versioned.rs",
        ),
    ];
    for (template_file, target) in template_and_output {
        let output = materialize(&mut handlebars, Path::new(template_file), &args);
        fs_err::write(Path::new(target), output).expect("failed to write file");
    }
}

fn materialize(
    handlebars: &mut Handlebars,
    template_file: &Path,
    args: &BTreeMap<&str, &[i64]>,
) -> String {
    let template = fs_err::read_to_string(template_file).expect("failed to read file");
    handlebars
        .render_template(&template, args)
        .expect("failed to render template")
}
