# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.18] - 2020-07-08
### Added
- Using extfg-sigma version with fixed auth serno overflowing bug.

## [0.1.17] - 2020-07-07
### Added
- extfg-sigma message protocol support.

## [0.1.16] - 2020-06-15
### Added
- Managing application with systemd's ``systemctl``.

## [0.1.15] - 2020-06-10
### Added
- Building RPM package with ``cargo rpm build``.

## [0.1.14] - 2020-06-04
### Changed
- Using AtomicUsize instead of Mutex to switch between multiple connections.

## [0.1.13] - 2020-06-04
### Added
- External library for TSYS DHI XML message protocol serialization/deserialization.

## [0.1.12] - 2020-06-02
### Added
- External library for IBM Safer Payments® XML message protocol serialization/deserialization.

## [0.1.11] - 2020-06-01
### Added
- IBM Safer Payments® XML dummy message protocol support.

## [0.1.10] - 2020-05-11
### Added
- Returning 504 Gateway Timeout if there is 5 second timeout occurred while communicating with DHI host.

## [0.1.9] - 2020-05-05
### Added
- Parsing .toml configuration file.

## [0.1.8] - 2020-05-05
### Added
- Parsing command-line options (configuration file, help, version).

## [0.1.7] - 2020-05-04
### Changed
- Using actix-web framework.

## [0.1.6] - 2020-04-28
### Added
- DHI response deserialization.
- Converting DHI response to JSON and sending it back as HTTP payload.

## [0.1.5] - 2020-04-27
### Changed
- Talking to ISO host asynchronously.

## [0.1.4] - 2020-04-27
### Added
- Deserializing http requests and sending them **synchronously** (i.e. blocking) to the hardcoded ISO host.

## [0.1.3] - 2020-04-26
### Added
- Using tokio and hyper frameworks to handle http requests.

## [0.1.2] - 2020-04-16
### Added
- ``iso_fields`` object as a part of ``struct Request``.

## [0.1.1] - 2020-04-16
### Added
- Dummy serialization from json to xml

## [0.1.0] - 2020-04-15
### Added
- Initial commit
