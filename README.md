> [!NOTE]
> This is a very rough draft of the end product. Issues and bugs are expected.

# disorder
Small Tera-based template scaffolding tool. 

## Usage
Install via 
```
cargo install --git https://github.com/heveleen/disorder --locked
```

---

Deploy a template:
- Pass down the template directory via the `input` flag. If missing, defaults to the running dir.
- Pass down the manifest file name with the `manifest` flag. If missing, defaults to `manifest.toml`.
- Pass down the output directory with the `output` flag. If missing, defaults to `./out`.

--- 

Create a new template automatically:
- WIP

## Template format
This is only useful if you either want to create templates manually or want to inspect existing ones.

```toml
# The display name of the template/project
title = "My Awesome Web App"
# A short summary of what this template generates
description = "A full-stack web application with authentication, database, and API routes."
# Semantic version of this template. Displayed alongside the title and description
version = "1.0.0"

# Each [[elements]] entry defines a single prompt shown to the user during scaffolding.
#
# Fields:
# - id - key used to reference element in code
# - title - short label shown in the prompt
# - description - helper text displayed below the title
# - default - (optional) pre-filled value;

# String element
[[elements]]
id = "project_name"
title = "Project Name"
description = "The name used for the package and directory."
default = "my-app"

# String element with no default
[[elements]]
id = "author"
title = "Author"
description = "Full name of the project author."
# User must type something in.

# Boolean element via a yes/no prompt
[[elements]]
id = "use_typescript"
title = "Use TypeScript?"
description = "Enable TypeScript instead of plain JavaScript."
default = true

# Number element
[[elements]]
id = "server_port"
title = "Server Port"
description = "The port the development server will listen on."
default = 3000
```

## Licensing
This project is licensed under either **MIT** or **Apache-2.0** Licenses, at your choice.

See [MIT](LICENSE-MIT) and [Apache-2.0](LICENSE-APACHE) license files for details.

Each release tarball contains a `THIRD_PARTY_LICENSES.html` file containing notices and acknowledgements of third party crates used in this project.
