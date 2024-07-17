# Improvements

The DNS client part of this server works, but does not handle a lot of other cases besides the expected A and CNAME records.
There are many improvements that can be made.

There are many improvements that can be made, and for that, they have been sorted into two subgroups: `quick fix`, for changes that are just scaling up the code that is currently existing, and easily understood. Then, there is `slow fix`, which is for code that is estimated to take longer and more brainpower to code in.

## Quick Fixes

__Add more options for RCODE__:
In `src/dns_components/dns_header.rs` there is a method called `get_error()` which is called whenever an RCODE is found in the header that is non-zero. This currently only directly supports an `RCODE` of `3`, or Not Found. Additional support for different RCODES could be sdded is somewhat easily, with more branches to the match statement.
