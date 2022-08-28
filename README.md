# cargo-gitv

Reasonable and opinionated version management based on Cargo and Git metadata.

## How it Works

`cargo-gitv` produces semantic versioning compatible version strings based on the version defined in your `Cargo.toml` and the git metadata. It supports producing release and pre-release versions. Furthermore it also supports plain timestamp-based versions useful for continuous delivery pipelines. Furthermore it provides commands to bump your version in `Cargo.toml`.

The base semantic version is always taken from `Cargo.toml`. For a release version `cargo-gitv` uses `<commit timestamp>-<git sha>` as build metadata. This generates versions such as `1.1.0+20220715144403-90bc61e`. For pre-release versions we currently support development versions and release candidates. The commit timestamp is used together with the pre-release type string as the identifieres. This leads to version strings as `1.1.1-dev.20220715144403+90bc61e` for development versions and `1.1.1-rc.20220715144403+90bc61e` for release candidates.

Please note that we use the commit timestamp and not the current timestamp. Due to this, we are able to produce stable version numbers for each commit, since all information used to assemble the version number is contained in the commit.

Finally, `cargo-gitv` verifies that the version in `Cargo.toml` is greater than the latest release (development and release candidates) or exactly matches the current Git version tag (releases). The rationale is that on a release the version in `Cargo.toml` should match the git tag that is used to release. Furthermore, `cargo-gitv` is based on the idea that the version gets bumped after a release to the next version (maybe even several times). In semantic versioning, a pre-release version is always said to precede the release version, i.e., `1.1.1-rc.20220715144403+90bc61e < 1.1.1+20220715144403+90bc61e`. In our opinion it makes sense to have pre-release versions share the version number that is later to be released. Please note that there might be several base versions before the next release. One might start with `1.1.2` right after the release, bump to `1.2.0` when new features are introduced and end up with `2.0.0` when an API change had to be introduced.

## Usage

* Computes a semantic-versioning compatible workspace version following a set of rules.
* Defaults to computing pre-release versions. 
* Default pre-release identifier is `dev`.
* Default base version is the Cargo version. 
* A git version tag is a `v` followed by a normal semantic version, i.e., a tag that satisfies a regexp like `^v[1-9]\d*.[1-9]\d*.[1-9]\d*$`. A git release candidate version (rc version) is defined as the release version but with a `rc` instead of a `v`.
* The `verify` subcommand can be used to check that the Cargo version is consistent with the workspace version. It only compares the base versions. If there is no git version, verify will always succeed, as the base version of the workspace version is taken from Cargo metadata. The verify command behaves differently based on the mode it is invoked with. For --dev mode it expects the Cargo version to be greater than the last git version, as the Cargo version should represent the next release version (pre-release versions are always considered smaller than the final release version). The rc and release modes expect the commit to be tagged with a rc or release version tag resepctively and the Cargo version to match that tag.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
