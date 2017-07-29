# systime_converter
Library providing one single function: convert a standard-libary SystemTime
into the much more user-friendly DateTime format used by the Chrono crate.

For some reason chrono doesn't provide this conversion, 
even though it's handy for dealing with things like 
file metadata, etc. that are typed as SystemTime

Though the implementation is trivially simple, this is provided
so that its users can perform the conversion
without first having to figure out how to work with SystemTime.
