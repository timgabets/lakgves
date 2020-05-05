# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
