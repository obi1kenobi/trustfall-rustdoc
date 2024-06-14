use std::{collections::BTreeMap, env, path::Path};

use handlebars::Handlebars;

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
    let mut args = BTreeMap::new();
    args.insert("version_numbers", versions.as_slice());

    let template_and_output = [
        ("../../template/parser.template.rs", "../../src/parser.rs"),
        ("../../template/query.template.rs", "../../src/query.rs"),
        (
            "../../template/versioned.template.rs",
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
