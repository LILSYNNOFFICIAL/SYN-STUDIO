# SYN Security Model

SYN documents are untrusted creative data.

## Core rule

**A SYN file is data, not authority.**

Opening a document must not grant it shell access, native code execution, unrestricted filesystem access, or unrestricted network access.

## Runtime boundaries

The prototype runtime accepts a constrained declarative action vocabulary. Unknown actions are rejected. Interaction targets are validated before rendering. Embedded image media is preferred over arbitrary remote media.

Future sensitive capabilities will be mediated by the host runtime rather than granted merely because a document requests them.

## Source and JavaScript

The Studio source editor exposes the SYN document model as JSON. General-purpose JavaScript is not executable through the document's current interaction model.

A future JavaScript extension system must run in an isolated, capability-gated sandbox with explicit host APIs.

## External links

HTTP and HTTPS navigation may be represented declaratively, but navigation is performed by the host browser. A SYN document cannot use link actions as a substitute for arbitrary network requests.

## Compatibility

Format evolution must preserve the distinction between document data and runtime authority. New capabilities should be additive, explicitly scoped, and safely degradable when a host does not grant them.
