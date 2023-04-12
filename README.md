# Link Router

This program helps manage which browser (and/or profile) should open a url.
This can be helpful if you wish to use a specific browser for certain apps or
to use the same IDP for multiple accounts keeping each session in a different
profile.

## How it works

The idea is to make `link-router` your default browser so that it has an
opportunity to decide with real browser gets the url request.

## Installation

```
sh setup.sh
```

Will build the application, and configure it to be your default browser.

## Configuration

If you want to configure different browsers or allow lists simply make the
change in the source and recompile / install.
