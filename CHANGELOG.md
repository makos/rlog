# 1.0.0
* **Public API**
    * Please note that for the most part everything should work as before, but because I'm removing public API fields I incremented the major version.
    * Removed `time_fmt` and `date_fmt` in favor of using ISO8061 timestamps straight away via the `format` field.
    * Added setter and getter methods for `path` and `format` methods.
* **Tests and testenv**
    * All tests refit for new API.
    * Removed `instantiate()` function from testenv.

# 0.2.0
* **Public API**
    * Added two new fields to the `Logger` struct:
        * `time_fmt` and `date_fmt` to allow customization of ISO8061 timestamps.
    * All changes backwards-compatible.

# 0.1.1 - 0.1.4
* No public API changes.

# 0.1.0
* Initial release.