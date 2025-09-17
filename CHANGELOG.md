# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.1.1 (2025-09-17)

### Documentation

 - <csr-id-95060642ddd21ff8771d7d0bdc5004696e30b25b/> fix rustdoc blanks
 - <csr-id-2f21149aeee7132c6e4b01d37996663fcc5c1442/> rustdoc examples
 - <csr-id-9d5ef584fd6aadf515da8564533893b0b06c7c73/> inline doc tests
 - <csr-id-77a9b01aa3d87095e9c6a4e3ef2615211641d489/> fix codecov URL

### New Features

 - <csr-id-709ff767098b29a43404d39ef438fd83922c4547/> remove old tarpaulin action
 - <csr-id-a2f8047067005b3f9b8fc438c1abc381c8bd8741/> add From<primitive> for Binary
 - <csr-id-ae0179fbe19f7a74779c51dc61279d5a6ef4ea4b/> removed FromBinary and IntoBinary
 - <csr-id-657f377dcb33fb687e839bc76b0f3c4e6d2cf754/> add verbose to all CI tasks

### Bug Fixes

 - <csr-id-14fca9cf6c9f3b2d10f9875c0537dbd593d54708/> icicle chart link
 - <csr-id-93e9ba9c7ea30023c034e2718b2952a7071dec4f/> new codecov token
 - <csr-id-83ce41a5247a9dada6c028b38330a01558c894df/> codecov URL
 - <csr-id-aea89cc781b805cb4cd76ee9c77754e802c6ec47/> add cfg attributes to test modules
 - <csr-id-98eb032858672b7a3982a3d92e4b7ea51f3549fa/> switch from clippy-check action
 - <csr-id-521d940a1daff039bd65ad000d2923e2164b1839/> make clippy generate human-readable output

### Other

 - <csr-id-2ffeb7ee851a59acf9c3e53573d67c2c88ca80fd/> adding rustdoc comments and tests
 - <csr-id-4570c12ad163b1847833fba829faaf2deecff540/> add codecov config
 - <csr-id-ad62294dc1ffbfafd7740079ad70f008e19d022b/> set tarpaulin version to latest
 - <csr-id-85ccb48f850c0fb8cde681f082f36e16d68298f3/> upgrade tarpaulin
 - <csr-id-7fdda19039df09bb84408da0939f57b0fe73979a/> better install for libssl
 - <csr-id-a3b0304ccd4646ca7447fb8d1c6cb30adab7bbe0/> install libssl for tarpaulin

### Style

 - <csr-id-aa39290f06830ad28d35e12bbac426b7361c7b23/> fmt
 - <csr-id-51a9505e4e4c16fe0a333ad80f07548b4f7f1659/> final clippy
 - <csr-id-df7271c0cd273c490feaec2f52b68b209ecf1a64/> more clippy
 - <csr-id-c321b2c062640822139d685bbb62022b2b735464/> clippy
 - <csr-id-11eda38dec948b58644e2f5c31bcc21be0aa98f3/> rustfmt

### Test

 - <csr-id-857455c800a84d2d9c1f80d7a18a4df3964b8c94/> add primitives test

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 38 commits contributed to the release over the course of 4 calendar days.
 - 26 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Fix(doc) cfg_attr logic ([`77de413`](https://github.com/johnstonskj/rust-wrapbin/commit/77de4133d3aac1204d67a50c10db2c9448abbf84))
    - Fix rustdoc blanks ([`9506064`](https://github.com/johnstonskj/rust-wrapbin/commit/95060642ddd21ff8771d7d0bdc5004696e30b25b))
    - Rustdoc examples ([`2f21149`](https://github.com/johnstonskj/rust-wrapbin/commit/2f21149aeee7132c6e4b01d37996663fcc5c1442))
    - Adding rustdoc comments and tests ([`2ffeb7e`](https://github.com/johnstonskj/rust-wrapbin/commit/2ffeb7ee851a59acf9c3e53573d67c2c88ca80fd))
    - Icicle chart link ([`14fca9c`](https://github.com/johnstonskj/rust-wrapbin/commit/14fca9cf6c9f3b2d10f9875c0537dbd593d54708))
    - Add primitives test ([`857455c`](https://github.com/johnstonskj/rust-wrapbin/commit/857455c800a84d2d9c1f80d7a18a4df3964b8c94))
    - Add codecov config ([`4570c12`](https://github.com/johnstonskj/rust-wrapbin/commit/4570c12ad163b1847833fba829faaf2deecff540))
    - Inline doc tests ([`9d5ef58`](https://github.com/johnstonskj/rust-wrapbin/commit/9d5ef584fd6aadf515da8564533893b0b06c7c73))
    - New codecov token ([`93e9ba9`](https://github.com/johnstonskj/rust-wrapbin/commit/93e9ba9c7ea30023c034e2718b2952a7071dec4f))
    - Codecov URL ([`83ce41a`](https://github.com/johnstonskj/rust-wrapbin/commit/83ce41a5247a9dada6c028b38330a01558c894df))
    - Fix codecov URL ([`77a9b01`](https://github.com/johnstonskj/rust-wrapbin/commit/77a9b01aa3d87095e9c6a4e3ef2615211641d489))
    - Remove old tarpaulin action ([`709ff76`](https://github.com/johnstonskj/rust-wrapbin/commit/709ff767098b29a43404d39ef438fd83922c4547))
    - Set tarpaulin version to latest ([`ad62294`](https://github.com/johnstonskj/rust-wrapbin/commit/ad62294dc1ffbfafd7740079ad70f008e19d022b))
    - Upgrade tarpaulin ([`85ccb48`](https://github.com/johnstonskj/rust-wrapbin/commit/85ccb48f850c0fb8cde681f082f36e16d68298f3))
    - Add From<primitive> for Binary ([`a2f8047`](https://github.com/johnstonskj/rust-wrapbin/commit/a2f8047067005b3f9b8fc438c1abc381c8bd8741))
    - Better install for libssl ([`7fdda19`](https://github.com/johnstonskj/rust-wrapbin/commit/7fdda19039df09bb84408da0939f57b0fe73979a))
    - Install libssl for tarpaulin ([`a3b0304`](https://github.com/johnstonskj/rust-wrapbin/commit/a3b0304ccd4646ca7447fb8d1c6cb30adab7bbe0))
    - Removed FromBinary and IntoBinary ([`ae0179f`](https://github.com/johnstonskj/rust-wrapbin/commit/ae0179fbe19f7a74779c51dc61279d5a6ef4ea4b))
    - Add cfg attributes to test modules ([`aea89cc`](https://github.com/johnstonskj/rust-wrapbin/commit/aea89cc781b805cb4cd76ee9c77754e802c6ec47))
    - Fmt ([`aa39290`](https://github.com/johnstonskj/rust-wrapbin/commit/aa39290f06830ad28d35e12bbac426b7361c7b23))
    - Final clippy ([`51a9505`](https://github.com/johnstonskj/rust-wrapbin/commit/51a9505e4e4c16fe0a333ad80f07548b4f7f1659))
    - Switch from clippy-check action ([`98eb032`](https://github.com/johnstonskj/rust-wrapbin/commit/98eb032858672b7a3982a3d92e4b7ea51f3549fa))
    - Make clippy generate human-readable output ([`521d940`](https://github.com/johnstonskj/rust-wrapbin/commit/521d940a1daff039bd65ad000d2923e2164b1839))
    - Add verbose to all CI tasks ([`657f377`](https://github.com/johnstonskj/rust-wrapbin/commit/657f377dcb33fb687e839bc76b0f3c4e6d2cf754))
    - More clippy ([`df7271c`](https://github.com/johnstonskj/rust-wrapbin/commit/df7271c0cd273c490feaec2f52b68b209ecf1a64))
    - Clippy ([`c321b2c`](https://github.com/johnstonskj/rust-wrapbin/commit/c321b2c062640822139d685bbb62022b2b735464))
    - Rustfmt ([`11eda38`](https://github.com/johnstonskj/rust-wrapbin/commit/11eda38dec948b58644e2f5c31bcc21be0aa98f3))
    - Merge pull request #5 from johnstonskj/dependabot/github_actions/actions/download-artifact-5 ([`fcb3e72`](https://github.com/johnstonskj/rust-wrapbin/commit/fcb3e726ab7ada90888879b4c76320cd04201a78))
    - Merge pull request #4 from johnstonskj/dependabot/github_actions/actions/upload-artifact-4 ([`e17fc21`](https://github.com/johnstonskj/rust-wrapbin/commit/e17fc217ed3677ab07f5daab4c4f679021c30ac2))
    - Merge pull request #3 from johnstonskj/dependabot/github_actions/codecov/codecov-action-5.5.1 ([`e82315f`](https://github.com/johnstonskj/rust-wrapbin/commit/e82315f6e9cb5cd0f088f2313830ef41c77d9fd8))
    - Merge pull request #2 from johnstonskj/dependabot/github_actions/actions/checkout-5 ([`225a62c`](https://github.com/johnstonskj/rust-wrapbin/commit/225a62ca50c8228e262e2ef83af5c3b91b5aa4d0))
    - Merge pull request #1 from johnstonskj/dependabot/github_actions/Swatinem/rust-cache-2 ([`4ee5e72`](https://github.com/johnstonskj/rust-wrapbin/commit/4ee5e7293560b6f863a379a41973451965cd2155))
    - Bump actions/download-artifact from 4 to 5 ([`e3a140c`](https://github.com/johnstonskj/rust-wrapbin/commit/e3a140c5350e54e59c963b81ddfde7f5a3da1908))
    - Bump actions/upload-artifact from 1 to 4 ([`1ffc4f8`](https://github.com/johnstonskj/rust-wrapbin/commit/1ffc4f8424f515e646074fa05094150fd9d0d744))
    - Bump codecov/codecov-action from 1.0.2 to 5.5.1 ([`5d18d71`](https://github.com/johnstonskj/rust-wrapbin/commit/5d18d71044e1c1a5406fb04e0bb1778abdf3568d))
    - Bump actions/checkout from 2 to 5 ([`6e25191`](https://github.com/johnstonskj/rust-wrapbin/commit/6e25191acef547ebfcd3e823ff6045600c8a4899))
    - Bump Swatinem/rust-cache from 1 to 2 ([`63daa39`](https://github.com/johnstonskj/rust-wrapbin/commit/63daa39c1902858242fd5a39153412b659722dd7))
    - Initial commit ([`1e1f4ec`](https://github.com/johnstonskj/rust-wrapbin/commit/1e1f4ec3e90207da356e8db9183af0b620abdfd5))
</details>

