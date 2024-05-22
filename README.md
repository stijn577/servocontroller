# RTIC testing

## This is just simple project to try and get an async runtime working for embedded rust, I might need it for future projects, so this is just the basic setup for that

### Reason

I had issues getting set up with a non-framework way of doing it because the delay struct couldn't be shared across different channels. I had a look at both Embassy and RTIC, but RTIC seemed more interesting to me because it only provides a runtime, not a HAL as well. But this is just my preference.

### [list of TODOs](TODO.md)


