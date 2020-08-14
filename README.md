#  WIP: git-lint

![image](https://github.com/alexanderjeurissen/git-lint/blob/main/public/assets/screenshots/linter%20copy.png?raw=true)

Lint staged files using designated linters per file extension, written in Rust for optimal performance

Heavily inspired by:

- [lint-emit](https://github.com/ragone/lint-emit)
- [yarn lint-staged](https://github.com/okonet/lint-staged)


### Motivation

linting prior to engineers pushing changes to CI is a meaninful way to reduce wait time and number of pipeline rebuilds on CI due to syntax errors.

There are several libraries that tie linting to a designated git hook, some of which have been listed above.

This project came about mostly due to disastisfaction with the overall feature set and speed of existing solutions.

1. Most libraries only support a very specific use-case (e.g. linting staged files, linting changed files, linting files in a commit range etc.) I want to simply specify a git command in the config and lint the diff of that.
2. Often these libraries include features that dont work for my workflow, these features often add overhead which results in reduced execution speed

One library that I've used for a while is `yarn lint-staged` it's very mature but has a couple of design decisions which dont really work for me:

#### Node dependency

`yarn lint-staged` is great, but it requires running Node which in itself adds significant overhead to the actual linting.

Removing the need to run `node` is the premise around using `NetCat` for libraries such as :

- [rubocop-daemon](https://github.com/fohte/rubocop-daemon#more-speed)
- [eslint-d](https://github.com/mantoni/eslint_d.js#moar-speed)

To give you a sense on node.js startup time, here is a benchmark comparison for a simple hello world function, recorded using [hyperfine](https://github.com/sharkdp/hyperfine):

![image](https://github.com/alexanderjeurissen/lint-staged/blob/main/public/assets/screenshots/hello_world_benchmark.png?raw=true)

#### Merge-commits

`yarn lint-staged` lints on merge-commits. This is undesirable because it will lint changes that are not part of *our* changeset. *This increases the likelihood of old lint errors triggering and blocking `git merge --continue`*

In addition in most cases we are unnecessarily linting files multiple times, if each commit is linted, then the likelihood of a merge between two branches resulting in lint errors is relatively small. In those cases we can rely on CI to catch those instances. pre-commit linting should be a best effort, not the sole linting strategy one employs.

#### Backup strategy

`yarn lint-staged` has a very extensive backup strategy. which revolves around:

1. Creating a snapshot for backup purposes (without deleting things from the working tree)
2. Stashing of files that have partially unstaged changes, and applying this stash after adding lint fixers to staging area.
3. Reverts to pre-lint snapshot on exceptions
4. handling of merge meta data and deleted files

Most of this is great, but there are a couple of changes that I'd like to see:

1. Merge head handling is not needed given that I dont think linting on merge commits is desirable.
2. Instead of staging the lint changes and ammending it, I'd like to see a separate commit including the lint fixes.

What I deem essential is:

1. A backup stash tagged with timestamp and branch, so that one can manually recover if things go bad (which should not really be the case as most linters are very stable at this point)
2. Stash all unstaged changes
3. Do the linting and stage + commit the changes
4. unstash unstaged changes


### Benchmarks (WIP)

tested using [hyperfine](https://github.com/sharkdp/hyperfine)

![image](https://github.com/alexanderjeurissen/lint-staged/blob/main/public/assets/screenshots/benchmark.png)

_**This means a increased execution speed by approx 60%**_

NOTE that these benchmarks are nowhere near complete, as this library has no feature parity.

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

**git-lint.config.toml**

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



