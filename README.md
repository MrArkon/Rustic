<h1 align="center">
  <img width="200" alt="Rustic" src="https://raw.githubusercontent.com/MrArkon/Rustic/master/assets/Rustic.png" />
  <br>
  Rustic
  <br>
</h1>

<h4 align="center">
  Rustic is a multi-purpose Discord bot written in <a href="http://rust-lang.org" target="_blank">Rust</a> with <a href="http://github.com/serenity-rs/serenity" target="_blank">Serenity</a>.
  <br>
</h4>

<p align="center">
  <a href="https://github.com/MrArkon/Rustic/actions/workflows/workflow.yml">
    <img alt="Workflow" 
         src="https://img.shields.io/github/workflow/status/MrArkon/Rustic/Continuous%20integration?logo=github&style=for-the-badge">
  </a>
  <a href="https://github.com/MrArkon/Rustic/blob/master/LICENSE">
    <img alt="License" 
         src="https://img.shields.io/github/license/MrArkon/Rustic?style=for-the-badge">
  </a>
  <a href="https://github.com/MrArkon/Rustic/issues">
    <img alt="Issues" 
         src="https://img.shields.io/github/issues/MrArkon/Rustic?label=ISSUES&logo=github&style=for-the-badge">
  </a>
  <a href="https://www.codacy.com/gh/MrArkon/Rustic/dashboard?utm_source=github.com&amp;utm_medium=referral&amp;utm_content=MrArkon/Rustic&amp;utm_campaign=Badge_Grade">
    <img alt="Code Quality" 
         src="https://img.shields.io/codacy/grade/0ee26216e06b4f5a98c5240a1ddd87f1?logo=codacy&style=for-the-badge">
  </a>
</p>

# Note: Repository Archived
You might have noticed the activity on this repository has been very low since last year which is the reason why I decided to archive this project. I might come back to this project in the future but as of now I am focusing on my other two bots: [JukeBox](https://github.com/MrArkon/JukeBox) and [Winston](https://github.com/MrArkon/Winston)

## 🚀 Setup & Configuration
> Note: This guide assumes you have [Rust](https://rust-lang.org), [Cargo](https://github.com/rust-lang/cargo) & [PostGreSQL](https://www.postgresql.org/) installed. You will not get any support for self-hosting.
1. Rename `config.toml.example` to `config.toml`
2. Enter the appropriate details in the config file
3. Add your database url to your environment variables
4. Create the database using the sqlx CLI
```shell
$ sqlx database create
```
5. Apply the migrations using the sqlx CLI
```shell
$ sqlx migrate run
```
6. You should be able to run the bot with `cargo run --release`

## 📃 License
Rustic is licensed under the AGPL 3.0 license. See the file [`LICENSE`](https://github.com/MrArkon/Rustic/blob/master/LICENSE) for more information.

----

<h4 align="center">Show some :heart: by starring :star: this repository!</h4>
