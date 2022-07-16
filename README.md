# cargo-gitv

Helps to manage your versions.

* Computes a semantic-versioning compatible workspace version following a set of rules.
* Defaults to computing pre-release versions. 
* Default pre-release identifier is `dev`.
* Default base version is the latest git version tag. A git version tag is a `v` followed by a normal semantic version, i.e., a tag that satisfies a regexp like `^v[1-9]\d*.[1-9]\d*.[1-9]\d*$`.
* If there is no git version tag, the base version is taken from the Cargo metadata.
* The `verify` subcommand can be used to check that the Cargo version is consistent with the workspace version. It only compares the base versions. If there is no git version, verify will always succeed, as the base version of the workspace version is taken from Cargo metadata.