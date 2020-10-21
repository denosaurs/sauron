# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog],
and this project adheres to [Semantic Versioning].

## [0.0.2] - 2020-10-21

### Bug Fixes

- use deno.land/x url for mordor ([`eaad6d7`])
- use raw github url instead ([`c0cf0b0`])
- use deno.land/x version of sauron ([`c72d076`])

## [0.0.1] - 2020-10-20

### Features

- check api now works! ([`4410bfa`])
- switch over to wasm32-wasi to enable linter ([`b16ec7c`])
- sauron wasm cli, disable lint to fix errors ([`5062a58`])
- json serialization of diagnostics! ([`f70820c`])
- deserialize return value :D ([`a76a463`])
- initial push of sauron_wasm (HIGHLY WIP and ugly :P) ([`0b068f9`])
- sauron_fmt ([`7885ce4`])
- use check_file ([`b0034d6`])
- add me as co-author ([`ec1008a`])
- use recommended rules ([`c2889b9`])
- make linter fs independent ([`e39ce4e`])
- add check_path and change check_file ([`6c7a922`])
- add action workflow ([`185e015`])
- copypaste detector ([`e67fb2d`])
- add commandline interface ([`9385045`])
- add has_* rules for structure ([`f556c53`])
- :sparkles: Add DiagnosticLevel enum for rules ([`b507f61`])
- :beers: Add file structure checking along with no_index and snake_case rules ([`a84a548`])
- mimic dlint cli ([`0326e8c`])
- :sparkles: Generated a simple rust project ([`67dea9e`])

### Bug Fixes

- bump vercel deno version ([`0465ef1`])
- add no-lint header to fix linting ([`70878dc`])
- see if sauron deps work on main branch ([`138fe57`])
- replace rule name from _ to - ([`fb0377e`])
- coverpage and edit link ([`3b0626f`])
- scope name ([`f82274d`])
- check_file in sauron_duplicate and sauron_lint ([`76242ef`])
- use LintContext ([`306dc3a`])
- update rule impls to use check_path ([`a35280c`])
- remove old color call ([`cde8751`])
- supress warnings and format ([`74dbb00`])
- mimic linting behaviour of deno cli ([`019f9b9`])

[keep a changelog]: https://keepachangelog.com/en/1.0.0/
[semantic versioning]: https://semver.org/spec/v2.0.0.html
[0.0.2]: https://github.com/denosaurs/sauron/compare/0.0.1...0.0.2
[`eaad6d7`]: https://github.com/denosaurs/sauron/commit/eaad6d746a0cd3db1dc72ec3a3190ff32611def8
[`c0cf0b0`]: https://github.com/denosaurs/sauron/commit/c0cf0b06fdfd5d431752e3f9036f6537ca1e5aff
[`c72d076`]: https://github.com/denosaurs/sauron/commit/c72d076900b0a0f8b0997795807774a66874a178
[0.0.1]: https://github.com/denosaurs/sauron/compare/0.0.1
[`4410bfa`]: https://github.com/denosaurs/sauron/commit/4410bfa7772c0a0051939d31ed93fefe63ac2794
[`b16ec7c`]: https://github.com/denosaurs/sauron/commit/b16ec7c248fee6b3527bdb7d2c86d287c9cbbc2e
[`5062a58`]: https://github.com/denosaurs/sauron/commit/5062a58c92278de25c714bfb26be9c5e37398fa4
[`f70820c`]: https://github.com/denosaurs/sauron/commit/f70820cc8420ef2079baa0968e30f034d3e6b34d
[`a76a463`]: https://github.com/denosaurs/sauron/commit/a76a4631f4d7ea4ad3498e1c474f9eb141ba084f
[`0b068f9`]: https://github.com/denosaurs/sauron/commit/0b068f9dd8b66334cdd786888aa550489644e87a
[`7885ce4`]: https://github.com/denosaurs/sauron/commit/7885ce442d338e83907ec7615b75310b6cc77e14
[`b0034d6`]: https://github.com/denosaurs/sauron/commit/b0034d6ac753a574d036e8be4daae1f12fe54374
[`ec1008a`]: https://github.com/denosaurs/sauron/commit/ec1008a24d43bb2e1c7b2814f501c0e7c4a43a71
[`c2889b9`]: https://github.com/denosaurs/sauron/commit/c2889b9faaf81dba82742f7eb2125e8e17ac99ae
[`e39ce4e`]: https://github.com/denosaurs/sauron/commit/e39ce4e9395c0341a6e48d8cde42e91764b651af
[`6c7a922`]: https://github.com/denosaurs/sauron/commit/6c7a9221df634e04ae1098452d48ca3be5137f4a
[`185e015`]: https://github.com/denosaurs/sauron/commit/185e01527782adee6ec4c867955e082cca2226e2
[`e67fb2d`]: https://github.com/denosaurs/sauron/commit/e67fb2d338186b9bf8e8eabedd717e07196cd83b
[`9385045`]: https://github.com/denosaurs/sauron/commit/93850457be0afa785b07c68584c419d6551b9f12
[`f556c53`]: https://github.com/denosaurs/sauron/commit/f556c53b3e8266bd9799d20772d11fb5e401e009
[`b507f61`]: https://github.com/denosaurs/sauron/commit/b507f61f1dd9457380636c9a349927412883b4fd
[`a84a548`]: https://github.com/denosaurs/sauron/commit/a84a548ebec94bc3483f73aa07aa4ede4af4dd40
[`0326e8c`]: https://github.com/denosaurs/sauron/commit/0326e8cb61b49aa94f7408e3ed9616aa0a36cd90
[`67dea9e`]: https://github.com/denosaurs/sauron/commit/67dea9ef74e45831b21658ffc5dafc9a28be94f3
[`0465ef1`]: https://github.com/denosaurs/sauron/commit/0465ef179aab6982ca36e35e365dad397ae203e5
[`70878dc`]: https://github.com/denosaurs/sauron/commit/70878dccdb0a7cefd95735317256a73a28dc4856
[`138fe57`]: https://github.com/denosaurs/sauron/commit/138fe57ef7f5d6f1cac100fe71da9ada12512f52
[`fb0377e`]: https://github.com/denosaurs/sauron/commit/fb0377e7caae9e6e77e7e139baafda2e835cf008
[`3b0626f`]: https://github.com/denosaurs/sauron/commit/3b0626f3d24a0a8744e3e8df2e3176b2cb57ae0e
[`f82274d`]: https://github.com/denosaurs/sauron/commit/f82274dae8c7b61e816b423f90431704adaabe23
[`76242ef`]: https://github.com/denosaurs/sauron/commit/76242efaf3d67608de44dcb283bea0d51236ef60
[`306dc3a`]: https://github.com/denosaurs/sauron/commit/306dc3a4752c82caa1f9f652a64e356834cc11a1
[`a35280c`]: https://github.com/denosaurs/sauron/commit/a35280c9ca0e464fe8522ac6da901219436fd7d2
[`cde8751`]: https://github.com/denosaurs/sauron/commit/cde87513dc76db4083c18218833c83c213f01b8c
[`74dbb00`]: https://github.com/denosaurs/sauron/commit/74dbb002f027e82ce4cd29110cbdffc2faee01d3
[`019f9b9`]: https://github.com/denosaurs/sauron/commit/019f9b9ad25bcf74a727d1febd8134ce03d1934e
