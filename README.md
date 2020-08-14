#  WIP: lint-staged

Lint staged files using designated linters per file extension, written in Rust for optimal performance

Heavily inspired by:

- [lint-emit](https://github.com/ragone/lint-emit)
- [yarn lint-staged](https://github.com/okonet/lint-staged)


### Motivation

Pre-commit linting saves engineers valuable time as it reduces pipeline rebuilds on CI due to syntax errors.

`yarn lint-staged` is great, but it requires running Node which in itself adds significant overhead to the actual linting.

Removing the need to run `node` is the premise around using `NetCat` for libraries such as :

- [rubocop-daemon](https://github.com/fohte/rubocop-daemon#more-speed)
- [eslint-d](https://github.com/mantoni/eslint_d.js#moar-speed)

This library aims for feature parity with `lint-staged` whilst significantly improving performance by:

1. No `node-js` startup time, saving ~300ms
2. Written in Rust and therfore Compiled vs interpreted.
3. Multi-threaded execution with fast fail.

### Benchmarks

tested using [hyperfine](https://github.com/sharkdp/hyperfine)

![image](https://github.com/alexanderjeurissen/lint-staged/blob/main/public/assets/screenshots/benchmark.png)

_**This means a increased execution speed by approx 60%**_


**lint-staged.config.js**

```js
module.exports = {
  "*.{js,html, html.erb}": filenames =>
    filenames.map(filename => `bin/eslint-daemon-wrapper --fix ${filename}`),
  "*.css": filenames => filenames.map(filename => `stylelint --fix ${filename}`),
  "*.scss": filenames => filenames.map(filename => `stylelint --fix --syntax=scss ${filename}`),
  "*.{rb, erb}": filenames =>
    filenames.map(
      filename => `bin/rubocop-daemon-wrapper ${filename} --auto-correct`
    )
};
```

**lint-staged.config.toml**

```toml 
[[linters]]
name = 'eslint'
cmd = 'bin/eslint-daemon-wrapper'
args = ['--fix', '{file}']
ext = ['js', 'html', 'html.erb']

[[linters]]
name = 'stylelint'
cmd = 'stylelint'
args = ['--fix', '{file}']
ext = ['css']

[[linters]]
name = 'stylelint'
cmd = 'stylelint'
args = ['--fix', '--syntax=scss', '{file}']
ext = ['scss']

[[linters]]
name = 'rubocop'
cmd = 'bin/rubocop-daemon-wrapper'
args = ['{file}', '--auto-correct']
ext = ['rb', 'erb']
```



